#![allow(unused)]

use crate::GenericManiaSkin;

pub trait ManiaSkin {
    type Keymode;
    type ToParams;
    type FromReturn;
    
    fn to_generic_mania(self, params: Self::ToParams) -> Result<GenericManiaSkin, Box<dyn std::error::Error>>;
    fn from_generic_mania(skin: GenericManiaSkin) -> Result<Self::FromReturn, Box<dyn std::error::Error>>;

    fn get_keymode(&self, keymode: u8) -> Option<&Self::Keymode>;
}

// pub trait TaikoSkin {
    
// }