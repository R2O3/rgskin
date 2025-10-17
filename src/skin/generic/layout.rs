use crate::common::{Anchor, Vector2, Vector3};

#[derive(Clone)]
pub struct KeymodeLayout {
    pub keymode: u8,
    pub receptor_above_notes: bool,
    pub x_offset: u32,
    pub hit_position: u32,
    pub receptor_offset: u32,
    pub column_widths: Vec<u32>,
    pub column_spacing: Vec<u32>,
}

pub struct HUDLayout {
    pub combo: (Vector3<f32>, Anchor),
    pub rating: (Vector3<f32>, Anchor),
    pub accuracy: (Vector3<f32>, Anchor),
    pub score: (Vector3<f32>, Anchor),
}

impl KeymodeLayout {
    pub fn average_column_width(&self) -> u32 {
        self.column_widths.iter().sum::<u32>() / (self.column_widths.len() as u32)
    }

    pub fn average_column_spacing(&self) -> u32 {
        self.column_spacing.iter().sum::<u32>() / (self.column_spacing.len() as u32)
    }
}