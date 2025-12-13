use crate::common::alignment::*;
use crate::common::vector::*;

#[derive(Clone)]
pub struct KeymodeLayout {
    pub keymode: u8,
    pub receptor_above_notes: bool,
    pub x_offset: f32, // relative
    pub hit_position: f32, // relative
    pub receptor_offset: i32,
    pub column_widths: Vec<f32>, // relative
    pub column_spacing: Vec<u32>,
}

pub struct HUDLayout {
    pub combo: (Vector3<f32>, Alignment),
    pub rating: (Vector3<f32>, Alignment),
    pub accuracy: (Vector3<f32>, Alignment),
    pub score: (Vector3<f32>, Alignment),
    pub judgement: (Vector3<f32>, Alignment),
}

impl KeymodeLayout {
    pub fn average_column_width(&self) -> f32 {
        self.column_widths.iter().sum::<f32>() / (self.column_widths.len() as f32)
    }

    pub fn average_column_spacing(&self) -> u32 {
        self.column_spacing.iter().sum::<u32>() / (self.column_spacing.len() as u32)
    }
}