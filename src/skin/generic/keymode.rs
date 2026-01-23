#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{skin::generic::{elements::*, layout::KeymodeLayout}, traits::KeymodeInvariant};

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
}

impl KeymodeInvariant for Keymode {
    fn get_keymode(&self) -> u8 { self.keymode }
}