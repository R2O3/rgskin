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
 
pub fn import_binaries_from_dir<F>(
    path: &str,
    patterns: &[StringPattern],
    mut loader: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(String, &[u8]) -> Result<(), Box<dyn std::error::Error>>,
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
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(String, &[u8]) -> Result<(), Box<dyn std::error::Error>>,
{
    fn recurse<F>(
        dir_path: &str,
        base_path: &str,
        extensions: &[&str],
        loader: &mut F,
        seen_files: &mut SeenFiles,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(String, &[u8]) -> Result<(), Box<dyn std::error::Error>>,
    {
        if let Ok(entries) = node::read_dir(dir_path) {
            for entry in entries {
                let entry_path = node::join_path(dir_path, &entry);
 
                if node::is_directory(&entry_path)? {
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
                            if let Ok(bytes) = node::read_file_bytes(&entry_path) {
                                loader(path_without_ext.to_string(), &bytes)?;
                            }
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
) -> Result<TextureStore, Box<dyn std::error::Error>> {
    let mut files: HashMap<String, Vec<u8>> = HashMap::new();
    import_binaries_from_dir(path, patterns, |path, bytes| {
        files.insert(path, bytes.to_vec());
        Ok(())
    })?;
    build_texture_store_from_files(&files, None)
}
 
pub fn import_all_textures_from_dir(
    path: &str,
    load_only: Option<&[StringPattern]>,
) -> Result<TextureStore, Box<dyn std::error::Error>> {
    let mut files: HashMap<String, Vec<u8>> = HashMap::new();
    import_all_binaries_from_dir(path, &["png", "jpg", "jpeg"], |path, bytes| {
        files.insert(path, bytes.to_vec());
        Ok(())
    })?;
    build_texture_store_from_files(&files, load_only)
}
 
pub fn import_samples_from_dir(
    path: &str,
    relative_sample_paths: &[StringPattern],
) -> Result<SampleStore, Box<dyn std::error::Error>> {
    let mut sample_store = SampleStore::new();
    import_binaries_from_dir(path, relative_sample_paths, |path, bytes| {
        sample_store.load_from_bytes(path, bytes)?;
        Ok(())
    })?;
    Ok(sample_store)
}
 
pub fn import_all_samples_from_dir(
    path: &str,
) -> Result<SampleStore, Box<dyn std::error::Error>> {
    let mut sample_store = SampleStore::new();
    import_all_binaries_from_dir(path, &["wav", "ogg"], |path, bytes| {
        sample_store.load_from_bytes(path, bytes)?;
        Ok(())
    })?;
    Ok(sample_store)
}
 
pub fn read_str_from_path(path: &str) -> String {
    node::read_file_string(path).unwrap_or_default()
}

macro_rules! impl_skin_importer {
    ($fn_name:ident, $config_file:expr, $config_type:ty, $skin_type:ty, filtered) => {
        pub fn $fn_name(
            path: &str,
            import_all: bool,
        ) -> Result<$skin_type, Box<dyn std::error::Error>> {
            let config_content = read_str_from_path(&node::join_path(path, $config_file));
            let config = <$config_type>::from_str(&config_content)?;
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
        ) -> Result<$skin_type, Box<dyn std::error::Error>> {
            let config_content = read_str_from_path(&node::join_path(path, $config_file));
            let config = <$config_type>::from_str(&config_content)?;
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