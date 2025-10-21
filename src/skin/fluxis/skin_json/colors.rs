use tinyjson::JsonValue;
use std::collections::HashMap;
use crate::{common::Rgba, utils::serde::json::get_string};

#[derive(Clone, Debug)]
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

impl JudgementColors {
    pub fn from_map(obj: &HashMap<String, JsonValue>) -> Self {
        let default = Self::default();
        let mut judgements = default.clone();
        
        if let Some(s) = Self::get_string(obj, "flawless") {
            judgements.flawless = Rgba::from_hex(&s).unwrap_or(default.flawless);
        }
        if let Some(s) = Self::get_string(obj, "perfect") {
            judgements.perfect = Rgba::from_hex(&s).unwrap_or(default.perfect);
        }
        if let Some(s) = Self::get_string(obj, "great") {
            judgements.great = Rgba::from_hex(&s).unwrap_or(default.great);
        }
        if let Some(s) = Self::get_string(obj, "alright") {
            judgements.alright = Rgba::from_hex(&s).unwrap_or(default.alright);
        }
        if let Some(s) = Self::get_string(obj, "okay") {
            judgements.okay = Rgba::from_hex(&s).unwrap_or(default.okay);
        }
        if let Some(s) = Self::get_string(obj, "miss") {
            judgements.miss = Rgba::from_hex(&s).unwrap_or(default.miss);
        }
        
        judgements
    }
    
    fn get_string(map: &HashMap<String, JsonValue>, key: &str) -> Option<String> {
        map.get(key).and_then(|v| v.get::<String>()).map(|s| s.to_string())
    }
    
    pub fn to_json(&self) -> JsonValue {
        let mut map = HashMap::new();
        map.insert("flawless".to_string(), JsonValue::from(self.flawless.to_hex()));
        map.insert("perfect".to_string(), JsonValue::from(self.perfect.to_hex()));
        map.insert("great".to_string(), JsonValue::from(self.great.to_hex()));
        map.insert("alright".to_string(), JsonValue::from(self.alright.to_hex()));
        map.insert("okay".to_string(), JsonValue::from(self.okay.to_hex()));
        map.insert("miss".to_string(), JsonValue::from(self.miss.to_hex()));
        JsonValue::from(map)
    }
}

#[derive(Clone, Debug)]
pub struct SnapColors {
    pub snap_1_3: Rgba,
    pub snap_1_4: Rgba,
    pub snap_1_6: Rgba,
    pub snap_1_8: Rgba,
    pub snap_1_12: Rgba,
    pub snap_1_16: Rgba,
    pub snap_1_24: Rgba,
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

impl SnapColors {
    pub fn from_map(obj: &HashMap<String, JsonValue>) -> Self {
        let default = Self::default();
        let mut snap_colors = default.clone();
        
        if let Some(s) = get_string(obj, "1/3") {
            snap_colors.snap_1_3 = Rgba::from_hex(&s).unwrap_or(default.snap_1_3);
        }
        if let Some(s) = get_string(obj, "1/4") {
            snap_colors.snap_1_4 = Rgba::from_hex(&s).unwrap_or(default.snap_1_4);
        }
        if let Some(s) = get_string(obj, "1/6") {
            snap_colors.snap_1_6 = Rgba::from_hex(&s).unwrap_or(default.snap_1_6);
        }
        if let Some(s) = get_string(obj, "1/8") {
            snap_colors.snap_1_8 = Rgba::from_hex(&s).unwrap_or(default.snap_1_8);
        }
        if let Some(s) = get_string(obj, "1/12") {
            snap_colors.snap_1_12 = Rgba::from_hex(&s).unwrap_or(default.snap_1_12);
        }
        if let Some(s) = get_string(obj, "1/16") {
            snap_colors.snap_1_16 = Rgba::from_hex(&s).unwrap_or(default.snap_1_16);
        }
        if let Some(s) = get_string(obj, "1/24") {
            snap_colors.snap_1_24 = Rgba::from_hex(&s).unwrap_or(default.snap_1_24);
        }
        if let Some(s) = get_string(obj, "1/48") {
            snap_colors.snap_1_48 = Rgba::from_hex(&s).unwrap_or(default.snap_1_48);
        }
        
        snap_colors
    }
    
    pub fn to_json(&self) -> JsonValue {
        let mut map = HashMap::new();
        map.insert("1/3".to_string(), JsonValue::from(self.snap_1_3.to_hex()));
        map.insert("1/4".to_string(), JsonValue::from(self.snap_1_4.to_hex()));
        map.insert("1/6".to_string(), JsonValue::from(self.snap_1_6.to_hex()));
        map.insert("1/8".to_string(), JsonValue::from(self.snap_1_8.to_hex()));
        map.insert("1/12".to_string(), JsonValue::from(self.snap_1_12.to_hex()));
        map.insert("1/16".to_string(), JsonValue::from(self.snap_1_16.to_hex()));
        map.insert("1/24".to_string(), JsonValue::from(self.snap_1_24.to_hex()));
        map.insert("1/48".to_string(), JsonValue::from(self.snap_1_48.to_hex()));
        JsonValue::from(map)
    }
}
