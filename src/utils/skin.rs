use std::collections::HashSet;

use crate::{sample::SampleStore, texture::TextureStore, traits::SkinConfig, Store};

pub fn cleanup_stores<T: SkinConfig>(config: &T, textures: Option<&mut TextureStore>, samples: Option<&mut SampleStore>) {
    if samples.is_some() {
        let req_sample_paths: HashSet<_> = config.get_required_sample_paths().iter()
        .map(|p| p.to_lowercase())
        .collect();

        samples.unwrap()
        .retain(|t| req_sample_paths.contains(&t.get_path().to_lowercase()));
    }

    if textures.is_some() {
        let req_texture_paths: HashSet<_> = config.get_required_texture_paths().iter()
        .map(|p| p.to_lowercase())
        .collect();

        textures.unwrap()
        .retain(|t| req_texture_paths.contains(&t.get_path().to_lowercase()));
    }
}