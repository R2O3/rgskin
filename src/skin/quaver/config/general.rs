use merge::Merge;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::utils;
use crate::utils::serde::{add_key_value, parse_bool, parse_key_value_eq, serialize_bool};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Merge)]
pub struct General {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(skip)]
    pub name: String,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(skip)]
    pub author: String,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(skip)]
    pub version: String,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(strategy = utils::merge::any::overwrite)]
    pub center_cursor: bool,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(strategy = utils::merge::any::overwrite)]
    pub use_skin_backgrounds: bool,
}

impl Default for General {
    fn default() -> Self {
        Self {
            name: String::new(),
            author: String::new(),
            version: String::new(),
            center_cursor: false,
            use_skin_backgrounds: false,
        }
    }
}

impl General {
    pub fn from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut general = Self::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('[') {
                continue;
            }
            let (key_str, value_str) = parse_key_value_eq(line).unwrap_or_default();
            match key_str {
                "Name" => general.name = value_str.to_string(),
                "Author" => general.author = value_str.to_string(),
                "Version" => general.version = value_str.to_string(),
                "CenterCursor" => general.center_cursor = parse_bool(value_str),
                "UseSkinBackgrounds" => general.use_skin_backgrounds = parse_bool(value_str),
                _ => {}
            }
        }

        Ok(general)
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        add_key_value(&mut result, "Name", " = ", &self.name, "\n");
        add_key_value(&mut result, "Author", " = ", &self.author, "\n");
        add_key_value(&mut result, "Version", " = ", &self.version, "\n");
        add_key_value(&mut result, "CenterCursor", " = ", &serialize_bool(self.center_cursor).to_string(), "\n");
        add_key_value(&mut result, "UseSkinBackgrounds", " = ", &serialize_bool(self.use_skin_backgrounds).to_string(), "\n");
        result
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl General {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromStr))]
    pub fn wasm_from_str(content: &str) -> Result<General, String> {
        Self::from_str(content).map_err(|e| e.to_string())
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toString))]
    pub fn wasm_to_string(&self) -> String {
        self.to_string()
    }
}
