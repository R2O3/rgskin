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
    
    pub use crate::osu::SkinIni;
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
    use crate::{exporting::native::export_textures, io::texture::TextureStore};

    pub fn textures_to_dir(textures: &TextureStore, path: &str) -> io::Result<()>  {
        export_textures(textures, path)
    }

    pub mod osu {
        use std::io;

        use crate::{exporting::native::{export_osu_ini, export_osu_skin}, osu};

        pub fn skin_to_dir(skin: &osu::OsuSkin, path: &str) -> io::Result<()> {
            export_osu_skin(skin, path)
        }

        pub fn ini_to_dir(skin_ini: &osu::SkinIni, path: &str) -> io::Result<()> {
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
    use std::collections::HashMap;
    use wasm_bindgen::JsError;
    use crate::{exporting::browser::export_textures, io::texture::TextureStore};

    pub fn textures_to_files(textures: &TextureStore) -> Result<HashMap<String, Vec<u8>>, JsError> {
        export_textures(textures)
    }

    pub mod osu {
        use std::collections::HashMap;
        use wasm_bindgen::JsError;
        use crate::{exporting::browser::{export_osu_ini, export_osu_skin}, osu};

        pub fn skin_to_files(skin: &osu::OsuSkin) -> Result<HashMap<String, Vec<u8>>, JsError> {
            export_osu_skin(skin)
        }

        pub fn ini_to_string(skin_ini: &osu::SkinIni) -> String {
            export_osu_ini(skin_ini)
        }
    }

    pub mod fluxis {
        use std::collections::HashMap;
        use wasm_bindgen::JsError;
        use crate::{exporting::browser::{export_fluxis_layout_json, export_fluxis_skin, export_fluxis_skin_json}, fluxis};

        pub fn skin_to_files(skin: &fluxis::FluXisSkin) -> Result<HashMap<String, Vec<u8>>, JsError> {
            export_fluxis_skin(skin)
        }

        pub fn layout_to_string(layout_json: &fluxis::FluXisLayout) -> Result<String, JsError> {
            export_fluxis_layout_json(layout_json)
        }

        pub fn json_to_string(skin_json: &fluxis::SkinJson) -> String {
            export_fluxis_skin_json(skin_json)
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub mod export {
    use std::io;
    use crate::{exporting::native::export_textures, io::texture::TextureStore};

    pub fn textures_to_dir(textures: &TextureStore, path: &str) -> io::Result<()>  {
        export_textures(textures, path)
    }

    pub mod osu {
        use std::io;

        use crate::{exporting::native::{export_osu_ini, export_osu_skin}, osu};

        pub fn skin_to_dir(skin: &osu::OsuSkin, path: &str) -> io::Result<()> {
            export_osu_skin(skin, path)
        }

        pub fn ini_to_dir(skin_ini: &osu::SkinIni, path: &str) -> io::Result<()> {
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

#[cfg(not(target_arch = "wasm32"))]
pub mod import {
    use crate::{importing::native::{import_all_textures_from_dir, import_textures_from_dir}, io::texture::TextureStore};

    pub fn textures_from_dir(path: &str, relative_texture_paths: &[&str]) -> Result<TextureStore, Box<dyn std::error::Error>>  {
        import_textures_from_dir(path, relative_texture_paths)
    }

    pub fn all_textures_from_dir(path: &str) -> Result<TextureStore, Box<dyn std::error::Error>>  {
        import_all_textures_from_dir(path)
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
    use wasm_bindgen::JsError;
    use crate::{importing::browser::{import_all_textures_from_files, import_textures_from_files}, io::texture::TextureStore};

    pub fn textures_from_files(files: &HashMap<String, Vec<u8>>, relative_texture_paths: &[&str]) -> Result<TextureStore, JsError>  {
        import_textures_from_files(files, relative_texture_paths)
    }

    pub fn all_textures_from_files(files: &HashMap<String, Vec<u8>>) -> Result<TextureStore, JsError>  {
        import_all_textures_from_files(files)
    }

    pub mod osu {
        use std::collections::HashMap;
        use wasm_bindgen::JsError;
        use crate::{importing::browser::import_osu_mania_skin_from_files, OsuSkin};

        pub fn skin_from_files(files: &HashMap<String, Vec<u8>>) -> Result<OsuSkin, JsError> {
            import_osu_mania_skin_from_files(files)
        }
    }

    pub mod fluxis {
        use std::collections::HashMap;
        use wasm_bindgen::JsError;
        use crate::{importing::browser::import_fluxis_skin_from_files, fluxis::FluXisSkin};

        pub fn skin_from_files(files: &HashMap<String, Vec<u8>>) -> Result<FluXisSkin, JsError> {
            import_fluxis_skin_from_files(files)
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub mod import {
    use wasm_bindgen::JsError;
    use crate::{importing::native::{import_all_textures_from_dir, import_textures_from_dir}, io::texture::TextureStore};

   pub fn textures_from_dir(path: &str, relative_texture_paths: &[&str]) -> Result<TextureStore, JsError>  {
        import_textures_from_dir(path, relative_texture_paths).map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn all_textures_from_dir(path: &str) -> Result<TextureStore, JsError>  {
        import_all_textures_from_dir(path).map_err(|e| JsError::new(&e.to_string()))
    }

    pub mod osu {
        use wasm_bindgen::JsError;
        use crate::{importing::native::{read_str_from_path, import_osu_mania_skin_from_dir}, OsuSkin};

        pub fn skin_from_dir(path: &str) -> Result<OsuSkin, JsError> {
            import_osu_mania_skin_from_dir(path).map_err(|e| JsError::new(&e.to_string()))
        }

        pub fn ini_str_from_dir(path: &str) -> String {
            read_str_from_path(path)
        }
    }

    pub mod fluxis {
        use wasm_bindgen::JsError;
        use crate::{importing::native::{read_str_from_path, import_fluxis_skin_from_dir}, fluxis::FluXisSkin};

        pub fn skin_from_dir(path: &str) -> Result<FluXisSkin, JsError> {
            import_fluxis_skin_from_dir(path).map_err(|e| JsError::new(&e.to_string()))
        }

        pub fn json_str_from_dir(path: &str) -> String {
            read_str_from_path(path)
        }
    }
}
