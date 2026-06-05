use crate::numeric_enum;

numeric_enum! {
    pub enum QuaDimensions: u32 {
        X = 615,
        Y = 346,
        MaxResolution = 4096, // TODO: change this later
    }
}
