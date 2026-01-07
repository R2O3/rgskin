pub(crate) mod skin;
pub(crate) mod converting;
pub(crate) mod exporting;
pub(crate) mod importing;
pub(crate) mod io;
pub(crate) mod common;

mod resources;

mod parse;

pub mod image_proc;
pub mod utils;
pub mod extensions;

pub use skin::osu;
pub use skin::fluxis;
pub use skin::generic;
pub use parse::ini;

pub use osu::OsuSkin;
pub use fluxis::FluXisSkin;
pub use generic::GenericManiaSkin;

pub use extensions::{TextureArcExt, BinaryArcExt, BinaryArcExtOption};
pub use io::{Binary, BinaryState, Store, texture, sample};
pub use common::traits;

pub(crate) use resources::Resources;

pub mod prelude {
    pub use crate::OsuSkin;
    pub use crate::GenericManiaSkin;
    pub use crate::FluXisSkin;
    
    pub use crate::osu::OsuSkinIni;
    pub use crate::fluxis::{SkinJson, FluXisLayout};
    
    pub use crate::io::{Binary, RawBytes, BinaryState, BinaryStore, Store};
    pub use crate::io::texture::{TextureStore, Texture};
    pub use crate::io::sample::{SampleStore, Sample};
    
    pub use crate::common::traits::*;
    pub use crate::extensions::*;
    
    pub use crate::export;
    pub use crate::import;
}

#[cfg(not(target_arch = "wasm32"))]
pub mod export {
    use std::io;
    use crate::{exporting::native::{export_samples, export_textures}, io::texture::TextureStore, sample::SampleStore};

    pub fn textures_to_dir(textures: &TextureStore, path: &str) -> io::Result<()>  {
        export_textures(textures, path)
    }

    pub fn samples_to_dir(samples: &SampleStore, path: &str) -> io::Result<()>  {
        export_samples(samples, path)
    }

    pub mod osu {
        use std::io;

        use crate::{exporting::native::{export_osu_ini, export_osu_skin}, osu};

        pub fn skin_to_dir(skin: &osu::OsuSkin, path: &str) -> io::Result<()> {
            export_osu_skin(skin, path)
        }

        pub fn ini_to_dir(skin_ini: &osu::OsuSkinIni, path: &str) -> io::Result<()> {
            export_osu_ini(skin_ini, path)
        }
    }

    pub mod fluxis {
        use std::io;
        use crate::{exporting::native::{export_fluxis_layout_json, export_fluxis_skin, export_fluxis_skin_json}, fluxis};

        pub fn skin_to_dir(skin: &fluxis::FluXisSkin, path: &str) -> io::Result<()> {
            export_fluxis_skin(skin, path)
        }

        pub fn layout_to_dir(layout_json: &fluxis::FluXisLayout, path: &str) -> io::Result<()> {
            export_fluxis_layout_json(layout_json, path)
        }

        pub fn json_to_dir(skin_json: &fluxis::SkinJson, path: &str) -> io::Result<()> {
            export_fluxis_skin_json(skin_json, path)
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "browser"))]
pub mod export {
    use wasm_bindgen::prelude::*;
    use js_sys::{Map, Uint8Array};
    use crate::{exporting::browser::{export_samples, export_textures}, io::texture::TextureStore, sample::SampleStore};

    #[wasm_bindgen(js_name = texturesToFiles)]
    pub fn textures_to_files(textures: &TextureStore) -> Result<Map, JsError> {
        let files_map = export_textures(textures)?;
        
        let js_map = Map::new();
        for (key, value) in files_map {
            let uint8_array = Uint8Array::from(&value[..]);
            js_map.set(&JsValue::from_str(&key), &uint8_array);
        }
        
        Ok(js_map)
    }

    #[wasm_bindgen(js_name = samplesToFiles)]
    pub fn samples_to_files(samples: &SampleStore) -> Result<Map, JsError> {
        let files_map = export_samples(samples)?;
        
        let js_map = Map::new();
        for (key, value) in files_map {
            let uint8_array = Uint8Array::from(&value[..]);
            js_map.set(&JsValue::from_str(&key), &uint8_array);
        }
        
        Ok(js_map)
    }

    pub mod osu {
        use wasm_bindgen::prelude::*;
        use js_sys::{Map, Uint8Array};
        use crate::{exporting::browser::{export_osu_ini, export_osu_skin}, osu};

