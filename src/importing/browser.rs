#![cfg(all(target_arch = "wasm32", feature = "browser"))]

use std::str::FromStr;
use std::collections::HashMap;
use wasm_bindgen::JsError;
use crate::common::traits::SkinConfig;
use crate::fluxis::{self, FluXisSkin};
use crate::{osu, quaver};
use crate::sample::SampleStore;
use crate::utils::io::{get_filename, remove_extension, normalize};
use crate::utils::string::string_iter_as_str;
use crate::OsuSkin;
use crate::io::texture::TextureStore;
use crate::importing::common::{extension_matches, SeenFiles, build_texture_store_from_files};
use crate::io::StringPattern;
use crate::error::ImportError;

pub fn import_binaries_from_files<F>(
    files: &HashMap<String, Vec<u8>>,
    patterns: &[&str],
    extensions: &[&str],
    mut loader: F,
) -> Result<(), ImportError>
where
    F: FnMut(String, &[u8]) -> Result<(), ImportError>,
{
    let mut seen = SeenFiles::new();
    let patterns: Vec<StringPattern> = patterns
        .iter()
        .map(|p| StringPattern::from(p.to_string()))
        .collect();
    for (file_path, bytes) in files {
        let normalized = normalize(file_path);
        if let Some(ext_pos) = normalized.rfind('.') {
            let ext = &normalized[ext_pos + 1..];
            if extension_matches(ext, extensions) {
                let path_without_ext = remove_extension(&normalized);
                if seen.try_insert(&path_without_ext)
                    && patterns.iter().any(|p| p.matches_path(&path_without_ext))
                {
                    loader(path_without_ext.to_string(), bytes)?;
                }
            }
        }
    }
    Ok(())
}

pub fn import_all_binaries_from_files<F>(
    files: &HashMap<String, Vec<u8>>,
    extensions: &[&str],
    mut loader: F,
) -> Result<(), ImportError>
where
    F: FnMut(String, &[u8]) -> Result<(), ImportError>,
{
    let mut seen_files = SeenFiles::new();
    for (file_path, bytes) in files {
        let normalized_path = normalize(file_path);
        if let Some(ext_pos) = normalized_path.rfind('.') {
            let ext = &normalized_path[ext_pos + 1..];
            if extension_matches(ext, extensions) {
                let path_without_ext = remove_extension(&normalized_path);
                if seen_files.try_insert(&path_without_ext) {
                    loader(path_without_ext.to_string(), bytes)?;
                }
            }
        }
    }
    Ok(())
}

pub fn import_textures_from_files(
    files: &HashMap<String, Vec<u8>>,
    patterns: &[&str],
) -> Result<TextureStore, ImportError> {
    let mut filtered = HashMap::new();
    import_binaries_from_files(files, patterns, &["png", "jpg", "jpeg"], |path, bytes| {
        filtered.insert(path, bytes.to_vec());
        Ok(())
    })?;
    build_texture_store_from_files(&filtered, None).map_err(ImportError::from)
}

pub fn import_all_textures_from_files(
    files: &HashMap<String, Vec<u8>>,
) -> Result<TextureStore, ImportError> {
    let mut filtered = HashMap::new();
    import_all_binaries_from_files(files, &["png", "jpg", "jpeg"], |path, bytes| {
        filtered.insert(path, bytes.to_vec());
        Ok(())
    })?;
    build_texture_store_from_files(&filtered, None).map_err(ImportError::from)
}

pub fn import_samples_from_files(
    files: &HashMap<String, Vec<u8>>,
    relative_sample_paths: &[&str],
) -> Result<SampleStore, ImportError> {
    let mut sample_store = SampleStore::new();
    import_binaries_from_files(files, relative_sample_paths, &["wav", "ogg"], |path, bytes| {
        sample_store
            .load_from_bytes(path.clone(), bytes)
            .map_err(|source| ImportError::Sample {
                path,
                source: source.into(),
            })?;
        Ok(())
    })?;
    Ok(sample_store)
}

#[allow(unused)]
pub fn import_all_samples_from_files(
    files: &HashMap<String, Vec<u8>>,
) -> Result<SampleStore, ImportError> {
    let mut sample_store = SampleStore::new();
    import_all_binaries_from_files(files, &["wav", "ogg"], |path, bytes| {
        sample_store
            .load_from_bytes(path.clone(), bytes)
            .map_err(|source| ImportError::Sample {
                path,
                source: source.into(),
            })?;
        Ok(())
    })?;
    Ok(sample_store)
}

