use std::fs;
use std::path::Path;
use crate::osu;
use crate::utils::io::{get_filename, get_parent, get_stem, join_paths_unix};
use crate::utils::string::string_iter_as_str;
use crate::OsuSkin;
use crate::TextureStore;

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

pub fn import_osu_ini_str(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

pub fn import_osu_mania_skin_from_dir(path: &str) -> Result<OsuSkin, Box<dyn std::error::Error>> {
    let ini_path = Path::new(path).join("skin.ini");
    let ini_content = import_osu_ini_str(ini_path.to_str().unwrap());
    
    let skin_ini = osu::SkinIni::from_str(&ini_content)?;
    
    let texture_paths = skin_ini.get_mania_texture_paths();
    
    let texture_path_refs: Vec<&str> = string_iter_as_str(texture_paths.iter());
    
    let textures = import_textures_from_dir(path, &texture_path_refs)?;
    
    Ok(OsuSkin {
        skin_ini,
        textures,
    })
}