        #[wasm_bindgen(js_name = osuSkinToFiles)]
        pub fn skin_to_files(skin: &osu::OsuSkin) -> Result<Map, JsError> {
            let files_map = export_osu_skin(skin)?;
            
            let js_map = Map::new();
            for (key, value) in files_map {
                let uint8_array = Uint8Array::from(&value[..]);
                js_map.set(&JsValue::from_str(&key), &uint8_array);
            }
            
            Ok(js_map)
        }

        #[wasm_bindgen(js_name = iniToString)]
        pub fn ini_to_string(skin_ini: &osu::OsuSkinIni) -> String {
            export_osu_ini(skin_ini)
        }
    }

    pub mod fluxis {
        use wasm_bindgen::prelude::*;
        use js_sys::{Map, Uint8Array};
        use crate::{exporting::browser::{export_fluxis_layout_json, export_fluxis_skin, export_fluxis_skin_json}, fluxis};

        #[wasm_bindgen(js_name = fluXisSkinToFiles)]
        pub fn skin_to_files(skin: &fluxis::FluXisSkin) -> Result<Map, JsError> {
            let files_map = export_fluxis_skin(skin)?;
            
            let js_map = Map::new();
            for (key, value) in files_map {
                let uint8_array = Uint8Array::from(&value[..]);
                js_map.set(&JsValue::from_str(&key), &uint8_array);
            }
            
            Ok(js_map)
        }

        #[wasm_bindgen(js_name = layoutToString)]
        pub fn layout_to_string(layout_json: &fluxis::FluXisLayout) -> Result<String, JsError> {
            export_fluxis_layout_json(layout_json)
        }

