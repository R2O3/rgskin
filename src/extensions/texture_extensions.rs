#![allow(unused)]
#![warn(unused_imports)]

use std::sync::{Arc, RwLock};
use image::DynamicImage;
use crate::texture::Texture;

pub trait TextureArcExt {
    fn get_path(&self) -> String;
    fn path_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R;
    
    fn clone_texture(&self) -> Texture;
    fn take_texture(&self) -> Texture;
    
    fn get_image(&self) -> Option<DynamicImage>;
    fn set_image(&self, image: DynamicImage);
    fn take_image(&self) -> Option<DynamicImage>;
    fn clone_image(&self) -> Option<DynamicImage>;
    
    fn image_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&DynamicImage) -> R;
    
    fn image_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut DynamicImage) -> R;
    
    fn with_image<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&DynamicImage) -> R;
}

impl TextureArcExt for Arc<RwLock<Texture>> {
    fn get_path(&self) -> String {
        self.read().unwrap().path.clone()
    }
    
    fn path_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        let guard = self.read().unwrap();
        f(guard.path())
    }
    
    fn clone_texture(&self) -> Texture {
        self.read().unwrap().clone()
    }
    
    fn take_texture(&self) -> Texture {
        std::mem::replace(&mut *self.write().unwrap(), Texture::default())
    }
    
    fn get_image(&self) -> Option<DynamicImage> {
        use crate::BinaryArcExt;
        self.get_data()
    }
    
    fn set_image(&self, image: DynamicImage) {
        use crate::BinaryArcExt;
        self.set_data(image)
    }
    
    fn take_image(&self) -> Option<DynamicImage> {
        use crate::BinaryArcExt;
        self.take_data()
    }
    
    fn clone_image(&self) -> Option<DynamicImage> {
        use crate::BinaryArcExt;
        self.clone_data()
    }
    
    fn image_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&DynamicImage) -> R,
    {
        use crate::BinaryArcExt;
        self.data_ref(f)
    }
    
    fn image_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut DynamicImage) -> R,
    {
        use crate::BinaryArcExt;
        self.data_mut(f)
    }
    
    fn with_image<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&DynamicImage) -> R,
    {
        use crate::BinaryArcExt;
        self.with_data(f)
    }
}