#![cfg(all(target_arch = "wasm32", feature = "browser"))]

use std::str::FromStr;
use std::collections::HashMap;
use wasm_bindgen::JsError;
use crate::common::traits::SkinConfig;
use crate::fluxis::{self, FluXisSkin};
use crate::osu;
use crate::sample::SampleStore;
use crate::utils::io::{get_filename, get_parent, get_stem, remove_extension, normalize};
use crate::utils::string::string_iter_as_str;
use crate::OsuSkin;
use crate::io::texture::TextureStore;
use crate::importing::common::{file_matches_target, path_matches_target, extension_matches, SeenFiles};

fn path_matches(file_path: &str, target_relative_path: &str) -> bool {
    let file_name = get_filename(file_path);
    let file_stem = get_stem(&file_name);
    
    let target_parent = get_parent(target_relative_path);
    let target_filename = get_filename(target_relative_path);
    
    if !file_matches_target(&file_stem, &target_filename) {
        return false;
    }
    
    let file_parent = get_parent(file_path);
    
    path_matches_target(file_path, &file_parent, target_relative_path, &target_parent)
}

pub fn import_binaries_from_files<F>(
    files: &HashMap<String, Vec<u8>>,
    relative_paths: &[&str],
    mut loader: F,
) -> Result<(), JsError>
where
    F: FnMut(String, &[u8]) -> Result<(), JsError>,
{
    for &relative_path in relative_paths {
        let mut matches: Vec<(&String, &Vec<u8>)> = Vec::new();
        
        for (file_path, bytes) in files {
            let normalized_path = normalize(file_path);
            if path_matches(&normalized_path, relative_path) {
                matches.push((file_path, bytes));
            }
        }
        
        let target_filename = get_filename(relative_path);
        let chosen_file = matches.iter()
            .find(|(file_path, _)| {
                let normalized = normalize(file_path);
                let file_stem = get_stem(&get_filename(&normalized));
                file_stem == target_filename
            })
            .or_else(|| matches.first());
        
        if let Some((_, bytes)) = chosen_file {
            loader(relative_path.to_string(), bytes)?;
        }
    }
    
    Ok(())
}

pub fn import_all_binaries_from_files<F>(
    files: &HashMap<String, Vec<u8>>,
    extensions: &[&str],
    mut loader: F,
) -> Result<(), JsError>
where
    F: FnMut(String, &[u8]) -> Result<(), JsError>,
{
    let mut seen_files = SeenFiles::new();
    
    for (file_path, bytes) in files {
        let normalized_path = normalize(file_path);
        
        if let Some(ext_pos) = normalized_path.rfind('.') {
            let ext = &normalized_path[ext_pos + 1..];
            
            if extension_matches(ext, extensions) {
                let path_without_ext = remove_extension(&normalized_path);
                
                if seen_files.try_insert(&path_without_ext) {
                    loader(path_without_ext, bytes)?;
                }
            }
        }
    }
    
    Ok(())
}

pub fn import_textures_from_files(
    files: &HashMap<String, Vec<u8>>,
    relative_texture_paths: &[&str],
) -> Result<TextureStore, JsError> {
    let mut texture_store = TextureStore::new();
    
    import_binaries_from_files(files, relative_texture_paths, |path, bytes| {
        texture_store.load_from_bytes(path, bytes)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(())
    })?;
    
    Ok(texture_store)
}

pub fn import_all_textures_from_files(
    files: &HashMap<String, Vec<u8>>,
) -> Result<TextureStore, JsError> {
    let mut texture_store = TextureStore::new();
    
    import_all_binaries_from_files(files, &["png", "jpg", "jpeg"], |path, bytes| {
        texture_store.load_from_bytes(path, bytes)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(())
    })?;
    
    Ok(texture_store)
}

pub fn import_samples_from_files(
    files: &HashMap<String, Vec<u8>>,
    relative_sample_paths: &[&str],
) -> Result<SampleStore, JsError> {
    let mut sample_store = SampleStore::new();
    
    import_binaries_from_files(files, relative_sample_paths, |path, bytes| {
        sample_store.load_from_bytes(path, bytes)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(())
    })?;
    
    Ok(sample_store)
}

#[allow(unused)]
pub fn import_all_samples_from_files(
    files: &HashMap<String, Vec<u8>>,
) -> Result<SampleStore, JsError> {
    let mut sample_store = SampleStore::new();
    
    import_all_binaries_from_files(files, &["wav", "ogg"], |path, bytes| {
        sample_store.load_from_bytes(path, bytes)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(())
    })?;
    
    Ok(sample_store)
}

pub fn import_osu_mania_skin_from_files(
    files: &HashMap<String, Vec<u8>>,
) -> Result<OsuSkin, JsError> {
    let ini_content = files
        .iter()
        .find(|(path, _)| {
            let normalized = normalize(path);
            get_filename(&normalized).eq_ignore_ascii_case("skin.ini")
        })
        .and_then(|(_, bytes)| String::from_utf8(bytes.clone()).ok())
        .ok_or_else(|| JsError::new("skin.ini not found or invalid UTF-8"))?;

    let skin_ini = osu::OsuSkinIni::from_str(&ini_content)
        .map_err(|e| JsError::new(&e.to_string()))?;
    
    let texture_paths = skin_ini.get_required_texture_paths();
    let sample_paths = skin_ini.get_required_sample_paths();
    
    let texture_path_refs: Vec<&str> = string_iter_as_str(texture_paths.iter());
    let sample_path_refs: Vec<&str> = string_iter_as_str(sample_paths.iter());
    
    let textures = import_textures_from_files(files, &texture_path_refs)?;
    let samples = import_samples_from_files(files, &sample_path_refs)?;

    Ok(OsuSkin::new(skin_ini, Some(textures), Some(samples)))
}

pub fn import_fluxis_skin_from_files(
    files: &HashMap<String, Vec<u8>>,
) -> Result<FluXisSkin, JsError> {
    let json_content = files
        .iter()
        .find(|(path, _)| {
            let normalized = normalize(path);
            get_filename(&normalized).eq_ignore_ascii_case("skin.json")
        })
        .and_then(|(_, bytes)| String::from_utf8(bytes.clone()).ok())
        .ok_or_else(|| JsError::new("skin.json not found or invalid UTF-8"))?;
    
    let skin_json = fluxis::SkinJson::from_str(&json_content)
        .map_err(|e| JsError::new(&e.to_string()))?;

    let sample_paths = skin_json.get_required_sample_paths();
    let sample_path_refs: Vec<&str> = string_iter_as_str(sample_paths.iter());
    
    let textures = import_all_textures_from_files(files)?;
    let samples = import_samples_from_files(files, &sample_path_refs)?;

    Ok(FluXisSkin::new(skin_json, Some(textures), Some(samples)))
}
