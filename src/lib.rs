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
pub use skin::quaver;
pub use skin::fluxis;
pub use skin::generic;
pub use parse::ini;

pub use osu::OsuSkin;
pub use quaver::QuaSkin;
pub use fluxis::FluXisSkin;
pub use generic::GenericManiaSkin;

pub use extensions::{TextureArcExt, BinaryArcExt, BinaryArcExtOption};
pub use io::{Binary, BinaryState, Store, texture, sample};
pub use common::traits;

pub(crate) use resources::Resources;
pub(crate) use io::StringPattern;
pub(crate) use common::macros::ConstTypeEnum;

pub mod prelude {
    pub use crate::GenericManiaSkin;
    pub use crate::OsuSkin;
    pub use crate::QuaSkin;
    pub use crate::FluXisSkin;
    
    pub use crate::osu::OsuSkinIni;
    pub use crate::quaver::QuaSkinIni;
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
    use crate::{exporting::native::*, io::texture::TextureStore, sample::SampleStore};

    pub fn textures_to_dir(textures: &TextureStore, path: &str) -> io::Result<()> { export_textures(textures, path) }
    pub fn samples_to_dir(samples: &SampleStore, path: &str) -> io::Result<()> { export_samples(samples, path) }

    pub mod osu {
        use super::*;
        pub fn skin_to_dir(skin: &crate::osu::OsuSkin, path: &str) -> io::Result<()> { export_osu_skin(skin, path) }
        pub fn ini_to_dir(skin_ini: &crate::osu::OsuSkinIni, path: &str) -> io::Result<()> { export_osu_ini(skin_ini, path) }
    }

    pub mod quaver {
        use super::*;
        pub fn skin_to_dir(skin: &crate::quaver::QuaSkin, path: &str) -> io::Result<()> { export_quaver_skin(skin, path) }
        pub fn ini_to_dir(skin_ini: &crate::quaver::QuaSkinIni, path: &str) -> io::Result<()> { export_quaver_ini(skin_ini, path) }
    }

    pub mod fluxis {
        use super::*;
        pub fn skin_to_dir(skin: &crate::fluxis::FluXisSkin, path: &str) -> io::Result<()> { export_fluxis_skin(skin, path) }
        pub fn layout_to_dir(layout_json: &crate::fluxis::FluXisLayout, path: &str) -> io::Result<()> { export_fluxis_layout_json(layout_json, path) }
        pub fn json_to_dir(skin_json: &crate::fluxis::SkinJson, path: &str) -> io::Result<()> { export_fluxis_skin_json(skin_json, path) }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "browser"))]
pub mod export {
    use wasm_bindgen::prelude::*;
    use js_sys::Map;
    use crate::{exporting::browser::*, io::texture::TextureStore, sample::SampleStore, utils::wasm::*};

    #[wasm_bindgen(js_name = texturesToFiles)]
    pub fn textures_to_files(textures: &TextureStore) -> Result<Map, JsError> { Ok(hash_to_js(export_textures(textures)?)) }

    #[wasm_bindgen(js_name = samplesToFiles)]
    pub fn samples_to_files(samples: &SampleStore) -> Result<Map, JsError> { Ok(hash_to_js(export_samples(samples)?)) }

    pub mod osu {
        use super::*;
        #[wasm_bindgen(js_name = osuSkinToFiles)]
        pub fn skin_to_files(skin: &crate::osu::OsuSkin) -> Result<Map, JsError> { Ok(hash_to_js(export_osu_skin(skin)?)) }

        #[wasm_bindgen(js_name = osuIniToString)]
        pub fn ini_to_string(skin_ini: &crate::osu::OsuSkinIni) -> String { export_osu_ini(skin_ini) }
    }

    pub mod quaver {
        use super::*;
        #[wasm_bindgen(js_name = quaverSkinToFiles)]
        pub fn skin_to_files(skin: &crate::quaver::QuaSkin) -> Result<Map, JsError> { Ok(hash_to_js(export_quaver_skin(skin)?)) }

        #[wasm_bindgen(js_name = quaverIniToString)]
        pub fn ini_to_string(skin_ini: &crate::quaver::QuaSkinIni) -> String { export_quaver_ini(skin_ini) }
    }

    pub mod fluxis {
        use super::*;
        #[wasm_bindgen(js_name = fluXisSkinToFiles)]
        pub fn skin_to_files(skin: &crate::fluxis::FluXisSkin) -> Result<Map, JsError> { Ok(hash_to_js(export_fluxis_skin(skin)?)) }

        #[wasm_bindgen(js_name = fluxisLayoutToString)]
        pub fn layout_to_string(layout_json: &crate::fluxis::FluXisLayout) -> Result<String, JsError> { export_fluxis_layout_json(layout_json) }

