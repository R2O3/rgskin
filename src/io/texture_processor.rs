#![allow(unused)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::extensions::TextureArcExt;
use crate::Texture;

pub struct TextureProcessor<T = ()> {
    processed: HashMap<String, T>,
}

impl<T> TextureProcessor<T> {
    pub fn new() -> Self {
        Self {
            processed: HashMap::new(),
        }
    }

    pub fn process_once<F>(&mut self, texture: &Arc<RwLock<Texture>>, f: F) -> T
    where
        F: FnOnce(&Arc<RwLock<Texture>>) -> T,
        T: Clone,
    {
        let path = texture.get_path();
        
        if let Some(cached) = self.processed.get(&path) {
            return cached.clone();
        }
        
        let result = f(texture);
        self.processed.insert(path, result.clone());
        result
    }

    pub fn process_once_void<F>(&mut self, texture: &Arc<RwLock<Texture>>, f: F)
    where
        F: FnOnce(&Arc<RwLock<Texture>>),
        T: Default,
    {
        let path = texture.get_path();
        
        if self.processed.contains_key(&path) {
            return;
        }
        
        f(texture);
        self.processed.insert(path, T::default());
    }

    pub fn is_processed(&self, texture: &Arc<RwLock<Texture>>) -> bool {
        self.processed.contains_key(&texture.get_path())
    }

    pub fn get_cached(&self, texture: &Arc<RwLock<Texture>>) -> Option<&T> {
        self.processed.get(&texture.get_path())
    }
}

impl<T: Default> Default for TextureProcessor<T> {
    fn default() -> Self {
        Self::new()
    }
}
