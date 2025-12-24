use std::collections::HashSet;

use crate::common::traits::ManiaSkin;
use crate::common::vector::Vector2;
use crate::generic::gameplay::Gameplay;
use crate::generic::sound::Sounds;
use crate::io::texture::TextureStore;
use crate::sample::SampleStore;
use crate::skin::generic::{Keymode, Metadata};
use crate::{texture, BinaryArcExt, Store};
use crate::extensions::TextureArcExt;

#[derive(Clone)]
pub struct GenericManiaSkin {
    pub resolution: Vector2<u32>,
    pub sounds: Sounds,
    pub metadata: Metadata,
    pub gameplay: Gameplay,
    pub keymodes: Vec<Keymode>,
    pub textures: TextureStore,
    pub samples: SampleStore
}

impl<'a> ManiaSkin<'a> for GenericManiaSkin {
    type Keymode = Keymode;
    type ToParams = ();
    type FromReturn = Self;

    fn to_generic_mania(&self, _params: Self::ToParams) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
        Ok(self.clone())
    }

    fn from_generic_mania(skin: &GenericManiaSkin) -> Result<Self::FromReturn, Box<dyn std::error::Error>> {
        Ok(skin.clone())
    }

    fn get_keymode(&self, keymode: u8) -> Option<&Keymode> {
        for k in &self.keymodes {
            if k.keymode == keymode { return Some(k); }
        }
        None
    }

    fn get_required_texture_paths(&self) -> HashSet<String> {
        self.textures.iter()
        .filter(|t| t.1.is_loaded())
        .map(|t| t.0.to_string())
        .collect()
    }
    
    fn get_required_sample_paths(&self) -> HashSet<String> {
        self.samples.iter()
        .map(|t| t.0.to_string())
        .collect()
    }
}