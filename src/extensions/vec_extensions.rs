pub trait VecExtensions {
    fn average(&self) -> Option<f32>;
}

impl VecExtensions for Vec<f32> {
    fn average(&self) -> Option<f32> {
        if self.is_empty() {
            return None;
        }
        let sum: f32 = self.iter().sum();
        Some(sum / self.len() as f32)
    }
}
