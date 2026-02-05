#![allow(unused)]
#![cfg(feature = "node")]

#[cfg(all(target_arch = "wasm32", feature = "node"))]
use crate::io::node;
use crate::common::traits::SkinConfig;
use crate::io::texture::TextureStore;
use crate::sample::SampleStore;
use crate::utils::io::{get_filename, get_parent, get_stem, remove_extension};
use crate::importing::common::{file_matches_target, extension_matches, choose_best_match, SeenFiles};
use std::str::FromStr;

pub fn import_binaries_from_dir<F>(
    path: &str,
    relative_paths: &[&str],
    mut loader: F,
) -> Result<(), Box<dyn std::error::Error>> 
where
    F: FnMut(String, &[u8]) -> Result<(), Box<dyn std::error::Error>>,
{
    for &relative_path in relative_paths {
        let dir_path = node::join_path(path, &get_parent(relative_path));
        let filename = get_filename(relative_path);
        
        if let Ok(entries) = node::read_dir(&dir_path) {
            let mut matches: Vec<(String, String)> = Vec::new();
            
            for entry in entries {
                let file_stem = get_stem(&entry);
                
                if file_matches_target(&file_stem, &filename) {
                    let full_path = node::join_path(&dir_path, &entry);
                    matches.push((entry, full_path));
                }
            }
            
            let chosen_file = choose_best_match(&matches, &filename, |name| get_stem(name));
            
            if let Some((_, full_path)) = chosen_file {
                if let Ok(bytes) = node::read_file_bytes(full_path) {
                    loader(relative_path.to_string(), &bytes)?;
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
                } else {
                    if let Some(ext_pos) = entry.rfind('.') {
                        let ext = &entry[ext_pos + 1..];
                        
                        if extension_matches(ext, extensions) {
                            let relative_path = entry_path
                                .strip_prefix(base_path)
                                .unwrap_or(&entry)
                                .trim_start_matches('/').trim_start_matches('\\');
                            
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
        }
        Ok(())
    }
    
    let mut seen_files = SeenFiles::new();
    recurse(path, path, extensions, &mut loader, &mut seen_files)?;
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

pub fn import_all_samples_from_dir(path: &str) -> Result<SampleStore, Box<dyn std::error::Error>> {
    let mut sample_store = SampleStore::new();
    
    import_all_binaries_from_dir(path, &["wav", "ogg"], |path, bytes| {
        sample_store.load_from_bytes(path, bytes)?;
        Ok(())
    })?;
    
    Ok(sample_store)
}

pub fn import_samples_from_dir(path: &str, relative_sample_paths: &[&str]) -> Result<SampleStore, Box<dyn std::error::Error>> {
    let mut sample_store = SampleStore::new();
    
    import_binaries_from_dir(path, relative_sample_paths, |path, bytes| {
        sample_store.load_from_bytes(path, bytes)?;
        Ok(())
    })?;
    
    Ok(sample_store)
}

pub fn read_str_from_path(path: &str) -> String {
    node::read_file_string(path).unwrap_or_default()
}

pub fn import_osu_mania_skin_from_dir(path: &str) -> Result<crate::OsuSkin, Box<dyn std::error::Error>> {
    let ini_path = node::join_path(path, "skin.ini");
    let ini_content = node::read_file_string(&ini_path)?;

    let skin_ini = crate::osu::OsuSkinIni::from_str(&ini_content)?;
    
    let texture_paths = skin_ini.get_required_texture_paths();
    let sample_paths = skin_ini.get_required_sample_paths();
    
    let texture_path_refs: Vec<&str> = texture_paths.iter().map(|s| s.as_str()).collect();
    let sample_path_refs: Vec<&str> = sample_paths.iter().map(|s| s.as_str()).collect();
    
    let mut textures = TextureStore::new();
    import_binaries_from_dir(path, &texture_path_refs, |path, bytes| {
        textures.load_from_bytes(path, bytes)?;
        Ok(())
    })?;
    
    let mut samples = SampleStore::new();
    import_binaries_from_dir(path, &sample_path_refs, |path, bytes| {
        samples.load_from_bytes(path, bytes)?;
        Ok(())
    })?;

    Ok(crate::OsuSkin::new(skin_ini, Some(textures), Some(samples)))
}

pub fn import_fluxis_skin_from_dir(path: &str) -> Result<crate::fluxis::FluXisSkin, Box<dyn std::error::Error>> {
    let json_path = node::join_path(path, "skin.json");
    let json_content = node::read_file_string(&json_path)?;

    let skin_json = crate::fluxis::SkinJson::from_str(&json_content)?;
    
    let sample_paths = skin_json.get_required_sample_paths();
    let sample_path_refs: Vec<&str> = sample_paths.iter().map(|s| s.as_str()).collect();
    
    let mut textures = TextureStore::new();
    import_all_binaries_from_dir(path, &["png", "jpg", "jpeg"], |path, bytes| {
        textures.load_from_bytes(path, bytes)?;
        Ok(())
    })?;
    
    let mut samples = SampleStore::new();
    import_binaries_from_dir(path, &sample_path_refs, |path, bytes| {
        samples.load_from_bytes(path, bytes)?;
        Ok(())
    })?;

    Ok(crate::fluxis::FluXisSkin::new(skin_json, Some(textures), Some(samples)))
}