macro_rules! impl_skin_importer {
    ($fn_name:ident, $config_file:expr, $config_type:ty, $skin_type:ty, filtered) => {
        pub fn $fn_name(files: &HashMap<String, Vec<u8>>) -> Result<$skin_type, ImportError> {
            let found = files.iter().find(|(path, _)| {
                get_filename(&normalize(path)).eq_ignore_ascii_case($config_file)
            });

            let config = match found {
                Some((path, bytes)) => {
                    let content = String::from_utf8(bytes.clone()).map_err(|source| {
                        ImportError::ParseConfig {
                            path: path.clone(),
                            source: Box::new(source),
                        }
                    })?;
                    <$config_type>::from_str(&content).map_err(|source| {
                        ImportError::ParseConfig {
                            path: path.clone(),
                            source: source.into(),
                        }
                    })?
                }
                None => <$config_type>::default(),
            };

            let texture_paths = config.get_required_texture_paths();
            let sample_paths  = config.get_required_sample_paths();
            let texture_path_strings: Vec<String> = texture_paths.iter().map(|p| p.to_string()).collect();
            let sample_path_strings: Vec<String> = sample_paths.iter().map(|p| p.to_string()).collect();
            let texture_path_refs: Vec<&str> = string_iter_as_str(texture_path_strings.iter());
            let sample_path_refs:  Vec<&str> = string_iter_as_str(sample_path_strings.iter());
            let textures = import_textures_from_files(files, &texture_path_refs)?;
            let samples  = import_samples_from_files(files, &sample_path_refs)?;
            Ok(<$skin_type>::new(config, Some(textures), Some(samples)))
        }
    };
    ($fn_name:ident, $config_file:expr, $config_type:ty, $skin_type:ty, all) => {
        pub fn $fn_name(files: &HashMap<String, Vec<u8>>) -> Result<$skin_type, ImportError> {
            let found = files.iter().find(|(path, _)| {
                get_filename(&normalize(path)).eq_ignore_ascii_case($config_file)
            });

            let config = match found {
                Some((path, bytes)) => {
                    let content = String::from_utf8(bytes.clone()).map_err(|source| {
                        ImportError::ParseConfig {
                            path: path.clone(),
                            source: Box::new(source),
                        }
                    })?;
                    <$config_type>::from_str(&content).map_err(|source| {
                        ImportError::ParseConfig {
                            path: path.clone(),
                            source: source.into(),
                        }
                    })?
                }
                None => <$config_type>::default(),
            };

            let sample_paths = config.get_required_sample_paths();
            let sample_path_strings: Vec<String> = sample_paths.iter().map(|p| p.to_string()).collect();
            let sample_path_refs: Vec<&str> = string_iter_as_str(sample_path_strings.iter());
            let textures = import_all_textures_from_files(files)?;
            let samples  = import_samples_from_files(files, &sample_path_refs)?;
            Ok(<$skin_type>::new(config, Some(textures), Some(samples)))
        }
    };
}

impl_skin_importer!(import_osu_mania_skin_from_files, "skin.ini", osu::OsuSkinIni, OsuSkin, filtered);
impl_skin_importer!(import_fluxis_skin_from_files, "skin.json", fluxis::SkinJson, FluXisSkin, all);
impl_skin_importer!(import_quaver_skin_from_files, "skin.ini", quaver::QuaSkinIni, quaver::QuaSkin, filtered);

#[allow(unused)]
pub fn import_osu_mania_skin_from_files_js(files: &HashMap<String, Vec<u8>>) -> Result<OsuSkin, JsError> {
    import_osu_mania_skin_from_files(files).map_err(|e| JsError::new(&e.to_string()))
}

#[allow(unused)]
pub fn import_fluxis_skin_from_files_js(files: &HashMap<String, Vec<u8>>) -> Result<FluXisSkin, JsError> {
    import_fluxis_skin_from_files(files).map_err(|e| JsError::new(&e.to_string()))
}

#[allow(unused)]
pub fn import_quaver_skin_from_files_js(files: &HashMap<String, Vec<u8>>) -> Result<quaver::QuaSkin, JsError> {
    import_quaver_skin_from_files(files).map_err(|e| JsError::new(&e.to_string()))
}
