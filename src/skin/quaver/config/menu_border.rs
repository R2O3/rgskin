use merge::Merge;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::common::color::Rgba;
use crate::utils::serde::{add_key_value, parse_key_value_eq};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Merge)]
pub struct MenuBorder {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub background_line_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub foreground_line_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub button_text_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub button_text_hovered_color: Rgba,
}

impl Default for MenuBorder {
    fn default() -> Self {
        Self {
            background_line_color: Rgba { red: 9, green: 165, blue: 200, alpha: 255 },
            foreground_line_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            button_text_color: Rgba { red: 9, green: 165, blue: 200, alpha: 255 },
            button_text_hovered_color: Rgba { red: 81, green: 197, blue: 249, alpha: 255 },
        }
    }
}

impl MenuBorder {
    pub fn from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut border = Self::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('[') {
                continue;
            }
            let (key_str, value_str) = parse_key_value_eq(line).unwrap_or_default();
            match key_str {
                "BackgroundLineColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { border.background_line_color = c; }
                }
                "ForegroundLineColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { border.foreground_line_color = c; }
                }
                "ButtonTextColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { border.button_text_color = c; }
                }
                "ButtonTextHoveredColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { border.button_text_hovered_color = c; }
                }
                _ => {}
            }
        }

        Ok(border)
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let default = MenuBorder::default();

        if self.background_line_color != default.background_line_color {
            add_key_value(&mut result, "BackgroundLineColor", " = ", &self.background_line_color.to_str(), "\n");
        }
        if self.foreground_line_color != default.foreground_line_color {
            add_key_value(&mut result, "ForegroundLineColor", " = ", &self.foreground_line_color.to_str(), "\n");
        }
        if self.button_text_color != default.button_text_color {
            add_key_value(&mut result, "ButtonTextColor", " = ", &self.button_text_color.to_str(), "\n");
        }
        if self.button_text_hovered_color != default.button_text_hovered_color {
            add_key_value(&mut result, "ButtonTextHoveredColor", " = ", &self.button_text_hovered_color.to_str(), "\n");
        }

        result
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MenuBorder {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromStr))]
    pub fn wasm_from_str(content: &str) -> Result<MenuBorder, String> {
        Self::from_str(content).map_err(|e| e.to_string())
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toString))]
    pub fn wasm_to_string(&self) -> String {
        self.to_string()
    }
}
