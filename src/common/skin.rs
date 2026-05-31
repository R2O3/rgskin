#![allow(unused)]
use crate::common::alignment::Anchor;

// TODO: properly implement this later

pub enum AssetAttribute {
    Texture,
    Sample,
    Animatable(AnimationSpriteType),
    Alignment(Anchor),
}

impl AssetAttribute {
    pub fn as_anchor(&self) -> Option<Anchor> {
        match self {
            Self::Alignment(anchor) => Some(*anchor),
            _ => None,
        }
    }

    pub fn as_animation(&self) -> Option<&AnimationSpriteType> {
        match self {
            Self::Animatable(t) => Some(t),
            _ => None,
        }
    }

    pub fn is_texture(&self) -> bool {
        matches!(self, Self::Texture)
    }

    pub fn is_sample(&self) -> bool {
        matches!(self, Self::Sample)
    }
}

pub enum AnimationSpriteType {
    Single,
    SpriteSheet,
}