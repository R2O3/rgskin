#![allow(unused)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::extensions::TextureArcExt;
use crate::io::texture::Texture;
use crate::BinaryArcExt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureKey {
    Path(String),
    Hash(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorStrategy {
    Path,
    HashFallbackPath,
}

pub struct TextureProcessor<T = ()> {
    processed: HashMap<TextureKey, T>,
    strategy: ProcessorStrategy,
}

impl<T> TextureProcessor<T> {
    pub fn new() -> Self {
        Self::with_strategy(ProcessorStrategy::Path)
    }

    pub fn with_strategy(strategy: ProcessorStrategy) -> Self {
        Self {
            processed: HashMap::new(),
            strategy,
        }
    }

    fn get_key(&self, texture: &Arc<RwLock<Texture>>) -> TextureKey {
        match self.strategy {
            ProcessorStrategy::HashFallbackPath => {
                texture.get_hash()
                    .map(TextureKey::Hash)
                    .unwrap_or_else(|| TextureKey::Path(texture.get_path()))
            }
            ProcessorStrategy::Path => TextureKey::Path(texture.get_path()),
        }
    }

    pub fn process_once<F>(&mut self, texture: &Arc<RwLock<Texture>>, f: F) -> T
    where
        F: FnOnce(&Arc<RwLock<Texture>>) -> T,
        T: Clone,
    {
        let key = self.get_key(texture);
        
        if let Some(cached) = self.processed.get(&key) {
            return cached.clone();
        }
        
        let result = f(texture);
        self.processed.insert(key, result.clone());
        result
    }

    pub fn process_once_void<F>(&mut self, texture: &Arc<RwLock<Texture>>, f: F)
    where
        F: FnOnce(&Arc<RwLock<Texture>>),
        T: Default,
    {
        let key = self.get_key(texture);
        
        if self.processed.contains_key(&key) {
            return;
        }
        
        f(texture);
        self.processed.insert(key, T::default());
    }

    pub fn is_processed(&self, texture: &Arc<RwLock<Texture>>) -> bool {
        self.processed.contains_key(&self.get_key(texture))
    }

    pub fn get_cached(&self, texture: &Arc<RwLock<Texture>>) -> Option<&T> {
        self.processed.get(&self.get_key(texture))
    }
}

impl<T: Default> Default for TextureProcessor<T> {
    fn default() -> Self {
        Self::new()
    }
}
