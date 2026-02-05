#![allow(unused)]

use core::num;
use std::{collections::HashSet, rc::Rc, str::FromStr};
use merge::Merge;

use crate::{utils::skin::get_lane_type, GenericManiaSkin};

#[derive(Debug, PartialEq, Eq)]
pub enum LaneType {
    Primary,
    Secondary,
    Middle,
}

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

    fn get_receptors(&self) -> Vec<String>;
    fn get_receptors_down(&self) -> Vec<String>;

    fn get_normal_notes(&self) -> Vec<String>;

    fn get_long_note_heads(&self) -> Vec<String>;
    fn get_long_note_bodies(&self) -> Vec<String>;
    fn get_long_note_tails(&self) -> Vec<String>;

    fn primary_fallback(&self, _lane: usize) -> LaneFallback;
    fn secondary_fallback(&self, _lane: usize) -> LaneFallback;
    fn middle_fallback(&self, _lane: usize) -> LaneFallback;

    fn get_fallbacks(&self) -> Vec<LaneFallback> {
        let num_keys = self.get_keymode() as usize;

        (0..num_keys)
            .map(|idx| {
                match get_lane_type(self.get_keymode(), idx) {
                    LaneType::Primary => self.primary_fallback(idx+1),
                    LaneType::Secondary => self.secondary_fallback(idx+1),
                    LaneType::Middle => self.middle_fallback(idx+1),
                }
            })
            .collect()
    }
}

#[derive(Clone, Default)]
pub struct LaneFallback {
    pub receptor: String,
    pub receptor_down: String,
    pub normal_note: String,
    pub long_note_head: String,
    pub long_note_body: String,
    pub long_note_tail: String,
}