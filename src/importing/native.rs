#![allow(unused)]

#![cfg(not(target_arch = "wasm32"))]

use std::fs;
use std::path::Path;
use std::str::FromStr;
use crate::common::traits::SkinConfig;
use crate::fluxis::{self, FluXisSkin};
use crate::osu;
use crate::sample::SampleStore;
use crate::utils::io::{get_filename, get_parent, get_stem, join_paths_unix, remove_extension};
use crate::utils::string::string_iter_as_str;
use crate::OsuSkin;
use crate::io::texture::TextureStore;

pub fn import_binaries_from_dir<F>(
    path: &str,
    relative_paths: &[&str],
    mut loader: F,
) -> Result<(), Box<dyn std::error::Error>> 
where
    F: FnMut(String, &[u8]) -> Result<(), Box<dyn std::error::Error>>,
{
    for &relative_path in relative_paths {
        let dir_path = join_paths_unix(path, &get_parent(relative_path));
        let filename = get_filename(relative_path);
        
        if let Ok(entries) = fs::read_dir(&dir_path) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    let file_stem = get_stem(&file_name);
                    if file_stem == filename {
                        let full_path = join_paths_unix(&dir_path, &file_name);
                        
                        if let Ok(bytes) = fs::read(&full_path) {
                            loader(relative_path.to_string(), &bytes)?;
                            break;
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

pub fn import_all_binaries_from_dir<F>(
    path: &str,
    extensions: &[&str],
    mut loader: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(String, &[u8]) -> Result<(), Box<dyn std::error::Error>>,
{
    if !std::path::Path::new(path).exists() {
        return Ok(());
    }

    fn recurse<F>(
        dir_path: &str,
        base_path: &str,
        extensions: &[&str],
        loader: &mut F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(String, &[u8]) -> Result<(), Box<dyn std::error::Error>>,
    {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                
                if entry_path.is_dir() {
                    if let Some(path_str) = entry_path.to_str() {
                        recurse(path_str, base_path, extensions, loader)?;
                    }
                } else if let Some(ext) = entry_path.extension() {
                    let ext_lower = ext.to_string_lossy().to_lowercase();
                    if extensions.contains(&ext_lower.as_str()) {
                        if let (Some(full_path_str), Some(file_name)) = 
                            (entry_path.to_str(), entry_path.file_name().and_then(|n| n.to_str())) {
                            
                            let relative_path = full_path_str
                                .strip_prefix(base_path)
                                .unwrap_or(file_name)
                                .trim_start_matches('/').trim_start_matches('\\');
                            
                            if let Ok(bytes) = fs::read(&full_path_str) {
                                loader(remove_extension(relative_path), &bytes)?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    recurse(path, path, extensions, &mut loader)?;
    Ok(())
}

pub fn import_textures_from_dir(path: &str, relative_texture_paths: &[&str]) -> Result<TextureStore, Box<dyn std::error::Error>> {
    let mut texture_store = TextureStore::new();
    
    import_binaries_from_dir(path, relative_texture_paths, |path, bytes| {
        texture_store.load_from_bytes(path, bytes)?;
        Ok(())
    })?;
    
    Ok(texture_store)
}

pub fn import_all_textures_from_dir(path: &str) -> Result<TextureStore, Box<dyn std::error::Error>> {
    let mut texture_store = TextureStore::new();
    
    import_all_binaries_from_dir(path, &["png", "jpg", "jpeg"], |path, bytes| {
        texture_store.load_from_bytes(path, bytes)?;
        Ok(())
    })?;
    
    Ok(texture_store)
}

pub fn import_samples_from_dir(path: &str, relative_sample_paths: &[&str]) -> Result<SampleStore, Box<dyn std::error::Error>> {
    let mut sample_store = SampleStore::new();
    
    import_binaries_from_dir(path, relative_sample_paths, |path, bytes| {
        sample_store.load_from_bytes(path, bytes)?;
        Ok(())
    })?;
    
    Ok(sample_store)
}

#[allow(unused)]
pub fn import_all_samples_from_dir(path: &str) -> Result<SampleStore, Box<dyn std::error::Error>> {
    let mut sample_store = SampleStore::new();
    
    import_all_binaries_from_dir(path, &["wav", "ogg"], |path, bytes| {
        sample_store.load_from_bytes(path, bytes)?;
        Ok(())
    })?;
    
    Ok(sample_store)
}

pub fn read_str_from_path(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

pub fn import_osu_mania_skin_from_dir(path: &str) -> Result<OsuSkin, Box<dyn std::error::Error>> {
    let ini_path = Path::new(path).join("skin.ini");
    let ini_content = read_str_from_path(ini_path.to_str().unwrap());

    let skin_ini = osu::OsuSkinIni::from_str(&ini_content)?;
    
    let texture_paths = skin_ini.get_required_texture_paths();
    let sample_paths = skin_ini.get_required_sample_paths();
    
    let texture_path_refs: Vec<&str> = string_iter_as_str(texture_paths.iter());
    let sample_path_refs: Vec<&str> = string_iter_as_str(sample_paths.iter());
    
    let textures = import_textures_from_dir(path, &texture_path_refs)?;
    let samples = import_samples_from_dir(path, &sample_path_refs)?;

    Ok(OsuSkin::new(skin_ini, Some(textures), Some(samples)))
}

pub fn import_fluxis_skin_from_dir(path: &str) -> Result<FluXisSkin, Box<dyn std::error::Error>> {
    let json_path = Path::new(path).join("skin.json");
    let json_content = read_str_from_path(json_path.to_str().unwrap());
    
    let skin_json = fluxis::SkinJson::from_str(&json_content)?;

    let sample_paths = skin_json.get_required_sample_paths();
    let sample_path_refs: Vec<&str> = string_iter_as_str(sample_paths.iter());
    
    let textures = import_all_textures_from_dir(path)?;
    let samples = import_samples_from_dir(path, &sample_path_refs)?;

    Ok(FluXisSkin::new(skin_json, Some(textures), Some(samples)))
}
