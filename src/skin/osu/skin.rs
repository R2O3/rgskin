use crate::common::traits::ManiaSkin;
use crate::converting::osu::{from_generic_mania, to_generic_mania};
use crate::osu::Keymode;
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
}

impl ManiaSkin for OsuSkin {
    type Keymode = Keymode;
    type ToParams = ();
    type FromReturn = Self;

    fn to_generic_mania(self, _params: Self::ToParams) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
        to_generic_mania(self)
    }

    fn from_generic_mania(skin: GenericManiaSkin) -> Result<Self::FromReturn, Box<dyn std::error::Error>> {
        from_generic_mania(skin)
    }

    fn get_keymode(&self, keymode: u8) -> Option<&Keymode> {
        for k in &self.skin_ini.keymodes {
            if k.keymode == keymode { return Some(k); }
        }
        None
    }
}
