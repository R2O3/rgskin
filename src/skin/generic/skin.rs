use merge::Merge;
use rgskin_derive::GetAllTextures;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use std::collections::HashSet;
use crate::common::traits::ManiaSkin;
use crate::common::vector::Vector2;
use crate::generic::gameplay::Gameplay;
use crate::generic::sound::Sounds;
use crate::generic::UI;
use crate::io::texture::TextureStore;
use crate::io::traits::GetAllTextures;
use crate::sample::SampleStore;
use crate::skin::generic::{Keymode, Metadata};
use crate::{Binary, BinaryArcExt, Store, StringPattern, texture, utils};
use crate::extensions::TextureArcExt;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Merge, GetAllTextures)]
pub struct GenericManiaSkin {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub resolution: Vector2<u32>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub sounds: Sounds,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    #[merge(skip)]
    pub metadata: Metadata,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub ui: UI,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub gameplay: Gameplay,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    #[merge(strategy = utils::merge::skin::overwrite_keymode)]
    pub keymodes: Vec<Keymode>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub textures: TextureStore,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub samples: SampleStore
}

impl<'a> ManiaSkin<'a> for GenericManiaSkin {
    type Keymode = Keymode;
    type ToParams = ();
    type FromReturn = Self;

    fn to_generic_mania(&self, _params: Self::ToParams) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
        Ok(self.clone())
    }

    fn from_generic_mania(skin: &GenericManiaSkin) -> Result<Self::FromReturn, Box<dyn std::error::Error>> {
        Ok(skin.clone())
    }

    fn get_keymode(&self, keymode: u8) -> Option<&Keymode> {
        for k in &self.keymodes {
            if k.keymode == keymode { return Some(k); }
        }
        None
    }

    fn get_required_texture_paths(&self) -> Vec<StringPattern> {
        self.textures.iter()
            .into_iter().filter(|(_, texture)| texture.is_loaded())
            .map(|(path, _)| StringPattern::from(path.to_owned()))
            .collect()
    }
    
    fn get_required_sample_paths(&self) -> Vec<StringPattern> {
        self.samples.iter()
            .into_iter().map(|(path, _)| StringPattern::from(path.to_owned()))
            .collect()
    }
}

impl GenericManiaSkin {
    pub fn ensure_textures(&mut self) {
        let all_textures = self.get_all_textures();
        
        for texture_arc in all_textures {
            self.textures.insert_shared(texture_arc);
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl GenericManiaSkin {
    #[wasm_bindgen(js_name = toGenericMania)]
    pub fn to_generic_mania_wasm(&self) -> Result<GenericManiaSkin, JsValue> {
        self.to_generic_mania(())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = fromGenericMania)]
    pub fn from_generic_mania_wasm(skin: &GenericManiaSkin) -> Result<GenericManiaSkin, JsValue> {
        Self::from_generic_mania(skin)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = getKeymode)]
    pub fn get_keymode_wasm(&self, keymode: u8) -> Option<Keymode> {
        self.get_keymode(keymode).cloned()
    }

    #[wasm_bindgen(js_name = getRequiredTexturePaths)]
    pub fn get_required_texture_paths_wasm(&self) -> Vec<String> {
        self.get_required_texture_paths().into_iter().map(|p| p.to_string()).collect()
    }

    #[wasm_bindgen(js_name = getRequiredSamplePaths)]
    pub fn get_required_sample_paths_wasm(&self) -> Vec<String> {
        self.get_required_sample_paths().into_iter().map(|p| p.to_string()).collect()
    }
}