        #[wasm_bindgen(js_name = jsonToString)]
        pub fn json_to_string(skin_json: &fluxis::SkinJson) -> String {
            export_fluxis_skin_json(skin_json)
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub mod export {
    use wasm_bindgen::prelude::*;
    use crate::io::texture::TextureStore;
    use crate::sample::SampleStore;
    use crate::exporting::node::{
        export_samples,
        export_textures,
    };

    #[wasm_bindgen(js_name = texturesToDir)]
    pub fn textures_to_dir(textures: &TextureStore, path: &str) -> Result<(), JsError> {
        export_textures(textures, path).map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = samplesToDir)]
    pub fn samples_to_dir(samples: &SampleStore, path: &str) -> Result<(), JsError> {
        export_samples(samples, path).map_err(|e| JsError::new(&e.to_string()))
    }

    pub mod osu {
        use wasm_bindgen::prelude::*;
        use crate::osu;
        use crate::exporting::node::{export_osu_ini, export_osu_skin};

        #[wasm_bindgen(js_name = osuSkinToDir)]
        pub fn skin_to_dir(skin: &osu::OsuSkin, path: &str) -> Result<(), JsError> {
            export_osu_skin(skin, path).map_err(|e| JsError::new(&e.to_string()))
        }

        #[wasm_bindgen(js_name = iniToDir)]
        pub fn ini_to_dir(skin_ini: &osu::OsuSkinIni, path: &str) -> Result<(), JsError> {
            export_osu_ini(skin_ini, path).map_err(|e| JsError::new(&e.to_string()))
        }
    }

    pub mod fluxis {
        use wasm_bindgen::prelude::*;
        use crate::fluxis;
        use crate::exporting::node::{export_fluxis_layout_json, export_fluxis_skin, export_fluxis_skin_json};

        #[wasm_bindgen(js_name = fluXisSkinToDir)]
        pub fn skin_to_dir(skin: &fluxis::FluXisSkin, path: &str) -> Result<(), JsError> {
            export_fluxis_skin(skin, path).map_err(|e| JsError::new(&e.to_string()))
        }

        #[wasm_bindgen(js_name = layoutToDir)]
        pub fn layout_to_dir(layout_json: &fluxis::FluXisLayout, path: &str) -> Result<(), JsError> {
            export_fluxis_layout_json(layout_json, path).map_err(|e| JsError::new(&e.to_string()))
        }

        #[wasm_bindgen(js_name = jsonToDir)]
        pub fn json_to_dir(skin_json: &fluxis::SkinJson, path: &str) -> Result<(), JsError> {
            export_fluxis_skin_json(skin_json, path).map_err(|e| JsError::new(&e.to_string()))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod import {
    use crate::{importing::native::{import_all_samples_from_dir, import_all_textures_from_dir, import_textures_from_dir, import_samples_from_dir}, io::texture::TextureStore, sample::SampleStore};

    pub fn textures_from_dir(path: &str, relative_texture_paths: &[&str]) -> Result<TextureStore, Box<dyn std::error::Error>>  {
        import_textures_from_dir(path, relative_texture_paths)
    }

    pub fn all_textures_from_dir(path: &str) -> Result<TextureStore, Box<dyn std::error::Error>>  {
        import_all_textures_from_dir(path)
    }

    pub fn samples_from_dir(path: &str, relative_sample_paths: &[&str]) -> Result<SampleStore, Box<dyn std::error::Error>>  {
        import_samples_from_dir(path, relative_sample_paths)
    }

    pub fn all_samples_from_dir(path: &str) -> Result<SampleStore, Box<dyn std::error::Error>>  {
        import_all_samples_from_dir(path)
    }

    pub mod osu {
        use crate::{importing::native::{read_str_from_path, import_osu_mania_skin_from_dir}, OsuSkin};

        pub fn skin_from_dir(path: &str) -> Result<OsuSkin, Box<dyn std::error::Error>> {
            import_osu_mania_skin_from_dir(path)
        }

        pub fn ini_str_from_dir(path: &str) -> String {
            read_str_from_path(path)
        }
    }

    pub mod fluxis {
        use crate::{importing::native::{read_str_from_path, import_fluxis_skin_from_dir}, fluxis::FluXisSkin};

        pub fn skin_from_dir(path: &str) -> Result<FluXisSkin, Box<dyn std::error::Error>> {
            import_fluxis_skin_from_dir(path)
        }

        pub fn json_str_from_dir(path: &str) -> String {
            read_str_from_path(path)
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "browser"))]
pub mod import {
    use std::collections::HashMap;
    use wasm_bindgen::prelude::*;
    use js_sys::{Array, Map, Uint8Array};
    use crate::{importing::browser::{import_all_samples_from_files, import_all_textures_from_files, import_samples_from_files, import_textures_from_files}, io::texture::TextureStore, sample::SampleStore};

    #[wasm_bindgen(js_name = texturesFromFiles)]
    pub fn textures_from_files(files: Map, relative_texture_paths: Array) -> Result<TextureStore, JsError>  {
        let mut files_map = HashMap::new();
        
        files.for_each(&mut |value, key| {
            if let Some(key_str) = key.as_string() {
                let uint8_array = Uint8Array::new(&value);
                let vec = uint8_array.to_vec();
                files_map.insert(key_str, vec);
            }
        });
        
        let paths: Result<Vec<String>, JsError> = relative_texture_paths
            .iter()
            .map(|v| v.as_string().ok_or_else(|| JsError::new("Expected string in array")))
            .collect();
        
        let paths = paths?;
        let path_refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        
        import_textures_from_files(&files_map, &path_refs)
    }

    #[wasm_bindgen(js_name = allTexturesFromFiles)]
    pub fn all_textures_from_files(files: Map) -> Result<TextureStore, JsError>  {
        let mut files_map = HashMap::new();
        
        files.for_each(&mut |value, key| {
            if let Some(key_str) = key.as_string() {
                let uint8_array = Uint8Array::new(&value);
                let vec = uint8_array.to_vec();
                files_map.insert(key_str, vec);
            }
        });
        
        import_all_textures_from_files(&files_map)
    }

    #[wasm_bindgen(js_name = samplesFromFiles)]
    pub fn samples_from_files(files: Map, relative_sample_paths: Array) -> Result<SampleStore, JsError>  {
        let mut files_map = HashMap::new();
        
        files.for_each(&mut |value, key| {
            if let Some(key_str) = key.as_string() {
                let uint8_array = Uint8Array::new(&value);
                let vec = uint8_array.to_vec();
                files_map.insert(key_str, vec);
            }
        });
        
        let paths: Result<Vec<String>, JsError> = relative_sample_paths
            .iter()
            .map(|v| v.as_string().ok_or_else(|| JsError::new("Expected string in array")))
            .collect();
        
        let paths = paths?;
        let path_refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        
        import_samples_from_files(&files_map, &path_refs)
    }

    #[wasm_bindgen(js_name = allSamplesFromFiles)]
    pub fn all_samples_from_files(files: Map) -> Result<SampleStore, JsError>  {
        let mut files_map = HashMap::new();
        
        files.for_each(&mut |value, key| {
            if let Some(key_str) = key.as_string() {
                let uint8_array = Uint8Array::new(&value);
                let vec = uint8_array.to_vec();
                files_map.insert(key_str, vec);
            }
        });
        
        import_all_samples_from_files(&files_map)
    }

    pub mod osu {
        use std::collections::HashMap;
        use wasm_bindgen::prelude::*;
        use js_sys::{Map, Uint8Array};
        use crate::{importing::browser::import_osu_mania_skin_from_files, OsuSkin};

        #[wasm_bindgen(js_name = osuSkinFromFiles)]
        pub fn skin_from_files(files: Map) -> Result<OsuSkin, JsError> {
            let mut files_map = HashMap::new();
            
            files.for_each(&mut |value, key| {
                if let Some(key_str) = key.as_string() {
                    let uint8_array = Uint8Array::new(&value);
                    let vec = uint8_array.to_vec();
                    files_map.insert(key_str, vec);
                }
            });
            
            import_osu_mania_skin_from_files(&files_map)
        }
    }

    pub mod fluxis {
        use std::collections::HashMap;
        use wasm_bindgen::prelude::*;
        use js_sys::{Map, Uint8Array};
        use crate::{importing::browser::import_fluxis_skin_from_files, fluxis::FluXisSkin};

        #[wasm_bindgen(js_name = fluXisSkinFromFiles)]
        pub fn skin_from_files(files: Map) -> Result<FluXisSkin, JsError> {
            let mut files_map = HashMap::new();
            
            files.for_each(&mut |value, key| {
                if let Some(key_str) = key.as_string() {
                    let uint8_array = Uint8Array::new(&value);
                    let vec = uint8_array.to_vec();
                    files_map.insert(key_str, vec);
                }
            });
            
            import_fluxis_skin_from_files(&files_map)
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub mod import {
    use wasm_bindgen::prelude::*;
    use js_sys::Array;
    use crate::io::texture::TextureStore;
    use crate::sample::SampleStore;
    use crate::importing::node::{
        import_all_samples_from_dir,
        import_all_textures_from_dir,
        import_samples_from_dir,
        import_textures_from_dir,
    };

    #[wasm_bindgen(js_name = texturesFromDir)]
    pub fn textures_from_dir(path: &str, relative_texture_paths: Array) -> Result<TextureStore, JsError> {
        let paths: Result<Vec<String>, JsError> = relative_texture_paths
            .iter()
            .map(|v| v.as_string().ok_or_else(|| JsError::new("Expected string in array")))
            .collect();
        
        let paths = paths?;
        let path_refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        
        import_textures_from_dir(path, &path_refs).map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = allTexturesFromDir)]
    pub fn all_textures_from_dir(path: &str) -> Result<TextureStore, JsError> {
        import_all_textures_from_dir(path).map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = samplesFromDir)]
    pub fn samples_from_dir(path: &str, relative_sample_paths: Array) -> Result<SampleStore, JsError> {
        let paths: Result<Vec<String>, JsError> = relative_sample_paths
            .iter()
            .map(|v| v.as_string().ok_or_else(|| JsError::new("Expected string in array")))
            .collect();
        
        let paths = paths?;
        let path_refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        
        import_samples_from_dir(path, &path_refs).map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = allSamplesFromDir)]
    pub fn all_samples_from_dir(path: &str) -> Result<SampleStore, JsError> {
        import_all_samples_from_dir(path).map_err(|e| JsError::new(&e.to_string()))
    }

    pub mod osu {
        use wasm_bindgen::prelude::*;
        use crate::OsuSkin;
        use crate::importing::node::{import_osu_mania_skin_from_dir, read_str_from_path};

        #[wasm_bindgen(js_name = osuSkinFromDir)]
        pub fn skin_from_dir(path: &str) -> Result<OsuSkin, JsError> {
            import_osu_mania_skin_from_dir(path).map_err(|e| JsError::new(&e.to_string()))
        }

        #[wasm_bindgen(js_name = iniStrFromDir)]
        pub fn ini_str_from_dir(path: &str) -> String {
            read_str_from_path(path)
        }
    }

    pub mod fluxis {
        use wasm_bindgen::prelude::*;
        use crate::fluxis::FluXisSkin;
        use crate::importing::node::{import_fluxis_skin_from_dir, read_str_from_path};

        #[wasm_bindgen(js_name = fluXisSkinFromDir)]
        pub fn skin_from_dir(path: &str) -> Result<FluXisSkin, JsError> {
            import_fluxis_skin_from_dir(path).map_err(|e| JsError::new(&e.to_string()))
        }

        #[wasm_bindgen(js_name = jsonStrFromDir)]
        pub fn json_str_from_dir(path: &str) -> String {
            read_str_from_path(path)
        }
    }
}
