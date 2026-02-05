#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{define_keymode, traits::{LaneFallback, KeymodeInvariant}};

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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = FluXisKeymode))]
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

impl KeymodeInvariant for Keymode {
    fn get_keymode(&self) -> u8 { self.keymode }

    fn get_receptors(&self) -> Vec<String> { self.receptor_images.clone() }
    fn get_receptors_down(&self) -> Vec<String> { self.receptor_images_down.clone() }

    fn get_normal_notes(&self) -> Vec<String> { self.normal_note_images.clone() }

    fn get_long_note_heads(&self) -> Vec<String> { self.long_note_head_images.clone() }
    fn get_long_note_bodies(&self) -> Vec<String> { self.long_note_body_images.clone() }
    fn get_long_note_tails(&self) -> Vec<String> { self.long_note_tail_images.clone() }

    fn primary_fallback(&self, _lane: usize) -> LaneFallback {
        let keymode = self.get_keymode();

        LaneFallback {
            receptor: format!("receptor/{}k-{}-up", keymode, _lane),
            receptor_down: format!("receptor/{}k-{}-down", keymode, _lane),
            normal_note: format!("hitobjects/note/{}k-{}", keymode, _lane),
            long_note_head: format!("hitobjects/longnotestart/{}k-{}", keymode, _lane),
            long_note_body: format!("hitobjects/longnotebody/{}k-{}", keymode, _lane),
            long_note_tail: format!("hitobjects/longnoteend/{}k-{}", keymode, _lane),
        }
    }
    fn secondary_fallback(&self, _lane: usize) -> LaneFallback { self.primary_fallback(_lane) }
    fn middle_fallback(&self, _lane: usize) -> LaneFallback { self.primary_fallback(_lane) }
}