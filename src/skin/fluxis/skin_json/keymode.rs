use std::collections::HashMap;

use tinyjson::JsonValue;

use crate::utils::serde::json::{get_bool, get_f64};

#[derive(Clone, Debug)]
pub struct Keymode {
    pub keymode: u8,
    pub column_width: u32,
    pub hit_position: i32,
    pub tint_notes: bool,
    pub tint_lns: bool,
    pub tint_receptors: bool,
    pub colors: Vec<String>,
    pub receptors_first: bool,
    pub receptor_offset: i32,
    
    pub receptor_images: Vec<String>,
    pub receptor_images_down: Vec<String>,
    pub normal_note_images: Vec<String>,
    pub long_note_body_images: Vec<String>,
    pub long_note_tail_images: Vec<String>,
    pub tick_images: Vec<String>,
    pub tick_images_small: Vec<String>,
}

impl Default for Keymode {
    fn default() -> Self {
        Self {
            keymode: 0,
            column_width: 150,
            hit_position: 35,
            tint_notes: false,
            tint_lns: false,
            tint_receptors: false,
            colors: Vec::new(),
            receptors_first: true,
            receptor_offset: 0,
            receptor_images: Vec::new(),
            receptor_images_down: Vec::new(),
            normal_note_images: Vec::new(),
            long_note_body_images: Vec::new(),
            long_note_tail_images: Vec::new(),
            tick_images: Vec::new(),
            tick_images_small: Vec::new(),
        }
    }
}

impl Keymode {
    pub fn from_map(obj: &HashMap<String, JsonValue>, keymode_num: u8) -> Self {
        let mut keymode = Self::default();
        keymode.keymode = keymode_num;
        
        if let Some(v) = get_f64(obj, "column_width") {
            keymode.column_width = v as u32;
        }
        if let Some(v) = get_f64(obj, "hit_position") {
            keymode.hit_position = v as i32;
        }
        if let Some(v) = get_bool(obj, "tint_notes") {
            keymode.tint_notes = v;
        }
        if let Some(v) = get_bool(obj, "tint_lns") {
            keymode.tint_lns = v;
        }
        if let Some(v) = get_bool(obj, "tint_receptors") {
            keymode.tint_receptors = v;
        }
        if let Some(v) = get_bool(obj, "receptors_first") {
            keymode.receptors_first = v;
        }
        if let Some(v) = get_f64(obj, "receptor_offset") {
            keymode.receptor_offset = v as i32;
        }
        
        if let Some(colors) = obj.get("colors").and_then(|v| v.get::<Vec<JsonValue>>()) {
            keymode.colors = colors.iter()
                .filter_map(|v| v.get::<String>().cloned())
                .collect();
        }
        
        keymode
    }

    pub fn to_json(&self) -> JsonValue {
        let mut map = HashMap::new();
        map.insert("column_width".to_string(), JsonValue::from(self.column_width as f64));
        map.insert("hit_position".to_string(), JsonValue::from(self.hit_position as f64));
        map.insert("tint_notes".to_string(), JsonValue::from(self.tint_notes));
        map.insert("tint_lns".to_string(), JsonValue::from(self.tint_lns));
        map.insert("tint_receptors".to_string(), JsonValue::from(self.tint_receptors));
        map.insert("receptors_first".to_string(), JsonValue::from(self.receptors_first));
        map.insert("receptor_offset".to_string(), JsonValue::from(self.receptor_offset as f64));
        
        let colors: Vec<JsonValue> = self.colors.iter()
            .map(|s| JsonValue::from(s.clone()))
            .collect();
        map.insert("colors".to_string(), JsonValue::from(colors));
        
        JsonValue::from(map)
    }
}
