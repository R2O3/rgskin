use crate::common::vector::Vector2;

pub struct Resizer {
    pub source: Vector2<u32>,
    pub target: Vector2<u32>,
}

impl Resizer {
    pub fn new(dimentions: Vector2<u32>, target: Option<Vector2<u32>>) -> Self {
        Self { source: dimentions, target: target.unwrap_or(dimentions) }
    }

    pub fn scale_fit(&self, target_width: u32, target_height: u32) -> f32 {
        let scale_x = target_width as f32 / self.source.x as f32;
        let scale_y = target_height as f32 / self.source.y as f32;
        scale_x.min(scale_y)
    }

    pub fn scale_fill(&self, target_width: u32, target_height: u32) -> f32 {
        let scale_x = target_width as f32 / self.source.x as f32;
        let scale_y = target_height as f32 / self.source.y as f32;
        scale_x.max(scale_y)
    }

    pub fn scale_fill_relative(&self) -> f32 {
        self.scale_fill(self.target.x, self.target.y)
    }

    pub fn scale_fit_relative(&self) -> f32 {
        self.scale_fit(self.target.x, self.target.y)
    }
    
    /// Scale from source to target in X
    pub fn to_target_x<T: num_traits::NumCast>(&self, value: f32) -> T {
        num_traits::cast(value * self.target.x as f32 * self.scale_fit_relative()).unwrap()
    }

    /// Scale from source to target in Y
    pub fn to_target_y<T: num_traits::NumCast>(&self, value: f32) -> T {
        num_traits::cast(value * self.target.y as f32 * self.scale_fit_relative()).unwrap()
    }

    /// Scale from target back to source in X
    pub fn from_target_x<T: num_traits::NumCast>(&self, value: f32) -> T {
        num_traits::cast(value * (self.target.x as f32 / self.scale_fit_relative())).unwrap()
    }

    /// Scale from target back to source in Y
    pub fn from_target_y<T: num_traits::NumCast>(&self, value: f32) -> T {
        num_traits::cast(value * (self.target.y as f32 / self.scale_fit_relative())).unwrap()
    }

    pub fn resize_fit(&self, target_width: u32, target_height: u32) -> Vector2<u32> {
        let scale = self.scale_fit(target_width, target_height);
        Vector2::new(
            (self.source.x as f32 * scale).round() as u32,
            (self.source.y as f32 * scale).round() as u32
        )
    }

    pub fn resize_fill(&self, target_width: u32, target_height: u32) -> Vector2<u32> {
        let scale = self.scale_fill(target_width, target_height);
        Vector2::new(
            (self.source.x as f32 * scale).round() as u32,
            (self.source.y as f32 * scale).round() as u32
        )
    }

    pub fn auto_resize_fit(&self) -> Vector2<u32> {
        self.resize_fit(self.target.x, self.target.y)
    }

    pub fn auto_resize_fill(&self) -> Vector2<u32> {
        self.resize_fill(self.target.x, self.target.y)
    }
}
