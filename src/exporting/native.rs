use std::io;
use std::fs;
use std::path::Path;
use crate::utils::io::change_extension;
use crate::Store;
use crate::{osu, fluxis, TextureStore};

pub fn export_textures(textures: &TextureStore, path: &str) -> io::Result<()> {
    fs::create_dir_all(path)?;
    
    for texture_path in textures.get_paths() {
        if let Some(texture_ref) = textures.get(&texture_path) {
            if let Some(img) = texture_ref.data() {
                let texture_path_with_ext = change_extension(&texture_path, "png");
                let output_path = Path::new(path).join(&texture_path_with_ext);
                
                if let Some(parent) = output_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                
                img.save_with_format(&output_path, image::ImageFormat::Png)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            }
        }
    }
    
    Ok(())
}

pub fn export_osu_ini(skin_ini: &osu::SkinIni, path: &str) -> io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    
    let ini_content = skin_ini.to_str();
    
    fs::write(path, ini_content)?;
    
    Ok(())
}

pub fn export_fluxis_skin_json(skin_json: &fluxis::SkinJson, path: &str) -> io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    
    let json_content = skin_json.to_str()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    
    fs::write(path, json_content)?;
    
    Ok(())
}

pub fn export_fluxis_layout_json(layout_json: &fluxis::FluXisLayout, path: &str) -> io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    
    let json_content = layout_json.to_str().unwrap();
    
    fs::write(path, json_content)?;
    
    Ok(())
}

pub fn export_osu_skin(skin_ini: &osu::SkinIni, textures: Option<&TextureStore>, path: &str) -> io::Result<()> {
    let skin_path = Path::new(path).join(&skin_ini.general.name);
    fs::create_dir_all(&skin_path)?;
    
    let ini_path = skin_path.join("skin.ini");
    export_osu_ini(skin_ini, ini_path.to_str().unwrap())?;
    
    if let Some(texture_store) = textures {
        export_textures(texture_store, skin_path.to_str().unwrap())?;
    }
    
    Ok(())
}

pub fn export_fluxis_skin(skin_json: &fluxis::SkinJson, textures: Option<&TextureStore>, path: &str) -> io::Result<()> {
    let skin_path = Path::new(path).join(&skin_json.info.name);
    fs::create_dir_all(&skin_path)?;
    
    let json_path = skin_path.join("skin.json");
    export_fluxis_skin_json(skin_json, json_path.to_str().unwrap())?;
    
    if let Some(texture_store) = textures {
        export_textures(texture_store, skin_path.to_str().unwrap())?;
    }
    
    Ok(())
}