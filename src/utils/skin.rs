use std::collections::HashSet;

use crate::{sample::SampleStore, texture::TextureStore, traits::{LaneType, SkinConfig}, Binary, Store};

// TODO: add method for generating mipmaps for textures (for osu)
// TODO: add ensure_textures to add textures from skin elements in here without manual adding them

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
