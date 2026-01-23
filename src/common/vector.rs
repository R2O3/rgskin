use merge::Merge;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Merge for Vector2<T> {
    fn merge(&mut self, other: Self) {
        use crate::utils::merge;
        merge::any::overwrite(&mut self.x, other.x);
        merge::any::overwrite(&mut self.y, other.y);
    }
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
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

impl<T> Merge for Vector3<T> {
    fn merge(&mut self, other: Self) {
        use crate::utils::merge;
        merge::any::overwrite(&mut self.x, other.x);
        merge::any::overwrite(&mut self.y, other.y);
        merge::any::overwrite(&mut self.z, other.z);
    }
}
