use std::sync::{Arc, RwLock};
use crate::texture::Texture;

pub trait GetAllTextures {
    fn get_all_textures(&self) -> Vec<Arc<RwLock<Texture>>>;
}

impl<T: GetAllTextures> GetAllTextures for Option<T> {
    fn get_all_textures(&self) -> Vec<Arc<RwLock<Texture>>> {
        self.as_ref().map_or_else(Vec::new, |inner| inner.get_all_textures())
    }
}

impl<T: GetAllTextures> GetAllTextures for Vec<T> {
    fn get_all_textures(&self) -> Vec<Arc<RwLock<Texture>>> {
        self.iter().flat_map(|item| item.get_all_textures()).collect()
    }
}

impl GetAllTextures for Arc<RwLock<Texture>> {
    fn get_all_textures(&self) -> Vec<Arc<RwLock<Texture>>> {
        vec![self.clone()]
    }
}

#[doc(hidden)]
pub struct Wrap<T>(pub T);

#[doc(hidden)]
pub trait ExtractSpecific {
    fn _extract_textures(&self) -> Vec<Arc<RwLock<crate::io::texture::Texture>>>;
}

impl<T: GetAllTextures> ExtractSpecific for Wrap<&T> {
    fn _extract_textures(&self) -> Vec<Arc<RwLock<crate::io::texture::Texture>>> {
        self.0.get_all_textures()
    }
}

#[doc(hidden)]
pub trait ExtractFallback {
    fn _extract_textures(&self) -> Vec<Arc<RwLock<crate::io::texture::Texture>>>;
}

impl<T> ExtractFallback for &Wrap<T> {
    fn _extract_textures(&self) -> Vec<Arc<RwLock<crate::io::texture::Texture>>> {
        Vec::new()
    }
}
