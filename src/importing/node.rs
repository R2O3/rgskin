#![allow(unused)]
#![cfg(feature = "node")]

use std::collections::HashMap;
use std::str::FromStr;

#[cfg(all(target_arch = "wasm32", feature = "node"))]
use crate::io::node;
use crate::common::traits::SkinConfig;
use crate::io::texture::TextureStore;
use crate::sample::SampleStore;
use crate::utils::io::remove_extension;
use crate::importing::common::{extension_matches, build_texture_store_from_files, SeenFiles};
use crate::io::StringPattern;
use crate::error::ImportError;

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
        let entries = match node::read_dir(dir_path) {
            Ok(entries) => entries,
            Err(_) => return Ok(()),
        };

        for entry in entries {
            let entry_path = node::join_path(dir_path, &entry);

            if node::is_directory(&entry_path).map_err(|source| ImportError::Walk {
                path: entry_path.clone(),
                source: std::io::Error::new(std::io::ErrorKind::Other, source.to_string()),
            })? {
                recurse(&entry_path, base_path, extensions, loader, seen_files)?;
            } else if let Some(ext_pos) = entry.rfind('.') {
                let ext = &entry[ext_pos + 1..];
                if extension_matches(ext, extensions) {
                    let relative_path = entry_path
                        .strip_prefix(base_path)
                        .unwrap_or(&entry)
                        .trim_start_matches('/')
                        .trim_start_matches('\\');

                    let path_without_ext = remove_extension(relative_path);
                    if seen_files.try_insert(&path_without_ext) {
                        let bytes = node::read_file_bytes(&entry_path).map_err(|source| {
                            ImportError::Walk {
                                path: entry_path.clone(),
                                source: std::io::Error::new(std::io::ErrorKind::Other, source.to_string()),
                            }
                        })?;
                        loader(path_without_ext.to_string(), &bytes)?;
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
    let mut files: HashMap<String, Vec<u8>> = HashMap::new();
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

pub fn import_all_samples_from_dir(
    path: &str,
) -> Result<SampleStore, ImportError> {
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
    match node::read_file_string(path) {
        Ok(content) => Ok(Some(content)),
        Err(e) => {
            Ok(None)
        }
    }
}

macro_rules! impl_skin_importer {
    ($fn_name:ident, $config_file:expr, $config_type:ty, $skin_type:ty, filtered) => {
        pub fn $fn_name(
            path: &str,
            import_all: bool,
        ) -> Result<$skin_type, ImportError> {
            let config_path = node::join_path(path, $config_file);
            let config = match read_str_from_path(&config_path)? {
                Some(content) => <$config_type>::from_str(&content).map_err(|source| {
                    ImportError::ParseConfig {
                        path: config_path.clone(),
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
            let config_path = node::join_path(path, $config_file);
            let config = match read_str_from_path(&config_path)? {
                Some(content) => <$config_type>::from_str(&content).map_err(|source| {
                    ImportError::ParseConfig {
                        path: config_path.clone(),
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

impl_skin_importer!(import_osu_mania_skin_from_dir, "skin.ini", crate::osu::OsuSkinIni, crate::OsuSkin, filtered);
impl_skin_importer!(import_fluxis_skin_from_dir, "skin.json", crate::fluxis::SkinJson, crate::fluxis::FluXisSkin, all);
impl_skin_importer!(import_quaver_skin_from_dir, "skin.ini", crate::quaver::QuaSkinIni, crate::quaver::QuaSkin, filtered);