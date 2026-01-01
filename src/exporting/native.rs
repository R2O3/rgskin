use std::io;
use std::fs;
use std::path::Path;
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
    fs::create_dir_all(path)?;
    
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
        let output_path = Path::new(base_path).join(&texture_path_with_ext);
        
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        if let Some(img) = texture.get_data() {
            img.save_with_format(&output_path, image::ImageFormat::Png)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        } else if let Some(bytes) = texture.get_unloaded_data() {
            fs::write(&output_path, bytes)
        } else {
            Ok(())
        }
    })
}

pub fn export_samples(samples: &SampleStore, path: &str) -> io::Result<()> {
    export_binaries(samples, path, |sample, base_path| {
        let sample_path_with_ext = change_extension(sample.get_path(), "wav"); // TODO: preserve original extension to avoid bugs
        let output_path = Path::new(base_path).join(&sample_path_with_ext);
        
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        if let Some(bytes) = sample.get_data() {
            fs::write(&output_path, bytes)
        } else {
            Ok(())
        }
    })
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

pub fn export_osu_skin(skin: &OsuSkin, path: &str) -> io::Result<()> {
    let skin_ini = &skin.skin_ini;

    let skin_path = Path::new(path).join(&skin_ini.general.name);
    fs::create_dir_all(&skin_path)?;
    
    let ini_path = skin_path.join("skin.ini");
    export_osu_ini(skin_ini, ini_path.to_str().unwrap())?;
    
    export_textures(&skin.textures, skin_path.to_str().unwrap())?;
    export_samples(&skin.samples, skin_path.to_str().unwrap())?;
    
    Ok(())
}

pub fn export_fluxis_skin(skin: &FluXisSkin, path: &str) -> io::Result<()> {
    let skin_json = &skin.skin_json;
    let skin_path = Path::new(path).join(&skin_json.info.name);
    fs::create_dir_all(&skin_path)?;
    
    let json_path = skin_path.join("skin.json");
    export_fluxis_skin_json(skin_json, json_path.to_str().unwrap())?;
    
    export_textures(&skin.textures, skin_path.to_str().unwrap())?;
    export_samples(&skin.samples, skin_path.to_str().unwrap())?;
    
    Ok(())
}