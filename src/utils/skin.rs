use std::collections::HashSet;

use crate::{sample::SampleStore, texture::TextureStore, traits::{LaneType, SkinConfig}, Binary, Store};

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
