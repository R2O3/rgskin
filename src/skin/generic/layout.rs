#[derive(Clone)]
pub struct KeymodeLayout {
    pub keymode: u8,
    pub x_offset: u32,
    pub hit_position: u32,
    pub receptor_offset: u32,
    pub column_widths: Vec<u32>,
    pub column_spacing: Vec<u32>,
}

pub struct HUDLayout {
    // unimplemented
}

impl KeymodeLayout {
    pub fn average_column_width(&mut self) -> u32 {
        self.column_widths.iter().sum::<u32>() / (self.column_widths.len() as u32)
    }

    pub fn average_column_spacing(&mut self) -> u32 {
        self.column_spacing.iter().sum::<u32>() / (self.column_spacing.len() as u32)
    }
}