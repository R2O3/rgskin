use merge::Merge;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::utils;
use crate::utils::serde::{add_key_value, add_key_value_if_not_default, parse_key_value_eq};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq)]
pub enum ResultsBackgroundType {
    Header,
    Background,
    None,
}

impl Default for ResultsBackgroundType {
    fn default() -> Self {
        Self::Header
    }
}

impl ResultsBackgroundType {
    pub fn from_str(s: &str) -> Self {
        match s.trim() {
            "Background" => Self::Background,
            "None" => Self::None,
            _ => Self::Header,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Header => "Header",
            Self::Background => "Background",
            Self::None => "None",
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Merge)]
pub struct Results {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(strategy = utils::merge::any::overwrite)]
    pub results_background_type: ResultsBackgroundType,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(strategy = utils::merge::any::overwrite)]
    pub results_background_filter_alpha: f32,
}

impl Default for Results {
    fn default() -> Self {
        Self {
            results_background_type: ResultsBackgroundType::default(),
            results_background_filter_alpha: 1.0,
        }
    }
}

impl Results {
    pub fn from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut results = Self::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('[') {
                continue;
            }
            let (key_str, value_str) = parse_key_value_eq(line).unwrap_or_default();
            match key_str {
                "ResultsBackgroundType" => {
                    results.results_background_type = ResultsBackgroundType::from_str(value_str);
                }
                "ResultsBackgroundFilterAlpha" => {
                    results.results_background_filter_alpha = value_str.parse().unwrap_or(1.0);
                }
                _ => {}
            }
        }

        Ok(results)
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let default = Results::default();

        if self.results_background_type != default.results_background_type {
            add_key_value(&mut result, "ResultsBackgroundType", " = ", self.results_background_type.to_str(), "\n");
        }
        add_key_value_if_not_default::<f32>(
            &mut result,
            "ResultsBackgroundFilterAlpha",
            " = ",
            &self.results_background_filter_alpha,
            &default.results_background_filter_alpha,
        );

        result
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Results {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromStr))]
    pub fn wasm_from_str(content: &str) -> Result<Results, String> {
        Self::from_str(content).map_err(|e| e.to_string())
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toString))]
    pub fn wasm_to_string(&self) -> String {
        self.to_string()
    }
}
