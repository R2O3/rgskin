#![cfg(all(target_arch = "wasm32", feature = "browser"))]

use std::collections::HashMap;
use wasm_bindgen::JsError;
use crate::sample::SampleStore;
use crate::utils::io::change_extension;
use crate::Binary;
use crate::FluXisSkin;
use crate::OsuSkin;
use crate::Store;
use crate::{osu, fluxis, texture::TextureStore};

pub fn export_binaries<T, S, F>(
    store: &S,
    mut exporter: F,
) -> Result<HashMap<String, Vec<u8>>, JsError>
where
    S: Store<T>,
    T: 'static,
    F: FnMut(&T) -> Result<Option<(String, Vec<u8>)>, JsError>,
{
    let mut files = HashMap::new();
    let mut result: Result<(), JsError> = Ok(());
    
    store.for_each(|item| {
        if result.is_err() { return; }
        
        match exporter(item) {
            Ok(Some((path, bytes))) => {
                files.insert(path, bytes);
            }
            Ok(None) => {}
            Err(e) => {
                result = Err(e);
            }
        }
    });
    result?;
    
    Ok(files)
}

pub fn export_textures(textures: &TextureStore) -> Result<HashMap<String, Vec<u8>>, JsError> {
    export_binaries(textures, |texture| {
        let texture_path_with_ext = change_extension(texture.get_path(), "png");
        
        if let Some(img) = texture.get_data() {
            let mut bytes = Vec::new();
            let mut cursor = std::io::Cursor::new(&mut bytes);
            img.write_to(&mut cursor, image::ImageFormat::Png)
                .map_err(|e| JsError::new(&e.to_string()))?;
            Ok(Some((texture_path_with_ext, bytes)))
        } else if let Some(bytes) = texture.get_unloaded_data() {
            Ok(Some((texture_path_with_ext, bytes.to_vec())))
        } else {
            Ok(None)
        }
    })
}

pub fn export_samples(samples: &SampleStore) -> Result<HashMap<String, Vec<u8>>, JsError> {
    export_binaries(samples, |sample| {
        let sample_path_with_ext = change_extension(sample.get_path(), "wav"); // TODO: preserve original extension to avoid bugs
        
        if let Some(bytes) = sample.get_data() {
            Ok(Some((sample_path_with_ext, bytes.to_vec())))
        } else {
            Ok(None)
        }
    })
}

pub fn export_osu_ini(skin_ini: &osu::SkinIni) -> String {
    skin_ini.to_string()
}

pub fn export_fluxis_skin_json(skin_json: &fluxis::SkinJson) -> String {
    skin_json.to_string()
}

pub fn export_fluxis_layout_json(layout_json: &fluxis::FluXisLayout) -> Result<String, JsError> {
    layout_json.to_str()
        .map_err(|e| JsError::new(&e.to_string()))
}

pub fn export_osu_skin(skin: &OsuSkin) -> Result<HashMap<String, Vec<u8>>, JsError> {
    let mut files = HashMap::new();
    
    let ini_content = export_osu_ini(&skin.skin_ini);
    files.insert("skin.ini".to_string(), ini_content.into_bytes());
    
    let texture_files = export_textures(&skin.textures)?;
    files.extend(texture_files);
    
    let sample_files = export_samples(&skin.samples)?;
    files.extend(sample_files);
    
    Ok(files)
}

pub fn export_fluxis_skin(skin: &FluXisSkin) -> Result<HashMap<String, Vec<u8>>, JsError> {
    let mut files = HashMap::new();
    
    let json_content = export_fluxis_skin_json(&skin.skin_json);
    files.insert("skin.json".to_string(), json_content.into_bytes());
    
    let texture_files = export_textures(&skin.textures)?;
    files.extend(texture_files);
    
    let sample_files = export_samples(&skin.samples)?;
    files.extend(sample_files);
    
    Ok(files)
}
