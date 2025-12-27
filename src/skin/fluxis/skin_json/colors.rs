use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::common::color::Rgba;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct JudgementColors {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub flawless: Rgba,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub perfect: Rgba,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub great: Rgba,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub alright: Rgba,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub okay: Rgba,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub miss: Rgba,
}

impl Default for JudgementColors {
    fn default() -> Self {
        Self {
            flawless: Rgba::from_hex("#00C3FF").unwrap(),
            perfect: Rgba::from_hex("#22FFB5").unwrap(),
            great: Rgba::from_hex("#4BFF3B").unwrap(),
            alright: Rgba::from_hex("#FFF12B").unwrap(),
            okay: Rgba::from_hex("#F7AD40").unwrap(),
            miss: Rgba::from_hex("#FF5555").unwrap(),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapColors {
    #[serde(rename = "1/3")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub snap_1_3: Rgba,
    
    #[serde(rename = "1/4")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub snap_1_4: Rgba,
    
    #[serde(rename = "1/6")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub snap_1_6: Rgba,
    
    #[serde(rename = "1/8")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub snap_1_8: Rgba,
    
    #[serde(rename = "1/12")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub snap_1_12: Rgba,
    
    #[serde(rename = "1/16")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub snap_1_16: Rgba,
    
    #[serde(rename = "1/24")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub snap_1_24: Rgba,
    
    #[serde(rename = "1/48")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub snap_1_48: Rgba,
}

impl Default for SnapColors {
    fn default() -> Self {
        Self {
            snap_1_3: Rgba::from_hex("#FF5555").unwrap(),
            snap_1_4: Rgba::from_hex("#558EFF").unwrap(),
            snap_1_6: Rgba::from_hex("#8EFF55").unwrap(),
            snap_1_8: Rgba::from_hex("#FFE355").unwrap(),
            snap_1_12: Rgba::from_hex("#C655FF").unwrap(),
            snap_1_16: Rgba::from_hex("#55FFAA").unwrap(),
            snap_1_24: Rgba::from_hex("#FF55AA").unwrap(),
            snap_1_48: Rgba::from_hex("#BFBFBF").unwrap(),
        }
    }
}
