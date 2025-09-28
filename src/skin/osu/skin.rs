use crate::converting::osu::{from_generic_mania, to_generic_mania};
use crate::skin::generic::GenericManiaSkin;
use crate::skin::osu::SkinIni;
use crate::io::TextureStore;

pub struct OsuSkin {
    pub skin_ini: SkinIni,
    pub textures: TextureStore
}

impl OsuSkin {
    pub fn new(skin_ini: SkinIni, textures: Option<TextureStore>) -> Self {
        Self { skin_ini, textures: textures.unwrap_or(TextureStore::new()) }
    }

    pub fn to_generic_mania(self) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
        to_generic_mania(self)
    }

    pub fn from_generic_mania(skin: GenericManiaSkin) -> Result<OsuSkin, Box<dyn std::error::Error>> {
        from_generic_mania(skin)
    }
}