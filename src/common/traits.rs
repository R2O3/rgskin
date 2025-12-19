#![allow(unused)]

use std::{collections::HashSet, str::FromStr};

use crate::GenericManiaSkin;

pub trait ManiaSkin<'a> {
    type Keymode;
    type ToParams;
    type FromReturn;
    
    fn to_generic_mania(&self, params: Self::ToParams) -> Result<GenericManiaSkin, Box<dyn std::error::Error>>;
    fn from_generic_mania(skin: &GenericManiaSkin) -> Result<Self::FromReturn, Box<dyn std::error::Error>>;

    fn get_keymode(&self, keymode: u8) -> Option<&Self::Keymode>;
    fn get_dynamic_texture_paths(&self) -> HashSet<String>;
}

// pub trait TaikoSkin {
    
// }

pub trait SkinConfig: ToString + FromStr {
    fn get_dynamic_texture_paths(&self) -> HashSet<String>;
}

pub trait ManiaSkinConfig: SkinConfig {
    type Keymode;

    fn get_keymode(&self, keymode: u8) -> Option<&Self::Keymode>;
}
