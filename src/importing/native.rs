#![allow(unused)]
#![cfg(not(target_arch = "wasm32"))]

use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::collections::HashMap;
use crate::common::traits::SkinConfig;
use crate::error::ImportError;
use crate::fluxis::{self, FluXisSkin};
use crate::io::StringPattern;
use crate::{osu, quaver, Store};
use crate::sample::SampleStore;
use crate::utils::io::{get_filename, get_parent, get_stem, join_paths_unix, remove_extension};
use crate::OsuSkin;
use crate::io::texture::{TextureStore, Texture};
use crate::importing::common::{
    SeenFiles, build_texture_store_from_files, extension_matches,
};

pub fn import_binaries_from_dir<F>(
    path: &str,
    patterns: &[StringPattern],
    mut loader: F,
) -> Result<(), ImportError>
where
    F: FnMut(String, &[u8]) -> Result<(), ImportError>,
{
    import_all_binaries_from_dir(path, &["png", "jpg", "jpeg", "wav", "ogg"], |file_path, bytes| {
        if patterns.iter().any(|p| p.matches_path(&file_path)) {
            loader(file_path, bytes)?;
        }
        Ok(())
    })
}

pub fn import_all_binaries_from_dir<F>(
    path: &str,
    extensions: &[&str],
    mut loader: F,
) -> Result<(), ImportError>
where
    F: FnMut(String, &[u8]) -> Result<(), ImportError>,
{
    if !std::path::Path::new(path).exists() {
        return Ok(());
    }

    fn recurse<F>(
        dir_path: &str,
        base_path: &str,
        extensions: &[&str],
        loader: &mut F,
        seen_files: &mut SeenFiles,
    ) -> Result<(), ImportError>
    where
        F: FnMut(String, &[u8]) -> Result<(), ImportError>,
    {
        let entries = fs::read_dir(dir_path).map_err(|source| ImportError::Walk {
            path: dir_path.to_string(),
            source,
        })?;

        for entry in entries {
            let entry = entry.map_err(|source| ImportError::Walk {
                path: dir_path.to_string(),
                source,
            })?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                if let Some(path_str) = entry_path.to_str() {
                    recurse(path_str, base_path, extensions, loader, seen_files)?;
                }
            } else if let Some(ext) = entry_path.extension() {
                let ext_str = ext.to_string_lossy();

                if extension_matches(&ext_str, extensions) {
                    if let (Some(full_path_str), Some(file_name)) = (
                        entry_path.to_str(),
                        entry_path.file_name().and_then(|n| n.to_str()),
                    ) {
                        let relative_path = full_path_str
                            .strip_prefix(base_path)
                            .unwrap_or(file_name)
                            .trim_start_matches('/')
                            .trim_start_matches('\\');

                        let path_without_ext = remove_extension(relative_path);

                        if seen_files.try_insert(&path_without_ext) {
                            let bytes = fs::read(full_path_str).map_err(|source| ImportError::Walk {
                                path: full_path_str.to_string(),
                                source,
                            })?;
                            loader(path_without_ext.to_string(), &bytes)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    let mut seen_files = SeenFiles::new();
    recurse(path, path, extensions, &mut loader, &mut seen_files)?;
    Ok(())
}

pub fn import_textures_from_dir(
    path: &str,
    patterns: &[StringPattern],
) -> Result<TextureStore, ImportError> {
    let mut files = HashMap::new();
    import_binaries_from_dir(path, patterns, |path, bytes| {
        files.insert(path, bytes.to_vec());
        Ok(())
    })?;
    build_texture_store_from_files(&files, None).map_err(ImportError::from)
}

pub fn import_all_textures_from_dir(
    path: &str,
    load_only: Option<&[StringPattern]>,
) -> Result<TextureStore, ImportError> {
    let mut files: HashMap<String, Vec<u8>> = HashMap::new();
    import_all_binaries_from_dir(path, &["png", "jpg", "jpeg"], |path, bytes| {
        files.insert(path, bytes.to_vec());
        Ok(())
    })?;
    build_texture_store_from_files(&files, load_only).map_err(ImportError::from)
}

pub fn import_samples_from_dir(
    path: &str,
    relative_sample_paths: &[StringPattern],
) -> Result<SampleStore, ImportError> {
    let mut sample_store = SampleStore::new();
    import_binaries_from_dir(path, relative_sample_paths, |sample_path, bytes| {
        sample_store
            .load_from_bytes(sample_path.clone(), bytes)
            .map_err(|source| ImportError::Sample {
                path: sample_path,
                source: source.into(),
            })?;
        Ok(())
    })?;
    Ok(sample_store)
}

#[allow(unused)]
pub fn import_all_samples_from_dir(path: &str) -> Result<SampleStore, ImportError> {
    let mut sample_store = SampleStore::new();
    import_all_binaries_from_dir(path, &["wav", "ogg"], |sample_path, bytes| {
        sample_store
            .load_from_bytes(sample_path.clone(), bytes)
            .map_err(|source| ImportError::Sample {
                path: sample_path,
                source: source.into(),
            })?;
        Ok(())
    })?;
    Ok(sample_store)
}

pub fn read_str_from_path(path: &str) -> Result<Option<String>, ImportError> {
    if !Path::new(path).exists() {
        return Ok(None);
    }
    fs::read_to_string(path)
        .map(Some)
        .map_err(|source| ImportError::ReadConfig {
            path: path.to_string(),
            source,
        })
}

macro_rules! impl_skin_importer {
    ($fn_name:ident, $config_file:expr, $config_type:ty, $skin_type:ty, filtered) => {
        pub fn $fn_name(
            path: &str,
            import_all: bool,
        ) -> Result<$skin_type, ImportError> {
            let config_path = Path::new(path).join($config_file);
            let config_path_str = config_path.to_str().unwrap_or($config_file).to_string();

            let config = match read_str_from_path(&config_path_str)? {
                Some(content) => <$config_type>::from_str(&content).map_err(|source| {
                    ImportError::ParseConfig {
                        path: config_path_str.clone(),
                        source: source.into(),
                    }
                })?,
                None => <$config_type>::default(),
            };

            let texture_paths = config.get_required_texture_paths();
            let sample_paths  = config.get_required_sample_paths();
            let textures = if import_all {
                import_all_textures_from_dir(path, Some(&texture_paths))?
            } else {
                import_textures_from_dir(path, &texture_paths)?
            };
            let samples = if import_all {
                import_all_samples_from_dir(path)?
            } else {
                import_samples_from_dir(path, &sample_paths)?
            };
            Ok(<$skin_type>::new(config, Some(textures), Some(samples)))
        }
    };
    ($fn_name:ident, $config_file:expr, $config_type:ty, $skin_type:ty, all) => {
        pub fn $fn_name(
            path: &str,
            import_all: bool,
        ) -> Result<$skin_type, ImportError> {
            let config_path = Path::new(path).join($config_file);
            let config_path_str = config_path.to_str().unwrap_or($config_file).to_string();

            let config = match read_str_from_path(&config_path_str)? {
                Some(content) => <$config_type>::from_str(&content).map_err(|source| {
                    ImportError::ParseConfig {
                        path: config_path_str.clone(),
                        source: source.into(),
                    }
                })?,
                None => <$config_type>::default(),
            };

            let sample_paths = config.get_required_sample_paths();
            let textures = import_all_textures_from_dir(path, None)?;
            let samples = if import_all {
                import_all_samples_from_dir(path)?
            } else {
                import_samples_from_dir(path, &sample_paths)?
            };
            Ok(<$skin_type>::new(config, Some(textures), Some(samples)))
        }
    };
}

impl_skin_importer!(import_osu_mania_skin_from_dir, "skin.ini", osu::OsuSkinIni, OsuSkin, filtered);
impl_skin_importer!(import_fluxis_skin_from_dir, "skin.json", fluxis::SkinJson, FluXisSkin, all);
impl_skin_importer!(import_quaver_skin_from_dir, "skin.ini", quaver::QuaSkinIni, quaver::QuaSkin, filtered);