        #[wasm_bindgen(js_name = fluxisJsonToString)]
        pub fn json_to_string(skin_json: &crate::fluxis::SkinJson) -> String { export_fluxis_skin_json(skin_json) }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub mod export {
    use wasm_bindgen::prelude::*;
    use crate::{exporting::node::*, io::texture::TextureStore, sample::SampleStore};

    macro_rules! map_err { ($e:expr) => { $e.map_err(|e| JsError::new(&e.to_string())) } }

    #[wasm_bindgen(js_name = texturesToDir)]
    pub fn textures_to_dir(textures: &TextureStore, path: &str) -> Result<(), JsError> { map_err!(export_textures(textures, path)) }

    #[wasm_bindgen(js_name = samplesToDir)]
    pub fn samples_to_dir(samples: &SampleStore, path: &str) -> Result<(), JsError> { map_err!(export_samples(samples, path)) }

    pub mod osu {
        use super::*;
        #[wasm_bindgen(js_name = osuSkinToDir)]
        pub fn skin_to_dir(skin: &crate::osu::OsuSkin, path: &str) -> Result<(), JsError> { map_err!(export_osu_skin(skin, path)) }

        #[wasm_bindgen(js_name = osuIniToDir)]
        pub fn ini_to_dir(skin_ini: &crate::osu::OsuSkinIni, path: &str) -> Result<(), JsError> { map_err!(export_osu_ini(skin_ini, path)) }
    }

    pub mod quaver {
        use super::*;
        #[wasm_bindgen(js_name = quaverSkinToDir)]
        pub fn skin_to_dir(skin: &crate::quaver::QuaSkin, path: &str) -> Result<(), JsError> { map_err!(export_quaver_skin(skin, path)) }

        #[wasm_bindgen(js_name = quaverIniToDir)]
        pub fn ini_to_dir(skin_ini: &crate::quaver::QuaSkinIni, path: &str) -> Result<(), JsError> { map_err!(export_quaver_ini(skin_ini, path)) }
     }

     pub mod fluxis {
        use super::*;
        #[wasm_bindgen(js_name = fluXisSkinToDir)]
        pub fn skin_to_dir(skin: &crate::fluxis::FluXisSkin, path: &str) -> Result<(), JsError> { map_err!(export_fluxis_skin(skin, path)) }

        #[wasm_bindgen(js_name = fluxisLayoutToDir)]
        pub fn layout_to_dir(layout_json: &crate::fluxis::FluXisLayout, path: &str) -> Result<(), JsError> { map_err!(export_fluxis_layout_json(layout_json, path)) }

        #[wasm_bindgen(js_name = fluxisJsonToDir)]
        pub fn json_to_dir(skin_json: &crate::fluxis::SkinJson, path: &str) -> Result<(), JsError> { map_err!(export_fluxis_skin_json(skin_json, path)) }
     }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod import {
    use crate::{importing::native::*, io::texture::TextureStore, sample::SampleStore, StringPattern};
    
    type Res<T> = Result<T, Box<dyn std::error::Error>>;

    pub fn textures_from_dir(path: &str, relative_texture_paths: &[StringPattern]) -> Res<TextureStore> { import_textures_from_dir(path, relative_texture_paths) }
    pub fn all_textures_from_dir(path: &str, load_only: Option<&[StringPattern]>) -> Res<TextureStore> { import_all_textures_from_dir(path, load_only) }
    pub fn samples_from_dir(path: &str, relative_sample_paths: &[StringPattern]) -> Res<SampleStore> { import_samples_from_dir(path, relative_sample_paths) }
    pub fn all_samples_from_dir(path: &str) -> Res<SampleStore> { import_all_samples_from_dir(path) }

    pub mod osu {
        use super::*;
        pub fn skin_from_dir(path: &str, import_all: bool) -> Res<crate::osu::OsuSkin> { import_osu_mania_skin_from_dir(path, import_all) }
        pub fn ini_str_from_dir(path: &str) -> String { read_str_from_path(path) }
    }

    pub mod quaver {
        use super::*;
        pub fn skin_from_dir(path: &str, import_all: bool) -> Res<crate::quaver::QuaSkin> { import_quaver_skin_from_dir(path, import_all) }
        pub fn ini_str_from_dir(path: &str) -> String { read_str_from_path(path) }
    }

    pub mod fluxis {
        use super::*;
        pub fn skin_from_dir(path: &str, import_all: bool) -> Res<crate::fluxis::FluXisSkin> { import_fluxis_skin_from_dir(path, import_all) }
        pub fn json_str_from_dir(path: &str) -> String { read_str_from_path(path) }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "browser"))]
pub mod import {
    use wasm_bindgen::prelude::*;
    use js_sys::{Array, Map};
    use crate::{importing::browser::*, io::texture::TextureStore, sample::SampleStore, utils::wasm::*};

    #[wasm_bindgen(js_name = texturesFromFiles)]
    pub fn textures_from_files(files: Map, relative_texture_paths: Array) -> Result<TextureStore, JsError> {
        let paths = arr_to_strs(relative_texture_paths)?;
        let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
        import_textures_from_files(&js_to_hash(files), &path_refs)
    }

