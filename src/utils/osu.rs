use crate::numeric_enum;

numeric_enum! {
    pub enum OsuDimensions: f32 {
        X = 640.0,
        Y = 480.0,
        ReceptorWidth = 48.0,
        ReceptorHeight = 108.0,
        ReceptorScale = 1.6,
        ReceptorScale2x = 3.2
    }
}