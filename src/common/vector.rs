use merge::Merge;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}


impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Default for Vector2<f32> {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Default for Vector2<u32> {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl<T> Merge for Vector2<T> {
    fn merge(&mut self, other: Self) {
        use crate::utils::merge;
        merge::any::overwrite(&mut self.x, other.x);
        merge::any::overwrite(&mut self.y, other.y);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl Default for Vector3<f32> {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Default for Vector3<u32> {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

impl<T> Merge for Vector3<T> {
    fn merge(&mut self, other: Self) {
        use crate::utils::merge;
        merge::any::overwrite(&mut self.x, other.x);
        merge::any::overwrite(&mut self.y, other.y);
        merge::any::overwrite(&mut self.z, other.z);
    }
}
