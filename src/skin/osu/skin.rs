#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::common::traits::ManiaSkin;
use crate::common::vector::Vector2;
use crate::converting::osu::{from_generic_mania, to_generic_mania};
use crate::osu::Keymode;
use crate::sample::SampleStore;
use crate::skin::generic::GenericManiaSkin;
use crate::skin::osu::OsuSkinIni;
use crate::io::texture::TextureStore;
use crate::traits::SkinConfig;
use crate::utils::osu::OsuDimensions;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone)]
pub struct OsuSkin {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub resolution: Vector2<u32>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub skin_ini: OsuSkinIni,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub textures: TextureStore,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub samples: SampleStore
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl OsuSkin {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(skin_ini: OsuSkinIni, textures: Option<TextureStore>, samples: Option<SampleStore>) -> Self {
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl OsuSkin {
    #[wasm_bindgen(js_name = toGenericMania)]
    pub fn to_generic_mania_wasm(&self) -> Result<GenericManiaSkin, JsValue> {
        self.to_generic_mania(())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = fromGenericMania)]
    pub fn from_generic_mania_wasm(skin: &GenericManiaSkin) -> Result<OsuSkin, JsValue> {
        Self::from_generic_mania(skin)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = getKeymode)]
    pub fn get_keymode_wasm(&self, keymode: u8) -> Option<Keymode> {
        self.get_keymode(keymode).cloned()
    }

    #[wasm_bindgen(js_name = getRequiredTexturePaths)]
    pub fn get_required_texture_paths_wasm(&self) -> Vec<String> {
        self.get_required_texture_paths().into_iter().collect()
    }

    #[wasm_bindgen(js_name = getRequiredSamplePaths)]
    pub fn get_required_sample_paths_wasm(&self) -> Vec<String> {
        self.get_required_sample_paths().into_iter().collect()
    }
}
