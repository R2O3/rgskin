use crate::skin::generic::{elements::*, layout::KeymodeLayout};

#[derive(Clone)]
pub struct Keymode {
    pub keymode: u8,
    pub layout: KeymodeLayout,

    pub receptor_up: Vec<ReceptorUp>,
    pub receptor_down: Vec<ReceptorDown>,

    pub normal_note: Vec<NormalNote>,
    pub long_note_head: Vec<LongNoteHead>,
    pub long_note_body: Vec<LongNoteBody>,
    pub long_note_tail: Vec<LongNoteTail>,

    pub hit_lighting: HitLighting,
    pub column_lighting: ColumnLighting
}