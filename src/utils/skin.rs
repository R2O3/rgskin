use std::{collections::HashSet, sync::{Arc, RwLock}, marker::PhantomData};

use crate::{Binary, Store, StringPattern, sample::SampleStore, texture::TextureStore, traits::{LaneType, SkinConfig}};

// TODO: add method for generating mipmaps for textures (for osu)

pub fn cleanup_stores<T: SkinConfig>(config: &T, textures: Option<&mut TextureStore>, samples: Option<&mut SampleStore>) {
    if let Some(samples) = samples {
        let sample_paths = config.get_required_sample_paths();
        let req_sample_paths: HashSet<_> = sample_paths.iter().collect();

        samples.retain(|t| req_sample_paths.iter().any(|p| p.matches_path(&t.path)));
    }

    if let Some(textures) = textures {
        let texture_paths = config.get_required_texture_paths(); // owned, kept alive
        let req_texture_paths: HashSet<_> = texture_paths.iter().collect();

        textures.retain(|t| req_texture_paths.iter().any(|p| p.matches_path(&t.get_path())));
    }
}

pub fn get_lane_type(keymode: u8, idx: usize) -> LaneType {
    let middle_idx = ((keymode - 1) as f32 / 2.0).floor() as usize;
        
    if keymode % 2 == 1 && idx == middle_idx {
        LaneType::Middle
    } else {
        let center_dist = if keymode % 2 == 1 {
            if idx < middle_idx {
                middle_idx - idx
            } else {
                idx - middle_idx
            }
        } else {
            if idx <= middle_idx {
                middle_idx - idx + 1
            } else {
                idx - middle_idx
            }
        };
        
        if center_dist % 2 == 0 {
            LaneType::Primary
        } else {
            LaneType::Secondary
        }
    }
}




pub struct StoreRelocator<'a, T: 'static, S: Store<T>> {
    store: &'a mut S,
    _phantom: PhantomData<T>,
}

impl<'a, T: 'static, S: Store<T>> StoreRelocator<'a, T, S> {
    pub fn new(store: &'a mut S) -> Self {
        StoreRelocator { store, _phantom: PhantomData }
    }

    pub fn reloc_arc_lock<U: Binary>(&mut self, item: &Option<Arc<RwLock<U>>>, target_path: StringPattern) {
        if let Some(texture_arc) = item {
            let guard = texture_arc.read().unwrap();
            let path = guard.get_path();
            self.store.copy(path, &target_path);
        }
    }

    pub fn reloc_str(&mut self, item: &Option<String>, target_path: StringPattern) {
        if let Some(item_path) = item {
            self.store.copy(item_path, &target_path);
        }
    }
}
