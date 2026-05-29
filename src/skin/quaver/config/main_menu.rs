use merge::Merge;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::common::color::Rgba;
use crate::utils;
use crate::utils::serde::{add_key_value, add_key_value_if_not_default, parse_key_value_eq};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Merge)]
pub struct MainMenu {
    // Navigation
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub navigation_button_text_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub navigation_quit_button_text_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    
    #[merge(strategy = utils::merge::any::overwrite)]
    pub navigation_button_hovered_alpha: f32,

    // Tip Panel
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub tip_title_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub tip_text_color: Rgba,

    // News Post Panel
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub news_title_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub news_date_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub news_text_color: Rgba,

    // Footer Jukebox
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub jukebox_progress_bar_color: Rgba,

    // Audio Visualizer & Note Visualizer
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub audio_visualizer_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(strategy = utils::merge::any::overwrite)]
    pub audio_visualizer_opacity: f32,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(strategy = utils::merge::any::overwrite)]   
    pub note_visualizer_opacity: f32,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            navigation_button_text_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            navigation_quit_button_text_color: Rgba { red: 249, green: 100, blue: 93, alpha: 255 },
            navigation_button_hovered_alpha: 0.35,
            tip_title_color: Rgba { red: 69, green: 214, blue: 245, alpha: 255 },
            tip_text_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            news_title_color: Rgba { red: 69, green: 214, blue: 245, alpha: 255 },
            news_date_color: Rgba { red: 128, green: 128, blue: 128, alpha: 255 },
            news_text_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            jukebox_progress_bar_color: Rgba { red: 255, green: 222, blue: 124, alpha: 255 },
            audio_visualizer_color: Rgba { red: 9, green: 165, blue: 200, alpha: 255 },
            audio_visualizer_opacity: 0.85,
            note_visualizer_opacity: 0.60,
        }
    }
}

impl MainMenu {
    pub fn from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut menu = Self::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('[') {
                continue;
            }
            let (key_str, value_str) = parse_key_value_eq(line).unwrap_or_default();
            match key_str {
                "NavigationButtonTextColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { menu.navigation_button_text_color = c; }
                }
                "NavigationQuitButtonTextColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { menu.navigation_quit_button_text_color = c; }
                }
                "NavigationButtonHoveredAlpha" => {
                    menu.navigation_button_hovered_alpha = value_str.parse().unwrap_or(0.35);
                }
                "TipTitleColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { menu.tip_title_color = c; }
                }
                "TipTextColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { menu.tip_text_color = c; }
                }
                "NewsTitleColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { menu.news_title_color = c; }
                }
                "NewsDateColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { menu.news_date_color = c; }
                }
                "NewsTextColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { menu.news_text_color = c; }
                }
                "JukeboxProgressBarColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { menu.jukebox_progress_bar_color = c; }
                }
                "AudioVisualizerColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { menu.audio_visualizer_color = c; }
                }
                "AudioVisualizerOpacity" => {
                    menu.audio_visualizer_opacity = value_str.parse().unwrap_or(0.85);
                }
                "NoteVisualizerOpacity" => {
                    menu.note_visualizer_opacity = value_str.parse().unwrap_or(0.60);
                }
                _ => {}
            }
        }

        Ok(menu)
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let default = MainMenu::default();

        // Navigation
        if self.navigation_button_text_color != default.navigation_button_text_color {
            add_key_value(&mut result, "NavigationButtonTextColor", " = ", &self.navigation_button_text_color.to_str(), "\n");
        }
        if self.navigation_quit_button_text_color != default.navigation_quit_button_text_color {
            add_key_value(&mut result, "NavigationQuitButtonTextColor", " = ", &self.navigation_quit_button_text_color.to_str(), "\n");
        }
        add_key_value_if_not_default::<f32>(&mut result, "NavigationButtonHoveredAlpha", " = ", &self.navigation_button_hovered_alpha, &default.navigation_button_hovered_alpha);

        // Tip Panel
        if self.tip_title_color != default.tip_title_color {
            add_key_value(&mut result, "TipTitleColor", " = ", &self.tip_title_color.to_str(), "\n");
        }
        if self.tip_text_color != default.tip_text_color {
            add_key_value(&mut result, "TipTextColor", " = ", &self.tip_text_color.to_str(), "\n");
        }

        // News Post Panel
        if self.news_title_color != default.news_title_color {
            add_key_value(&mut result, "NewsTitleColor", " = ", &self.news_title_color.to_str(), "\n");
        }
        if self.news_date_color != default.news_date_color {
            add_key_value(&mut result, "NewsDateColor", " = ", &self.news_date_color.to_str(), "\n");
        }
        if self.news_text_color != default.news_text_color {
            add_key_value(&mut result, "NewsTextColor", " = ", &self.news_text_color.to_str(), "\n");
        }

        // Footer Jukebox
        if self.jukebox_progress_bar_color != default.jukebox_progress_bar_color {
            add_key_value(&mut result, "JukeboxProgressBarColor", " = ", &self.jukebox_progress_bar_color.to_str(), "\n");
        }

        // Audio / Note Visualizer
        if self.audio_visualizer_color != default.audio_visualizer_color {
            add_key_value(&mut result, "AudioVisualizerColor", " = ", &self.audio_visualizer_color.to_str(), "\n");
        }
        add_key_value_if_not_default::<f32>(&mut result, "AudioVisualizerOpacity", " = ", &self.audio_visualizer_opacity, &default.audio_visualizer_opacity);
        add_key_value_if_not_default::<f32>(&mut result, "NoteVisualizerOpacity", " = ", &self.note_visualizer_opacity, &default.note_visualizer_opacity);

        result
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MainMenu {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromStr))]
    pub fn wasm_from_str(content: &str) -> Result<MainMenu, String> {
        Self::from_str(content).map_err(|e| e.to_string())
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toString))]
    pub fn wasm_to_string(&self) -> String {
        self.to_string()
    }
}
