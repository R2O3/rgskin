#![allow(unused)]

use std::{collections::HashSet, str::FromStr};
use merge::Merge;

use crate::GenericManiaSkin;

pub trait ManiaSkin<'a>: Merge {
    type Keymode;
    type ToParams;
    type FromReturn;
    
    fn to_generic_mania(&self, params: Self::ToParams) -> Result<GenericManiaSkin, Box<dyn std::error::Error>>;
    fn from_generic_mania(skin: &GenericManiaSkin) -> Result<Self::FromReturn, Box<dyn std::error::Error>>;

    fn get_keymode(&self, keymode: u8) -> Option<&Self::Keymode>;
    fn get_required_texture_paths(&self) -> HashSet<String>;
    fn get_required_sample_paths(&self) -> HashSet<String>;

    fn merge(&mut self, other: Self) where Self: Sized {
        <Self as Merge>::merge(self, other);
    }
}

// pub trait TaikoSkin {
    
// }

pub trait SkinConfig: ToString + FromStr + Merge {
    fn get_required_texture_paths(&self) -> HashSet<String>;
    fn get_required_sample_paths(&self) -> HashSet<String> { HashSet::new() } // not all games have config for sounds

    fn merge(&mut self, other: Self) where Self: Sized {
        <Self as Merge>::merge(self, other);
    }
}

pub trait ManiaSkinConfig: SkinConfig {
    type Keymode;

    fn get_keymode(&self, keymode: u8) -> Option<&Self::Keymode>;
}

pub trait KeymodeInvariant {
    fn get_keymode(&self) -> u8;
}