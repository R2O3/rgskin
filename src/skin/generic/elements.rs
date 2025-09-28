use crate::io::Texture;
use image::imageops::FilterType;
use std::sync::{Arc, RwLock};

pub trait SkinElement: Sync + Send {
    fn as_texture(&self) -> std::sync::RwLockReadGuard<'_, Texture>;
    fn as_texture_mut(&self) -> std::sync::RwLockWriteGuard<'_, Texture>;
    
    fn with_texture<F, R>(&self, f: F) -> R
    where 
        F: FnOnce(&Texture) -> R,
    {
        let texture = self.as_texture();
        f(&*texture)
    }
    
    fn with_texture_mut<F, R>(&self, f: F) -> R
    where 
        F: FnOnce(&mut Texture) -> R,
    {
        let mut texture = self.as_texture_mut();
        f(&mut *texture)
    }
    
    fn path(&self) -> String {
        self.with_texture(|texture| texture.path().to_string())
    }
    
    fn has_data(&self) -> bool {
        self.with_texture(|texture| texture.has_data())
    }
}

#[derive(Clone)]
pub struct ReceptorUp {
    pub texture: Arc<RwLock<Texture>>,
}

impl ReceptorUp {
    pub fn new(texture: Arc<RwLock<Texture>>) -> Self {
        Self { texture }
    }
    
    pub fn with_texture_data(texture: Texture) -> Self {
        Self {
            texture: Arc::new(RwLock::new(texture)),
        }
    }
    
    pub fn from_path(path: String) -> Self {
        Self {
            texture: Arc::new(RwLock::new(Texture::new(path))),
        }
    }
}

impl SkinElement for ReceptorUp {
    fn as_texture(&self) -> std::sync::RwLockReadGuard<'_, Texture> {
        self.texture.read().unwrap()
    }
    
    fn as_texture_mut(&self) -> std::sync::RwLockWriteGuard<'_, Texture> {
        self.texture.write().unwrap()
    }
}

#[derive(Clone)]
pub struct ReceptorDown {
    pub texture: Arc<RwLock<Texture>>,
}

impl ReceptorDown {
    pub fn new(texture: Arc<RwLock<Texture>>) -> Self {
        Self { texture }
    }
    
    pub fn with_texture_data(texture: Texture) -> Self {
        Self {
            texture: Arc::new(RwLock::new(texture)),
        }
    }
    
    pub fn from_path(path: String) -> Self {
        Self {
            texture: Arc::new(RwLock::new(Texture::new(path))),
        }
    }
}

impl SkinElement for ReceptorDown {
    fn as_texture(&self) -> std::sync::RwLockReadGuard<'_, Texture> {
        self.texture.read().unwrap()
    }
    
    fn as_texture_mut(&self) -> std::sync::RwLockWriteGuard<'_, Texture> {
        self.texture.write().unwrap()
    }
}

pub trait Hitobject: SkinElement {
    fn normalize(&mut self) {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct NormalNote {
    pub texture: Arc<RwLock<Texture>>,
}

impl NormalNote {
    pub fn new(texture: Arc<RwLock<Texture>>) -> Self {
        Self { texture }
    }
    
    pub fn with_texture_data(texture: Texture) -> Self {
        Self {
            texture: Arc::new(RwLock::new(texture)),
        }
    }
    
    pub fn from_path(path: String) -> Self {
        Self {
            texture: Arc::new(RwLock::new(Texture::new(path))),
        }
    }
}

impl SkinElement for NormalNote {
    fn as_texture(&self) -> std::sync::RwLockReadGuard<'_, Texture> {
        self.texture.read().unwrap()
    }
    
    fn as_texture_mut(&self) -> std::sync::RwLockWriteGuard<'_, Texture> {
        self.texture.write().unwrap()
    }
}

impl Hitobject for NormalNote {}

#[derive(Clone)]
pub struct LongNoteHead {
    pub texture: Arc<RwLock<Texture>>,
}

impl LongNoteHead {
    pub fn new(texture: Arc<RwLock<Texture>>) -> Self {
        Self { texture }
    }
    
    pub fn with_texture_data(texture: Texture) -> Self {
        Self {
            texture: Arc::new(RwLock::new(texture)),
        }
    }
    
    pub fn from_path(path: String) -> Self {
        Self {
            texture: Arc::new(RwLock::new(Texture::new(path))),
        }
    }
}

impl SkinElement for LongNoteHead {
    fn as_texture(&self) -> std::sync::RwLockReadGuard<'_, Texture> {
        self.texture.read().unwrap()
    }
    
    fn as_texture_mut(&self) -> std::sync::RwLockWriteGuard<'_, Texture> {
        self.texture.write().unwrap()
    }
}

impl Hitobject for LongNoteHead {}

#[derive(Clone)]
pub struct LongNoteTail {
    pub texture: Arc<RwLock<Texture>>,
}

impl LongNoteTail {
    pub fn new(texture: Arc<RwLock<Texture>>) -> Self {
        Self { texture }
    }
    
    pub fn with_texture_data(texture: Texture) -> Self {
        Self {
            texture: Arc::new(RwLock::new(texture)),
        }
    }
    
    pub fn from_path(path: String) -> Self {
        Self {
            texture: Arc::new(RwLock::new(Texture::new(path))),
        }
    }
}

impl SkinElement for LongNoteTail {
    fn as_texture(&self) -> std::sync::RwLockReadGuard<'_, Texture> {
        self.texture.read().unwrap()
    }
    
    fn as_texture_mut(&self) -> std::sync::RwLockWriteGuard<'_, Texture> {
        self.texture.write().unwrap()
    }
}

impl Hitobject for LongNoteTail {}

#[derive(Clone)]
pub struct LongNoteBody {
    pub texture: Arc<RwLock<Texture>>,
}

impl LongNoteBody {
    pub fn new(texture: Arc<RwLock<Texture>>) -> Self {
        Self { texture }
    }
    
    pub fn with_texture_data(texture: Texture) -> Self {
        Self {
            texture: Arc::new(RwLock::new(texture)),
        }
    }
    
    pub fn from_path(path: String) -> Self {
        Self {
            texture: Arc::new(RwLock::new(Texture::new(path))),
        }
    }
}

impl SkinElement for LongNoteBody {
    fn as_texture(&self) -> std::sync::RwLockReadGuard<'_, Texture> {
        self.texture.read().unwrap()
    }
    
    fn as_texture_mut(&self) -> std::sync::RwLockWriteGuard<'_, Texture> {
        self.texture.write().unwrap()
    }
}

impl Hitobject for LongNoteBody {}