use crate::{generic::gameplay::Gameplay, io::TextureStore, skin::generic::{keymode, layout::{HUDLayout, KeymodeLayout}, Keymode, Metadata}};

pub struct GenericManiaSkin {
    pub metadata: Metadata,
    pub gameplay: Gameplay,
    pub keymodes: Vec<Keymode>,
    pub textures: TextureStore
}

impl GenericManiaSkin {
    pub fn get_keymode(&self, keymode: u8) -> Option<&Keymode> {
        for k in &self.keymodes {
            if k.keymode == keymode { return Some(k); }
        }
        None
    }
}