    #[wasm_bindgen(js_name = allTexturesFromFiles)]
    pub fn all_textures_from_files(files: Map) -> Result<TextureStore, JsError> {
        import_all_textures_from_files(&js_to_hash(files))
    }

    #[wasm_bindgen(js_name = samplesFromFiles)]
    pub fn samples_from_files(files: Map, relative_sample_paths: Array) -> Result<SampleStore, JsError> {
        let paths = arr_to_strs(relative_sample_paths)?;
        let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
        import_samples_from_files(&js_to_hash(files), &path_refs)
    }

    #[wasm_bindgen(js_name = allSamplesFromFiles)]
    pub fn all_samples_from_files(files: Map) -> Result<SampleStore, JsError> {
        import_all_samples_from_files(&js_to_hash(files))
    }

    pub mod osu {
        use super::*;
        #[wasm_bindgen(js_name = osuSkinFromFiles)]
        pub fn skin_from_files(files: Map) -> Result<crate::osu::OsuSkin, JsError> {
            import_osu_mania_skin_from_files(&js_to_hash(files))
        }
    }

    pub mod quaver {
        use super::*;
        #[wasm_bindgen(js_name = quaverSkinFromFiles)]
        pub fn skin_from_files(files: Map) -> Result<crate::quaver::QuaSkin, JsError> {
            import_quaver_skin_from_files(&js_to_hash(files))
        }
    }

    pub mod fluxis {
        use super::*;
        #[wasm_bindgen(js_name = fluXisSkinFromFiles)]
        pub fn skin_from_files(files: Map) -> Result<crate::fluxis::FluXisSkin, JsError> {
            import_fluxis_skin_from_files(&js_to_hash(files))
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub mod import {
    use wasm_bindgen::prelude::*;
    use js_sys::Array;
    use crate::{StringPattern, importing::node::*, io::texture::TextureStore, sample::SampleStore, utils::wasm::*};

    macro_rules! map_err { ($e:expr) => { $e.map_err(|e| JsError::new(&e.to_string())) } }

    #[wasm_bindgen(js_name = texturesFromDir)]
    pub fn textures_from_dir(path: &str, relative_texture_paths: Array) -> Result<TextureStore, JsError> {
        let paths = arr_to_strs(relative_texture_paths)?;
        let path_refs: Vec<StringPattern> = paths.iter().map(StringPattern::from).collect();
        map_err!(import_textures_from_dir(path, &path_refs))
    }

    #[wasm_bindgen(js_name = allTexturesFromDir)]
    pub fn all_textures_from_dir(path: &str, load_only: Option<Vec<String>>) -> Result<TextureStore, JsError> {
        let load_patterns: Option<Vec<StringPattern>> = load_only.map(|v| v.into_iter().map(StringPattern::from).collect());
        map_err!(import_all_textures_from_dir(path, load_patterns.as_ref().map(|v| &v[..])))
    }

    #[wasm_bindgen(js_name = samplesFromDir)]
    pub fn samples_from_dir(path: &str, relative_sample_paths: Array) -> Result<SampleStore, JsError> {
        let paths = arr_to_strs(relative_sample_paths)?;
        let path_refs: Vec<StringPattern> = paths.iter().map(StringPattern::from).collect();
        map_err!(import_samples_from_dir(path, &path_refs))
    }

    #[wasm_bindgen(js_name = allSamplesFromDir)]
    pub fn all_samples_from_dir(path: &str) -> Result<SampleStore, JsError> {
        map_err!(import_all_samples_from_dir(path))
    }

    pub mod osu {
        use super::*;
        #[wasm_bindgen(js_name = osuSkinFromDir)]
        pub fn skin_from_dir(path: &str, import_all: Option<bool>) -> Result<crate::osu::OsuSkin, JsError> { map_err!(import_osu_mania_skin_from_dir(path, import_all.unwrap_or(false))) }

        #[wasm_bindgen(js_name = osuIniStrFromDir)]
        pub fn ini_str_from_dir(path: &str) -> String { read_str_from_path(path) }
    }

    pub mod quaver {
        use super::*;
        #[wasm_bindgen(js_name = quaverSkinFromDir)]
        pub fn skin_from_dir(path: &str, import_all: Option<bool>) -> Result<crate::quaver::QuaSkin, JsError> { map_err!(import_quaver_skin_from_dir(path, import_all.unwrap_or(false))) }

        #[wasm_bindgen(js_name = quaverIniStrFromDir)]
        pub fn ini_str_from_dir(path: &str) -> String { read_str_from_path(path) }
    }

    pub mod fluxis {
        use super::*;
        #[wasm_bindgen(js_name = fluXisSkinFromDir)]
        pub fn skin_from_dir(path: &str, import_all: Option<bool>) -> Result<crate::fluxis::FluXisSkin, JsError> { map_err!(import_fluxis_skin_from_dir(path, import_all.unwrap_or(false))) }

        #[wasm_bindgen(js_name = fluxisJsonStrFromDir)]
        pub fn json_str_from_dir(path: &str) -> String { read_str_from_path(path) }
    }
}
