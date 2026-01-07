#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use std::collections::HashSet;
use std::str::FromStr;
use crate::common::traits::{ManiaSkinConfig, SkinConfig};
use crate::osu::static_assets;
use crate::skin::osu::keymode::Keymode;
use crate::skin::osu::General;
use crate::ini::from_ini;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Default)]
pub struct OsuSkinIni {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub general: General,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))] // TODO: maybe not a good idea to use getter_with_clone
    pub keymodes: Vec<Keymode>
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl OsuSkinIni {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        OsuSkinIni::default()
    }

    #[wasm_bindgen(js_name = fromStr)]
    pub fn from_str_wasm(json_str: &str) -> Result<Self, JsError> {
        Self::from_str(json_str).map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string_wasm(&self) -> String {
        self.to_string()
    }

    #[wasm_bindgen(js_name = "getRequiredTexturePaths")]
    pub fn wasm_get_required_texture_paths(&self) -> Vec<String> {
        self.get_required_texture_paths().into_iter().collect()
    }

    #[wasm_bindgen(js_name = "getRequiredSamplePaths")]
    pub fn wasm_get_required_sample_paths(&self) -> Vec<String> {
        self.get_required_sample_paths().into_iter().collect()
    }

    #[wasm_bindgen(js_name = "getKeymode")]
    pub fn wasm_get_keymode(&self, keymode: u8) -> Option<Keymode> {
        self.get_keymode(keymode).cloned()
    }
}

impl ToString for OsuSkinIni {
    fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str("[General]\n");
        result.push_str(&self.general.to_string());
        result.push('\n');

        for keymode in &self.keymodes {
            result.push_str("[Mania]\n");
            result.push_str(&keymode.to_str());
            result.push('\n');
        }

        result
    }
}

impl FromStr for OsuSkinIni {
    type Err = Box<dyn std::error::Error>;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut general = General::default();
        let mut keymodes = Vec::new();

        from_ini(str, |section, content| {
            match section {
                "General" => general = General::from_str(content)?,
                "Mania" => keymodes.push(Keymode::from_str(content)?),
                _ => { },
            }
            Ok(())
        })?;

        Ok(OsuSkinIni { general, keymodes })
    }
}

impl SkinConfig for OsuSkinIni {
    fn get_required_texture_paths(&self) -> HashSet<String> {
        let mut result = HashSet::new();

        for keymode in &self.keymodes {
            result.extend(keymode.get_texture_paths());
        }

        result.extend(static_assets::Mania::iter_mapped(|t| t.to_string()));
        result.extend(static_assets::Interface::iter_mapped(|t| t.to_string()));

        result
    }

    fn get_required_sample_paths(&self) -> HashSet<String> {
        let mut result: HashSet<String> = HashSet::new();
        result.extend(static_assets::Samples::iter_mapped(|s| s.to_string()));
        result
    }
}

impl ManiaSkinConfig for OsuSkinIni {
    type Keymode = Keymode;

    fn get_keymode(&self, keymode: u8) -> Option<&Keymode> {
        for k in &self.keymodes {
            if k.keymode == keymode { return Some(k); }
        }
        None
    }
}