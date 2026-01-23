use merge::Merge;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{derive_merge_for_all, utils::serde::{
    add_key_value, parse_bool, parse_key_value, parse_u16_list, serialize_bool, serialize_u16_slice
}};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
derive_merge_for_all! {
    strategy = crate::utils::merge::any::overwrite;
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
        pub animation_framerate: i16,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub allow_slider_ball_tint: bool,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub combo_burst_random: bool,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub cursor_centre: bool,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub cursor_expand: bool,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub cursor_rotate: bool,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub cursor_trail_rotate: bool,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub custom_combo_burst_sounds: Vec<u16>,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub hit_circle_overlay_above_number: bool,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub layered_hit_sounds: bool,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub slider_ball_flip: bool,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub spinner_fade_playfield: bool,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub spinner_frequency_modulate: bool,
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub spinner_no_blink: bool,
    }
}

impl Default for General {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            author: String::new(),
            version: "latest".to_string(),
            animation_framerate: -1,
            allow_slider_ball_tint: false,
            combo_burst_random: false,
            cursor_centre: true,
            cursor_expand: true,
            cursor_rotate: true,
            cursor_trail_rotate: true,
            custom_combo_burst_sounds: Vec::new(),
            hit_circle_overlay_above_number: true,
            layered_hit_sounds: true,
            slider_ball_flip: true,
            spinner_fade_playfield: false,
            spinner_frequency_modulate: true,
            spinner_no_blink: false,
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

            let (key_str, value_str) = parse_key_value(line).unwrap_or_default();

            match key_str {
                "Name" => general.name = value_str.to_string(),
                "Author" => general.author = value_str.to_string(),
                "Version" => general.version = value_str.to_string(),
                "AnimationFramerate" => general.animation_framerate = value_str.parse()?,
                "AllowSliderBallTint" => general.allow_slider_ball_tint = parse_bool(value_str),
                "ComboBurstRandom" => general.combo_burst_random = parse_bool(value_str),
                "CursorCentre" => general.cursor_centre = parse_bool(value_str),
                "CursorExpand" => general.cursor_expand = parse_bool(value_str),
                "CursorRotate" => general.cursor_rotate = parse_bool(value_str),
                "CursorTrailRotate" => general.cursor_trail_rotate = parse_bool(value_str),
                "CustomComboBurstSounds" => general.custom_combo_burst_sounds = parse_u16_list(value_str),
                "HitCircleOverlayAboveNumber" | "HitCircleOverlayAboveNumer" => {
                    general.hit_circle_overlay_above_number = parse_bool(value_str)
                },
                "LayeredHitSounds" => general.layered_hit_sounds = parse_bool(value_str),
                "SliderBallFlip" => general.slider_ball_flip = parse_bool(value_str),
                "SpinnerFadePlayfield" => general.spinner_fade_playfield = parse_bool(value_str),
                "SpinnerFrequencyModulate" => general.spinner_frequency_modulate = parse_bool(value_str),
                "SpinnerNoBlink" => general.spinner_no_blink = parse_bool(value_str),
                _ => { }
            }
        }

        Ok(general)
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        add_key_value(&mut result, "Name", ": ", &self.name, "\n");
        add_key_value(&mut result, "Author", ": ", &self.author, "\n");
        add_key_value(&mut result, "Version", ": ", &self.version, "\n");
        add_key_value(&mut result, "AnimationFramerate", ": ", &self.animation_framerate.to_string(), "\n");
        add_key_value(&mut result, "AllowSliderBallTint", ": ", &serialize_bool(self.allow_slider_ball_tint).to_string(), "\n");
        add_key_value(&mut result, "ComboBurstRandom", ": ", &serialize_bool(self.combo_burst_random).to_string(), "\n");
        add_key_value(&mut result, "CursorCentre", ": ", &serialize_bool(self.cursor_centre).to_string(), "\n");
        add_key_value(&mut result, "CursorExpand", ": ", &serialize_bool(self.cursor_expand).to_string(), "\n");
        add_key_value(&mut result, "CursorRotate", ": ", &serialize_bool(self.cursor_rotate).to_string(), "\n");
        add_key_value(&mut result, "CursorTrailRotate", ": ", &serialize_bool(self.cursor_trail_rotate).to_string(), "\n");
        
        if !self.custom_combo_burst_sounds.is_empty() {
            add_key_value(&mut result, "CustomComboBurstSounds", ": ", &serialize_u16_slice(&self.custom_combo_burst_sounds), "\n");
        }
        
        add_key_value(&mut result, "HitCircleOverlayAboveNumber", ": ", &serialize_bool(self.hit_circle_overlay_above_number).to_string(), "\n");
        add_key_value(&mut result, "LayeredHitSounds", ": ", &serialize_bool(self.layered_hit_sounds).to_string(), "\n");
        add_key_value(&mut result, "SliderBallFlip", ": ", &serialize_bool(self.slider_ball_flip).to_string(), "\n");
        add_key_value(&mut result, "SpinnerFadePlayfield", ": ", &serialize_bool(self.spinner_fade_playfield).to_string(), "\n");
        add_key_value(&mut result, "SpinnerFrequencyModulate", ": ", &serialize_bool(self.spinner_frequency_modulate).to_string(), "\n");
        add_key_value(&mut result, "SpinnerNoBlink", ": ", &serialize_bool(self.spinner_no_blink).to_string(), "\n");
        
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