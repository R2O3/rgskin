#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

use crate::define_keymode;

define_keymode!(
    (receptor_images, "Receptor", "", "-up"),
    (receptor_images_down, "Receptor", "", "-down"),
    (normal_note_images, "HitObjects", "Note", ""),
    (long_note_head_images, "HitObjects", "LongNoteStart", ""),
    (long_note_body_images, "HitObjects", "LongNoteBody", ""),
    (long_note_tail_images, "HitObjects", "LongNoteEnd", ""),
    (tick_images, "HitObjects", "Tick", ""),
    (tick_images_small, "HitObjects", "Tick", "-small"),
);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Keymode {
    #[serde(skip)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub keymode: u8,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub column_width: u32,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub hit_position: i32,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub tint_notes: bool,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub tint_lns: bool,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub tint_receptors: bool,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub colors: Vec<String>,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub receptors_first: bool,
    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub receptor_offset: i32,
    
    #[serde(skip)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub receptor_images: Vec<String>,
    
    #[serde(skip)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub receptor_images_down: Vec<String>,
    
    #[serde(skip)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub normal_note_images: Vec<String>,
    
    #[serde(skip)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub long_note_head_images: Vec<String>,
    
    #[serde(skip)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub long_note_body_images: Vec<String>,
    
    #[serde(skip)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub long_note_tail_images: Vec<String>,
    
    #[serde(skip)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub tick_images: Vec<String>,
    
    #[serde(skip)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub tick_images_small: Vec<String>,
}

impl Default for Keymode {
    fn default() -> Self {
        Self {
            keymode: 0,
            column_width: 150,
            hit_position: 35,
            tint_notes: false,
            tint_lns: false,
            tint_receptors: false,
            colors: Vec::new(),
            receptors_first: true,
            receptor_offset: 0,
            receptor_images: Vec::new(),
            receptor_images_down: Vec::new(),
            normal_note_images: Vec::new(),
            long_note_head_images: Vec::new(),
            long_note_body_images: Vec::new(),
            long_note_tail_images: Vec::new(),
            tick_images: Vec::new(),
            tick_images_small: Vec::new(),
        }
    }
}
