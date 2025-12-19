pub(crate) mod skin;
pub(crate) mod converting;
pub(crate) mod exporting;
pub(crate) mod importing;
pub(crate) mod io;
pub(crate) mod common;

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

pub use extensions::{TextureArcExt, BinaryArcExt};
pub use io::{Binary, BinaryState, Store, texture};
pub use common::traits;

pub mod prelude {
    pub use crate::OsuSkin;
    pub use crate::GenericManiaSkin;
    pub use crate::FluXisSkin;
    
    pub use crate::osu::SkinIni;
    pub use crate::fluxis::{SkinJson, FluXisLayout};
    
    pub use crate::io::{Binary, BinaryState, Store};
    pub use crate::io::texture::{TextureStore, Texture};
    
    pub use crate::common::traits::*;
    pub use crate::extensions::*;
    
    pub use crate::load;
    pub use crate::export;
    pub use crate::import;
}

pub mod assets {
    use std::collections::HashSet;
    use crate::{common::traits::SkinConfig, osu};

    pub fn get_mania_texture_paths(skin_ini: &osu::SkinIni) -> HashSet<String> {
        skin_ini.get_dynamic_texture_paths()
    }
}

pub mod load {
    pub mod osu {
        use std::str::FromStr;

        use crate::{converting::osu::{from_generic_mania, to_generic_mania}, io::texture::TextureStore, osu, skin::generic};

        pub fn skin_ini(str: &str) -> Result<osu::SkinIni, Box<dyn std::error::Error>> {
            osu::SkinIni::from_str(str)
        }

        pub fn from_ini(skin_ini: osu::SkinIni, assets: Option<TextureStore>) -> osu::OsuSkin {
            osu::OsuSkin::new(skin_ini, assets)
        }

        pub fn to_generic(skin: &osu::OsuSkin) -> Result<generic::GenericManiaSkin, Box<dyn std::error::Error>> {
            to_generic_mania(skin)
        }

        pub fn from_generic(skin: &generic::GenericManiaSkin) -> Result<osu::OsuSkin, Box<dyn std::error::Error>> {
            from_generic_mania(skin)
        }
    }

    pub mod fluxis {
        use std::str::FromStr;

        use crate::{converting::fluxis::{from_generic_mania, to_generic_mania}, io::texture::TextureStore, fluxis, skin::generic};

        pub fn skin_json(str: &str) -> Result<fluxis::SkinJson, Box<dyn std::error::Error>> {
            fluxis::SkinJson::from_str(str)
        }

        pub fn layout_json(str: &str) -> Result<fluxis::FluXisLayout, Box<dyn std::error::Error>> {
            fluxis::FluXisLayout::from_str(str)
        }

        pub fn from_json(skin_ini: fluxis::SkinJson, assets: Option<TextureStore>) -> fluxis::FluXisSkin {
            fluxis::FluXisSkin::new(skin_ini, assets)
        }

        pub fn to_generic(skin: &fluxis::FluXisSkin, layout: Option<&fluxis::FluXisLayout>) -> Result<generic::GenericManiaSkin, Box<dyn std::error::Error>> {
            to_generic_mania(skin, layout)
        }

        pub fn from_generic(skin: &generic::GenericManiaSkin) -> Result<(fluxis::FluXisSkin, fluxis::FluXisLayout), Box<dyn std::error::Error>> {
            from_generic_mania(skin)
        }
    }
}

pub mod export {
    use std::io;
    use crate::{exporting::native::export_textures, io::texture::TextureStore};

    pub fn textures_to_dir(textures: &TextureStore, path: &str) -> io::Result<()>  {
        export_textures(textures, path)
    }

    pub mod osu {
        use std::io;

        use crate::{exporting::native::{export_osu_ini, export_osu_skin}, osu, io::texture::TextureStore};

        pub fn skin_to_dir(skin_ini: &osu::SkinIni, textures: Option<&TextureStore>, path: &str) -> io::Result<()> {
            export_osu_skin(skin_ini, textures, path)
        }

        pub fn ini_to_dir(skin_ini: &osu::SkinIni, path: &str) -> io::Result<()> {
            export_osu_ini(skin_ini, path)
        }
    }

    pub mod fluxis {
        use std::io;
        use crate::{exporting::native::{export_fluxis_layout_json, export_fluxis_skin, export_fluxis_skin_json}, fluxis, io::texture::TextureStore};

        pub fn skin_to_dir(skin_json: &fluxis::SkinJson, textures: Option<&TextureStore>, path: &str) -> io::Result<()> {
            export_fluxis_skin(skin_json, textures, path)
        }

        pub fn layout_to_dir(layout_json: &fluxis::FluXisLayout, path: &str) -> io::Result<()> {
            export_fluxis_layout_json(layout_json, path)
        }

        pub fn json_to_dir(skin_json: &fluxis::SkinJson, path: &str) -> io::Result<()> {
            export_fluxis_skin_json(skin_json, path)
        }
    }
}

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