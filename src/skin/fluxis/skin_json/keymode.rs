#[derive(Clone, Debug)]
pub struct Keymode {
    pub keymode: u8,
    pub column_width: u32,
    pub hit_position: i32,
    pub tint_notes: bool,
    pub tint_lns: bool,
    pub tint_receptors: bool,
    pub colors: Vec<String>,
    pub receptors_first: bool,
    pub receptor_offset: i32,
    
    pub receptor_images: Vec<String>,
    pub receptor_images_down: Vec<String>,
    pub normal_note_images: Vec<String>,
    pub long_note_head_images: Vec<String>,
    pub long_note_body_images: Vec<String>,
    pub long_note_tail_images: Vec<String>,
    pub tick_images: Vec<String>,
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

