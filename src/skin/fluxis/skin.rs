#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::common::traits::ManiaSkin;
use crate::common::vector::Vector2;
use crate::converting::fluxis::{from_generic_mania, to_generic_mania};
use crate::fluxis::skin_json::Keymode;
use crate::sample::SampleStore;
use crate::skin::generic::GenericManiaSkin;
use crate::skin::fluxis::{FluXisLayout, SkinJson};
use crate::io::texture::TextureStore;
use crate::traits::SkinConfig;
use crate::utils::fluxis::FluXisDimensions;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone)]
pub struct FluXisSkin {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub resolution: Vector2<u32>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub skin_json: SkinJson,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub textures: TextureStore,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub samples: SampleStore
}

impl FluXisSkin {
    pub fn new(skin_json: SkinJson, textures: Option<TextureStore>, samples: Option<SampleStore>) -> Self {
        Self { skin_json,
            textures: textures.unwrap_or(TextureStore::new()),
            samples: samples.unwrap_or(SampleStore::new()),
            resolution: Vector2::new(FluXisDimensions::X.as_u32(), FluXisDimensions::Y.as_u32())
        }
    }
}

impl<'a> ManiaSkin<'a> for FluXisSkin {
    type Keymode = Keymode;
    type ToParams = Option<&'a FluXisLayout>;
    type FromReturn = (FluXisSkin, FluXisLayout);

    fn to_generic_mania(&self, params: Self::ToParams) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
        to_generic_mania(self, params)
    }

    fn from_generic_mania(skin: &GenericManiaSkin) -> Result<Self::FromReturn, Box<dyn std::error::Error>> {
        from_generic_mania(skin)
    }

    fn get_keymode(&self, keymode: u8) -> Option<&Keymode> {
        for k in &self.skin_json.keymodes {
            if k.keymode == keymode { return Some(k); }
        }
        None
    }

    fn get_required_texture_paths(&self) -> std::collections::HashSet<String> {
        self.skin_json.get_required_texture_paths()
    }
    
    fn get_required_sample_paths(&self) -> std::collections::HashSet<String> {
        self.skin_json.get_required_sample_paths()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl FluXisSkin {
    #[wasm_bindgen(js_name = toGenericMania)]
    pub fn to_generic_mania_wasm(&self) -> Result<GenericManiaSkin, JsValue> {
        self.to_generic_mania(None)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = toGenericManiaWithLayout)]
    pub fn to_generic_mania_with_layout_wasm(&self, layout: &FluXisLayout) -> Result<GenericManiaSkin, JsValue> {
        self.to_generic_mania(Some(layout))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = fromGenericMania)]
    pub fn from_generic_mania_wasm(skin: &GenericManiaSkin) -> Result<FluXisSkinWithLayout, JsValue> {
        Self::from_generic_mania(skin)
            .map(|(skin, layout)| FluXisSkinWithLayout { skin, layout })
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct FluXisSkinWithLayout {
    #[wasm_bindgen(getter_with_clone)]
    pub skin: FluXisSkin,
    #[wasm_bindgen(getter_with_clone)]
    pub layout: FluXisLayout,
}
