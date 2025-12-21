use std::io;
use std::fs;
use std::path::Path;
use crate::utils::io::change_extension;
use crate::Store;
use crate::{osu, fluxis, texture::TextureStore};

pub fn export_textures(textures: &TextureStore, path: &str) -> io::Result<()> {
    fs::create_dir_all(path)?;
    
    let mut result: io::Result<()> = Ok(());
    textures.for_each(|texture| {
        if result.is_err() { return; }
        let texture_path_with_ext = change_extension(texture.path(), "png");
        let output_path = Path::new(path).join(&texture_path_with_ext);

        if let Some(parent) = output_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                result = Err(e);
                return;
            }
        }

        if let Some(img) = texture.get_data() {
            if let Err(e) = img.save_with_format(&output_path, image::ImageFormat::Png)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e)) {
                result = Err(e);
                return;
            }
        } else if let Some(bytes) = texture.get_unloaded_data() {
            if let Err(e) = fs::write(&output_path, bytes) {
                result = Err(e);
                return;
            }
        }
    });
    result?;
    
    Ok(())
}

pub fn export_osu_ini(skin_ini: &osu::SkinIni, path: &str) -> io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    
    let ini_content = skin_ini.to_string();
    
    fs::write(path, ini_content)?;
    
    Ok(())
}

pub fn export_fluxis_skin_json(skin_json: &fluxis::SkinJson, path: &str) -> io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    
    let json_content = skin_json.to_string();
    
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