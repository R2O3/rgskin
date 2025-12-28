#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::common::traits::ManiaSkin;
use crate::common::vector::Vector2;
use crate::converting::osu::{from_generic_mania, to_generic_mania};
use crate::osu::Keymode;
use crate::sample::SampleStore;
use crate::skin::generic::GenericManiaSkin;
use crate::skin::osu::SkinIni;
use crate::io::texture::TextureStore;
use crate::traits::SkinConfig;
use crate::utils::osu::OsuDimensions;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct OsuSkin {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub resolution: Vector2<u32>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub skin_ini: SkinIni,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub textures: TextureStore,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub samples: SampleStore
}

impl OsuSkin {
    pub fn new(skin_ini: SkinIni, textures: Option<TextureStore>, samples: Option<SampleStore>) -> Self {
        Self { skin_ini,
            textures: textures.unwrap_or(TextureStore::new()),
            samples: samples.unwrap_or(SampleStore::new()),
            resolution: Vector2::new(OsuDimensions::X.as_u32(), OsuDimensions::Y.as_u32())
        }
    }
}

impl<'a> ManiaSkin<'a> for OsuSkin {
    type Keymode = Keymode;
    type ToParams = ();
    type FromReturn = Self;

    fn to_generic_mania(&self, _params: Self::ToParams) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
        to_generic_mania(self)
    }

    fn from_generic_mania(skin: &GenericManiaSkin) -> Result<Self::FromReturn, Box<dyn std::error::Error>> {
        from_generic_mania(skin)
    }

    fn get_keymode(&self, keymode: u8) -> Option<&Keymode> {
        for k in &self.skin_ini.keymodes {
            if k.keymode == keymode { return Some(k); }
        }
        None
    }

    fn get_required_texture_paths(&self) -> std::collections::HashSet<String> {
        self.skin_ini.get_required_texture_paths()
    }

    fn get_required_sample_paths(&self) -> std::collections::HashSet<String> {
        self.skin_ini.get_required_sample_paths()
    }
}
