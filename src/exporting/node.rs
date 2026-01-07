#![cfg(feature = "node")]

#[cfg(all(target_arch = "wasm32", feature = "node"))]
use crate::io::node;
use std::io;
use crate::sample::SampleStore;
use crate::utils::io::change_extension;
use crate::Binary;
use crate::FluXisSkin;
use crate::OsuSkin;
use crate::Store;
use crate::{osu, fluxis, texture::TextureStore};

pub fn export_binaries<T, S, F>(
    store: &S,
    path: &str,
    mut exporter: F,
) -> io::Result<()>
where
    S: Store<T>,
    T: 'static,
    F: FnMut(&T, &str) -> io::Result<()>,
{
    node::create_dir_all(path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    
    let mut result: io::Result<()> = Ok(());
    store.for_each(|item| {
        if result.is_err() { return; }
        
        if let Err(e) = exporter(item, path) {
            result = Err(e);
        }
    });
    result?;
    
    Ok(())
}

pub fn export_textures(textures: &TextureStore, path: &str) -> io::Result<()> {
    export_binaries(textures, path, |texture, base_path| {
        let texture_path_with_ext = change_extension(texture.get_path(), "png");
        let output_path = node::join_path(base_path, &texture_path_with_ext);
        
        if let Some(slash_pos) = output_path.rfind('/') {
            let parent = &output_path[..slash_pos];
            node::create_dir_all(parent)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        } else if let Some(slash_pos) = output_path.rfind('\\') {
            let parent = &output_path[..slash_pos];
            node::create_dir_all(parent)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        }
        
        if let Some(img) = texture.get_data() {
            let mut bytes: Vec<u8> = Vec::new();
            img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            
            node::write_file(&output_path, &bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        } else if let Some(bytes) = texture.get_unloaded_data() {
            node::write_file(&output_path, bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        } else {
            Ok(())
        }
    })
}

pub fn export_samples(samples: &SampleStore, path: &str) -> io::Result<()> {
    export_binaries(samples, path, |sample, base_path| {
        let sample_path_with_ext = change_extension(sample.get_path(), "wav");
        let output_path = node::join_path(base_path, &sample_path_with_ext);
        
        if let Some(slash_pos) = output_path.rfind('/') {
            let parent = &output_path[..slash_pos];
            node::create_dir_all(parent)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        } else if let Some(slash_pos) = output_path.rfind('\\') {
            let parent = &output_path[..slash_pos];
            node::create_dir_all(parent)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        }
        
        if let Some(bytes) = sample.get_data() {
            node::write_file(&output_path, bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        } else {
            Ok(())
        }
    })
}

pub fn export_osu_ini(skin_ini: &osu::OsuSkinIni, path: &str) -> io::Result<()> {
    if let Some(slash_pos) = path.rfind('/') {
        let parent = &path[..slash_pos];
        node::create_dir_all(parent)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    } else if let Some(slash_pos) = path.rfind('\\') {
        let parent = &path[..slash_pos];
        node::create_dir_all(parent)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    }
    
    let ini_content = skin_ini.to_string();
    
    node::write_file(path, ini_content.as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    
    Ok(())
}

pub fn export_fluxis_skin_json(skin_json: &fluxis::SkinJson, path: &str) -> io::Result<()> {
    if let Some(slash_pos) = path.rfind('/') {
        let parent = &path[..slash_pos];
        node::create_dir_all(parent)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    } else if let Some(slash_pos) = path.rfind('\\') {
        let parent = &path[..slash_pos];
        node::create_dir_all(parent)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    }
    
    let json_content = skin_json.to_string();
    
    node::write_file(path, json_content.as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    
    Ok(())
}

pub fn export_fluxis_layout_json(layout_json: &fluxis::FluXisLayout, path: &str) -> io::Result<()> {
    if let Some(slash_pos) = path.rfind('/') {
        let parent = &path[..slash_pos];
        node::create_dir_all(parent)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    } else if let Some(slash_pos) = path.rfind('\\') {
        let parent = &path[..slash_pos];
        node::create_dir_all(parent)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    }
    
    let json_content = layout_json.to_str().unwrap();
    
    node::write_file(path, json_content.as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    
    Ok(())
}

pub fn export_osu_skin(skin: &OsuSkin, path: &str) -> io::Result<()> {
    let skin_ini = &skin.skin_ini;
    let skin_path = node::join_path(path, &skin_ini.general.name);
    
    node::create_dir_all(&skin_path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    
    let ini_path = node::join_path(&skin_path, "skin.ini");
    export_osu_ini(skin_ini, &ini_path)?;
    
    export_textures(&skin.textures, &skin_path)?;
    export_samples(&skin.samples, &skin_path)?;
    
    Ok(())
}

pub fn export_fluxis_skin(skin: &FluXisSkin, path: &str) -> io::Result<()> {
    let skin_json = &skin.skin_json;
    let skin_path = node::join_path(path, &skin_json.info.name);
    
    node::create_dir_all(&skin_path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    
    let json_path = node::join_path(&skin_path, "skin.json");
    export_fluxis_skin_json(skin_json, &json_path)?;
    
    export_textures(&skin.textures, &skin_path)?;
    export_samples(&skin.samples, &skin_path)?;
    
    Ok(())
}