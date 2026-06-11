use merge::Merge;
use serde::{Deserialize, Serialize};
use crate::common::color::Rgba;
use rgskin_derive::merge_for_all;
use crate::utils::serde::serialize_rgb;


#[merge_for_all(strategy = crate::utils::merge::any::overwrite)]
#[derive(Clone, Debug, Serialize, Deserialize, Merge)]
#[serde(default)]
pub struct JudgementColors {
    #[serde(serialize_with = "serialize_rgb::hex")]
    pub flawless: Rgba,
    #[serde(serialize_with = "serialize_rgb::hex")]
    pub perfect: Rgba,
    #[serde(serialize_with = "serialize_rgb::hex")]
    pub great: Rgba,
    #[serde(serialize_with = "serialize_rgb::hex")]
    pub alright: Rgba,
    #[serde(serialize_with = "serialize_rgb::hex")]
    pub okay: Rgba,
    #[serde(serialize_with = "serialize_rgb::hex")]
    pub miss: Rgba,
}

impl JudgementColors {
    pub fn to_vec(&self) -> Vec<Rgba> {
        vec![
            self.flawless,
            self.perfect,
            self.great,
            self.alright,
            self.okay,
            self.miss,
        ]
    }

    pub fn from_vec(vec: Vec<Rgba>) -> Option<Self> {
        if vec.len() < 6 {
            return Some(Self::default());
        }
        let mut iter = vec.into_iter();
        Some(Self {
            flawless: iter.next()?,
            perfect: iter.next()?,
            great: iter.next()?,
            alright: iter.next()?,
            okay: iter.next()?,
            miss: iter.next()?,
        })
    }
}

impl Default for JudgementColors {
    fn default() -> Self {
        Self {
            flawless: Rgba::from_hex("#00C3FF").unwrap(),
            perfect: Rgba::from_hex("#22FFB5").unwrap(),
            great: Rgba::from_hex("#4BFF3B").unwrap(),
            alright: Rgba::from_hex("#FFF12B").unwrap(),
            okay: Rgba::from_hex("#F7AD40").unwrap(),
            miss: Rgba::from_hex("#FF5555").unwrap(),
        }
    }
}

#[merge_for_all(strategy = crate::utils::merge::any::overwrite)]
#[derive(Clone, Debug, Serialize, Deserialize, Merge)]
#[serde(default)]
pub struct SnapColors {
    #[serde(rename = "1/3", serialize_with = "serialize_rgb::hex")]
    pub snap_1_3: Rgba,
    #[serde(rename = "1/4", serialize_with = "serialize_rgb::hex")]
    pub snap_1_4: Rgba,
    #[serde(rename = "1/6", serialize_with = "serialize_rgb::hex")]
    pub snap_1_6: Rgba,
    #[serde(rename = "1/8", serialize_with = "serialize_rgb::hex")]
    pub snap_1_8: Rgba,
    #[serde(rename = "1/12", serialize_with = "serialize_rgb::hex")]
    pub snap_1_12: Rgba,
    #[serde(rename = "1/16", serialize_with = "serialize_rgb::hex")]
    pub snap_1_16: Rgba,
    #[serde(rename = "1/24", serialize_with = "serialize_rgb::hex")]
    pub snap_1_24: Rgba,
    #[serde(rename = "1/48", serialize_with = "serialize_rgb::hex")]
    pub snap_1_48: Rgba,
}

impl SnapColors {
    pub fn to_vec(&self) -> Vec<Rgba> {
        vec![
            self.snap_1_3,
            self.snap_1_4,
            self.snap_1_6,
            self.snap_1_8,
            self.snap_1_12,
            self.snap_1_16,
            self.snap_1_24,
            self.snap_1_48,
        ]
    }

    pub fn from_vec(vec: Vec<Rgba>) -> Option<Self> {
        if vec.len() < 8 {
            return Some(Self::default());
        }
        let mut iter = vec.into_iter();
        Some(Self {
            snap_1_3: iter.next()?,
            snap_1_4: iter.next()?,
            snap_1_6: iter.next()?,
            snap_1_8: iter.next()?,
            snap_1_12: iter.next()?,
            snap_1_16: iter.next()?,
            snap_1_24: iter.next()?,
            snap_1_48: iter.next()?,
        })
    }
}

impl Default for SnapColors {
    fn default() -> Self {
        Self {
            snap_1_3: Rgba::from_hex("#FF5555").unwrap(),
            snap_1_4: Rgba::from_hex("#558EFF").unwrap(),
            snap_1_6: Rgba::from_hex("#8EFF55").unwrap(),
            snap_1_8: Rgba::from_hex("#FFE355").unwrap(),
            snap_1_12: Rgba::from_hex("#C655FF").unwrap(),
            snap_1_16: Rgba::from_hex("#55FFAA").unwrap(),
            snap_1_24: Rgba::from_hex("#FF55AA").unwrap(),
            snap_1_48: Rgba::from_hex("#BFBFBF").unwrap(),
        }
    }
}
