use serde::{Deserialize, Serialize};
use crate::common::color::Rgba;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct JudgementColors {
    pub flawless: Rgba,
    pub perfect: Rgba,
    pub great: Rgba,
    pub alright: Rgba,
    pub okay: Rgba,
    pub miss: Rgba,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapColors {
    #[serde(rename = "1/3")]
    pub snap_1_3: Rgba,
    #[serde(rename = "1/4")]
    pub snap_1_4: Rgba,
    #[serde(rename = "1/6")]
    pub snap_1_6: Rgba,
    #[serde(rename = "1/8")]
    pub snap_1_8: Rgba,
    #[serde(rename = "1/12")]
    pub snap_1_12: Rgba,
    #[serde(rename = "1/16")]
    pub snap_1_16: Rgba,
    #[serde(rename = "1/24")]
    pub snap_1_24: Rgba,
    #[serde(rename = "1/48")]
    pub snap_1_48: Rgba,
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
