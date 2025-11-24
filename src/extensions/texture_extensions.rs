#![allow(unused)]
#![warn(unused_imports)]

use std::sync::{Arc, RwLock};
use image::DynamicImage;
use crate::{Binary, Texture};

pub trait TextureArcExt {
    fn get_path(&self) -> String;
    
    fn has_data(&self) -> bool;
    
    fn get_data(&self) -> Option<DynamicImage>;
    
    fn set_data(&self, data: DynamicImage);
    
    fn clear_data(&self);
    
    fn replace_data(&self, data: DynamicImage) -> Option<DynamicImage>;
    
    fn take_data(&self) -> Option<DynamicImage>;
    
    fn load_from_bytes(&self, bytes: Vec<u8>) -> Result<(), image::ImageError>;
    
    fn clone_texture(&self) -> Texture;
    
    fn clone_data(&self) -> Option<DynamicImage>;
    
    fn take_texture(&self) -> Texture;
    
    fn path_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R;
    
    fn data_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Option<DynamicImage>) -> R;
    
    fn data_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Option<DynamicImage>) -> R;
    
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
    
    fn has_data(&self) -> bool {
        self.read().unwrap().has_data()
    }
    
    fn get_data(&self) -> Option<DynamicImage> {
        self.read().unwrap().data().clone()
    }
    
    fn set_data(&self, data: DynamicImage) {
        self.write().unwrap().set_data(data);
    }
    
    fn clear_data(&self) {
        self.write().unwrap().clear();
    }
    
    fn replace_data(&self, data: DynamicImage) -> Option<DynamicImage> {
        self.write().unwrap().replace_data(data)
    }
    
    fn take_data(&self) -> Option<DynamicImage> {
        self.write().unwrap().take_data()
    }
    
    fn load_from_bytes(&self, bytes: Vec<u8>) -> Result<(), image::ImageError> {
        self.write().unwrap().set_from_bytes(bytes)
    }
    
    fn clone_texture(&self) -> Texture {
        self.read().unwrap().clone()
    }
    
    fn clone_data(&self) -> Option<DynamicImage> {
        self.read().unwrap().data().clone()
    }
    
    fn take_texture(&self) -> Texture {
        std::mem::replace(&mut *self.write().unwrap(), Texture::default())
    }
    
    fn path_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        let guard = self.read().unwrap();
        f(guard.path())
    }
    
    fn data_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Option<DynamicImage>) -> R,
    {
        let guard = self.read().unwrap();
        f(guard.data())
    }
    
    fn data_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Option<DynamicImage>) -> R,
    {
        let mut guard = self.write().unwrap();
        f(guard.data_mut())
    }
    
    fn image_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&DynamicImage) -> R,
    {
        let guard = self.read().unwrap();
        guard.data().as_ref().map(f)
    }
    
    fn image_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut DynamicImage) -> R,
    {
        let mut guard = self.write().unwrap();
        guard.data_mut().as_mut().map(f)
    }
    
    fn with_image<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&DynamicImage) -> R,
    {
        let guard = self.read().unwrap();
        f(guard.data().as_ref().unwrap())
    }
}