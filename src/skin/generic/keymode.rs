#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{skin::generic::{elements::*, layout::KeymodeLayout}, traits::{KeymodeInvariant, LaneFallback, LaneType}, utils::skin::get_lane_type, BinaryArcExtOption};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone)]
pub struct Keymode {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub keymode: u8,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub layout: KeymodeLayout,

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub receptor_up: Vec<ReceptorUp>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub receptor_down: Vec<ReceptorDown>,

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub normal_note: Vec<NormalNote>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub long_note_head: Vec<LongNoteHead>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub long_note_body: Vec<LongNoteBody>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub long_note_tail: Vec<LongNoteTail>,

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub hit_lighting: HitLighting,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub column_lighting: ColumnLighting,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub judgement_line: JudgementLine,

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub stage: Stage,

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub fallbacks: Vec<LaneFallback>,
}

impl KeymodeInvariant for Keymode {
    fn get_keymode(&self) -> u8 { self.keymode }

    fn get_receptors(&self) -> Vec<String> {
        self.receptor_up.iter().map(|r| r.texture.get_path().unwrap_or_default()).collect()
    }

    fn get_receptors_down(&self) -> Vec<String> {
        self.receptor_down.iter().map(|r| r.texture.get_path().unwrap_or_default()).collect()
    }

    fn get_normal_notes(&self) -> Vec<String> {
        self.normal_note.iter().map(|n| n.texture.get_path().unwrap_or_default()).collect()
    }

    fn get_long_note_heads(&self) -> Vec<String> {
        self.long_note_head.iter().map(|n| n.texture.get_path().unwrap_or_default()).collect()
    }

    fn get_long_note_bodies(&self) -> Vec<String> {
        self.long_note_body.iter().map(|n| n.texture.get_path().unwrap_or_default()).collect()
    }

    fn get_long_note_tails(&self) -> Vec<String> {
        self.long_note_tail.iter().map(|n| n.texture.get_path().unwrap_or_default()).collect()
    }

    // I really can't be bothered to properly implement this
    fn primary_fallback(&self, _lane: usize) -> LaneFallback { LaneFallback::default() }
    fn secondary_fallback(&self, _lane: usize) -> LaneFallback { LaneFallback::default() }
    fn middle_fallback(&self, _lane: usize) -> LaneFallback { LaneFallback::default() }
    fn get_fallbacks(&self) -> Vec<LaneFallback> { self.fallbacks.clone() }
}