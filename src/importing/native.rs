use std::fs;
use std::path::Path;
use crate::fluxis::{self, FluXisSkin};
use crate::osu;
use crate::utils::io::{get_filename, get_parent, get_stem, join_paths_unix, remove_extension};
use crate::utils::string::string_iter_as_str;
use crate::OsuSkin;
use crate::io::texture::TextureStore;

pub fn import_textures_from_dir(path: &str, relative_texture_paths: &[&str]) -> Result<TextureStore, Box<dyn std::error::Error>> {
    let mut texture_store = TextureStore::new();
    
    for &texture_path in relative_texture_paths {
        let dir_path = join_paths_unix(path, &get_parent(texture_path));
        let filename = get_filename(texture_path);
        
        if let Ok(entries) = fs::read_dir(&dir_path) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    let file_stem = get_stem(&file_name);
                    if file_stem == filename {
                        let full_path = join_paths_unix(&dir_path, &file_name);
                        
                        if let Ok(bytes) = fs::read(&full_path) {
                            
                            texture_store.load_from_bytes(texture_path.to_string(), bytes)?;
                            break;
                        }
                    }
                }
            }
        }
    }
    
    Ok(texture_store)
}

pub fn import_all_textures_from_dir(path: &str) -> Result<TextureStore, Box<dyn std::error::Error>> {
    let mut texture_store = TextureStore::new();
    
    fn recurse(
        texture_store: &mut TextureStore,
        dir_path: &str,
        base_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                
                if entry_path.is_dir() {
                    if let Some(path_str) = entry_path.to_str() {
                        recurse(texture_store, path_str, base_path)?;
                    }
                } else if let Some(ext) = entry_path.extension() {
                    let ext_lower = ext.to_string_lossy().to_lowercase();
                    if matches!(ext_lower.as_str(), "png" | "jpg" | "jpeg") {
                        if let (Some(full_path_str), Some(file_name)) = 
                            (entry_path.to_str(), entry_path.file_name().and_then(|n| n.to_str())) {
                            
                            let relative_path = full_path_str
                                .strip_prefix(base_path)
                                .unwrap_or(file_name)
                                .trim_start_matches('/').trim_start_matches('\\');
                            
                            if let Ok(bytes) = fs::read(&full_path_str) {
                                texture_store.load_from_bytes(remove_extension(relative_path), bytes)?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    recurse(&mut texture_store, path, path)?;
    Ok(texture_store)
}

pub fn read_str_from_path(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

pub fn import_osu_mania_skin_from_dir(path: &str) -> Result<OsuSkin, Box<dyn std::error::Error>> {
    let ini_path = Path::new(path).join("skin.ini");
    let ini_content = read_str_from_path(ini_path.to_str().unwrap());
    
    let skin_ini = osu::SkinIni::from_str(&ini_content)?;
    
    let texture_paths = skin_ini.get_mania_texture_paths();
    
    let texture_path_refs: Vec<&str> = string_iter_as_str(texture_paths.iter());
    
    let textures = import_textures_from_dir(path, &texture_path_refs)?;

    Ok(OsuSkin::new(skin_ini, Some(textures)))
}

pub fn import_fluxis_skin_from_dir(path: &str) -> Result<FluXisSkin, Box<dyn std::error::Error>> {
    let json_path = Path::new(path).join("skin.json");
    let json_content = read_str_from_path(json_path.to_str().unwrap());
    
    let skin_json = fluxis::SkinJson::from_str(&json_content)?;
    
    let textures = import_all_textures_from_dir(path)?;

    Ok(FluXisSkin::new(skin_json, Some(textures)))
}
