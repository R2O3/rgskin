#![allow(unused)]
use crate::common::alignment::Anchor;

// TODO: properly implement this later

pub enum AssetAttribute {
    Texture,
    Sample,
    Animatable(AnimationSpriteType),
    Alignment(Anchor),
}

pub enum AnimationSpriteType {
    Single,
    SpriteSheet,
}