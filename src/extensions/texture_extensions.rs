#![allow(unused)]
#![warn(unused_imports)]

use std::sync::{Arc, RwLock};
use image::RgbaImage;
use crate::texture::Texture;

pub trait TextureArcExt {
    fn clone_texture(&self) -> Texture;
    fn take_texture(&self) -> Texture;
    
    fn get_image(&self) -> Option<RgbaImage>;
    fn set_image(&self, image: RgbaImage);
    fn take_image(&self) -> Option<RgbaImage>;
    fn clone_image(&self) -> Option<RgbaImage>;
    
    fn image_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&RgbaImage) -> R;
    
    fn image_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut RgbaImage) -> R;
    
    fn with_image<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&RgbaImage) -> R;
}

impl TextureArcExt for Arc<RwLock<Texture>> {
    fn clone_texture(&self) -> Texture {
        self.read().unwrap().clone()
    }
    
    fn take_texture(&self) -> Texture {
        std::mem::replace(&mut *self.write().unwrap(), Texture::default())
    }
    
    fn get_image(&self) -> Option<RgbaImage> {
        use crate::BinaryArcExt;
        self.get_data()
    }
    
    fn set_image(&self, image: RgbaImage) {
        use crate::BinaryArcExt;
        self.set_data(image)
    }
    
    fn take_image(&self) -> Option<RgbaImage> {
        use crate::BinaryArcExt;
        self.take_data()
    }
    
    fn clone_image(&self) -> Option<RgbaImage> {
        use crate::BinaryArcExt;
        self.clone_data()
    }
    
    fn image_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&RgbaImage) -> R,
    {
        use crate::BinaryArcExt;
        self.data_ref(f)
    }
    
    fn image_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut RgbaImage) -> R,
    {
        use crate::BinaryArcExt;
        self.data_mut(f)
    }
    
    fn with_image<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&RgbaImage) -> R,
    {
        use crate::BinaryArcExt;
        self.with_data(f)
    }
}