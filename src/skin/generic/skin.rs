use crate::common::traits::ManiaSkin;
use crate::common::vector::Vector2;
use crate::generic::gameplay::Gameplay;
use crate::io::texture::TextureStore;
use crate::skin::generic::{Keymode, Metadata};

pub struct GenericManiaSkin {
    pub resolution: Vector2<u32>,
    pub metadata: Metadata,
    pub gameplay: Gameplay,
    pub keymodes: Vec<Keymode>,
    pub textures: TextureStore
}

impl ManiaSkin for GenericManiaSkin {
    type Keymode = Keymode;
    type ToParams = ();
    type FromReturn = Self;

    fn to_generic_mania(self, _params: Self::ToParams) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
        Ok(self)
    }

    fn from_generic_mania(skin: GenericManiaSkin) -> Result<Self::FromReturn, Box<dyn std::error::Error>> {
        Ok(skin)
    }

    fn get_keymode(&self, keymode: u8) -> Option<&Keymode> {
        for k in &self.keymodes {
            if k.keymode == keymode { return Some(k); }
        }
        None
    }
}
