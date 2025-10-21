use std::collections::HashMap;
use tinyjson::JsonValue;

use crate::{skin::fluxis::skin_json::{
    colors::{JudgementColors, SnapColors}, 
    info::Info, 
    keymode::Keymode, 
    overrides::Overrides
}, utils::serde::json::set_vec_element};

#[derive(Clone, Debug, Default)]
pub struct SkinJson {
    pub info: Info,
    pub keymodes: Vec<Keymode>,
    pub judgements: JudgementColors,
    pub snap_colors: SnapColors,
    pub overrides: Overrides,
}

impl SkinJson {
    pub fn from_str(str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let parsed: JsonValue = str.parse()?;
        let root: &HashMap<String, JsonValue> = parsed.get().ok_or("Invalid JSON root")?;
        
        let mut skin = SkinJson::default();

        skin.info = Info::from_map(root);
        skin.judgements = JudgementColors::from_map(root);
        skin.snap_colors = SnapColors::from_map(root);
        skin.overrides = Overrides::from_map(root);
        Self::parse_keymodes(root, &mut skin);

        skin.sync_keymodes_from_overrides();

        Ok(skin)
    }

    pub fn sync_overrides_from_keymodes(&mut self) {
        for keymode in &self.keymodes {
            let k = keymode.keymode;
            
            for (col_idx, img) in keymode.receptor_images.iter().enumerate() {
                if !img.is_empty() {
                    let key = format!("Receptor/{}-{}-up", k, col_idx + 1);
                    self.overrides.raw_overrides.insert(key, img.clone());
                }
            }
            
            for (col_idx, img) in keymode.receptor_images_down.iter().enumerate() {
                if !img.is_empty() {
                    let key = format!("Receptor/{}-{}-down", k, col_idx + 1);
                    self.overrides.raw_overrides.insert(key, img.clone());
                }
            }
            
            for (col_idx, img) in keymode.normal_note_images.iter().enumerate() {
                if !img.is_empty() {
                    let key = format!("HitObjects/Note/{}-{}", k, col_idx + 1);
                    self.overrides.raw_overrides.insert(key, img.clone());
                }
            }
            
            for (col_idx, img) in keymode.long_note_body_images.iter().enumerate() {
                if !img.is_empty() {
                    let key = format!("HitObjects/LongNoteBody/{}-{}", k, col_idx + 1);
                    self.overrides.raw_overrides.insert(key, img.clone());
                }
            }
            
            for (col_idx, img) in keymode.long_note_tail_images.iter().enumerate() {
                if !img.is_empty() {
                    let key = format!("HitObjects/LongNoteEnd/{}-{}", k, col_idx + 1);
                    self.overrides.raw_overrides.insert(key, img.clone());
                }
            }
            
            for (col_idx, img) in keymode.tick_images.iter().enumerate() {
                if !img.is_empty() {
                    let key = format!("HitObjects/Tick/{}-{}", k, col_idx + 1);
                    self.overrides.raw_overrides.insert(key, img.clone());
                }
            }
            
            for (col_idx, img) in keymode.tick_images_small.iter().enumerate() {
                if !img.is_empty() {
                    let key = format!("HitObjects/Tick/{}-{}-small", k, col_idx + 1);
                    self.overrides.raw_overrides.insert(key, img.clone());
                }
            }
        }
    }

    pub fn sync_overrides_from_stage(&mut self) {
        self.overrides.raw_overrides.insert("health_foreground".to_string(), self.overrides.stage.health_foreground.clone());
        self.overrides.raw_overrides.insert("health_background".to_string(), self.overrides.stage.health_background.clone());
        self.overrides.raw_overrides.insert("border_left".to_string(), self.overrides.stage.border_left.clone());
        self.overrides.raw_overrides.insert("border_right".to_string(), self.overrides.stage.border_right.clone());
        self.overrides.raw_overrides.insert("border_right_top".to_string(), self.overrides.stage.border_right_top.clone());
        self.overrides.raw_overrides.insert("border_right_bottom".to_string(), self.overrides.stage.border_right_bottom.clone());
        self.overrides.raw_overrides.insert("border_left_top".to_string(), self.overrides.stage.border_left_top.clone());
        self.overrides.raw_overrides.insert("border_left_bottom".to_string(), self.overrides.stage.border_left_bottom.clone());
        self.overrides.raw_overrides.insert("background_top".to_string(), self.overrides.stage.background_top.clone());
        self.overrides.raw_overrides.insert("background_bottom".to_string(), self.overrides.stage.background_bottom.clone());
        self.overrides.raw_overrides.insert("hitline".to_string(), self.overrides.stage.hitline.clone());
        self.overrides.raw_overrides.insert("column_lighting".to_string(), self.overrides.stage.column_lighting.clone());
        self.overrides.raw_overrides.insert("fail_flash".to_string(), self.overrides.stage.fail_flash.clone());
    }

    pub fn sync_stage_from_overrides(&mut self) {
        if let Some(value) = self.overrides.raw_overrides.get("health_foreground") {
            self.overrides.stage.health_foreground = value.clone();
        }
        if let Some(value) = self.overrides.raw_overrides.get("health_background") {
            self.overrides.stage.health_background = value.clone();
        }
        if let Some(value) = self.overrides.raw_overrides.get("border_left") {
            self.overrides.stage.border_left = value.clone();
        }
        if let Some(value) = self.overrides.raw_overrides.get("border_right") {
            self.overrides.stage.border_right = value.clone();
        }
        if let Some(value) = self.overrides.raw_overrides.get("border_right_top") {
            self.overrides.stage.border_right_top = value.clone();
        }
        if let Some(value) = self.overrides.raw_overrides.get("border_right_bottom") {
            self.overrides.stage.border_right_bottom = value.clone();
        }
        if let Some(value) = self.overrides.raw_overrides.get("border_left_top") {
            self.overrides.stage.border_left_top = value.clone();
        }
        if let Some(value) = self.overrides.raw_overrides.get("border_left_bottom") {
            self.overrides.stage.border_left_bottom = value.clone();
        }
        if let Some(value) = self.overrides.raw_overrides.get("background_top") {
            self.overrides.stage.background_top = value.clone();
        }
        if let Some(value) = self.overrides.raw_overrides.get("background_bottom") {
            self.overrides.stage.background_bottom = value.clone();
        }
        if let Some(value) = self.overrides.raw_overrides.get("hitline") {
            self.overrides.stage.hitline = value.clone();
        }
        if let Some(value) = self.overrides.raw_overrides.get("column_lighting") {
            self.overrides.stage.column_lighting = value.clone();
        }
        if let Some(value) = self.overrides.raw_overrides.get("fail_flash") {
            self.overrides.stage.fail_flash = value.clone();
        }
    }

    pub fn sync_keymodes_from_overrides(&mut self) {
        let overrides: Vec<(String, String)> = self.overrides.raw_overrides
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        
        for (key, value) in overrides {
            Self::parse_keymode_override(self, &key, &value);
        }
    }

    fn parse_keymodes(root: &HashMap<String, JsonValue>, skin: &mut SkinJson) {
        for k in 1..=10 {
            let key = format!("{}k", k);
            if let Some(obj) = root.get(&key).and_then(|v| v.get::<HashMap<String, JsonValue>>()) {
                let mut keymode = Keymode::from_map(obj, k as u8);
                
                Self::alloc_vecs(&mut keymode, k);
                
                skin.keymodes.push(keymode);
            }
        }
    }

    fn alloc_vecs(keymode: &mut Keymode, k: usize) {
        keymode.receptor_images = vec![String::new(); k];
        keymode.receptor_images_down = vec![String::new(); k];
        keymode.normal_note_images = vec![String::new(); k];
        keymode.long_note_body_images = vec![String::new(); k];
        keymode.long_note_tail_images = vec![String::new(); k];
        keymode.tick_images = vec![String::new(); k];
        keymode.tick_images_small = vec![String::new(); k];
    }

    fn parse_keymode_override(skin: &mut SkinJson, key: &str, value: &str) {
        let parts: Vec<&str> = key.split('/').collect();
        if parts.len() < 2 {
            return;
        }

        let category = parts[0];
        let subcategory = if parts.len() > 2 { parts[1] } else { "" };
        let identifier = parts[parts.len() - 1];

        if let Some((keymode_num, column_str)) = Self::extract_keymode_column(identifier) {
            if let Some(keymode) = skin.keymodes.iter_mut().find(|km| km.keymode == keymode_num as u8) {
                let col_idx = column_str.saturating_sub(1);
                
                match (category, subcategory) {
                    ("Receptor", _) if identifier.ends_with("-up") => {
                        set_vec_element(&mut keymode.receptor_images, col_idx, value);
                    }
                    ("Receptor", _) if identifier.ends_with("-down") => {
                        set_vec_element(&mut keymode.receptor_images_down, col_idx, value);
                    }
                    ("HitObjects", "Note") => {
                        set_vec_element(&mut keymode.normal_note_images, col_idx, value);
                    }
                    ("HitObjects", "LongNoteBody") => {
                        set_vec_element(&mut keymode.long_note_body_images, col_idx, value);
                    }
                    ("HitObjects", "LongNoteEnd") => {
                        set_vec_element(&mut keymode.long_note_tail_images, col_idx, value);
                    }
                    ("HitObjects", "Tick") if identifier.ends_with("-small") => {
                        set_vec_element(&mut keymode.tick_images_small, col_idx, value);
                    }
                    ("HitObjects", "Tick") => {
                        set_vec_element(&mut keymode.tick_images, col_idx, value);
                    }
                    _ => {}
                }
            }
        }
    }

    fn extract_keymode_column(s: &str) -> Option<(usize, usize)> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() < 2 {
            return None;
        }

        let keymode_part = parts[0];
        if !keymode_part.ends_with('k') {
            return None;
        }

        let keymode = keymode_part.trim_end_matches('k').parse::<usize>().ok()?;
        let column = parts[1].parse::<usize>().ok()?;
        
        Some((keymode, column))
    }

    pub fn to_str(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut root = HashMap::new();

        root.insert("info".to_string(), self.info.to_json());
        
        for keymode in &self.keymodes {
            let key = format!("{}k", keymode.keymode);
            root.insert(key, keymode.to_json());
        }

        root.insert("judgements".to_string(), self.judgements.to_json());
        root.insert("snap-colors".to_string(), self.snap_colors.to_json());
        root.insert("overrides".to_string(), self.overrides.to_json());

        let json_val = JsonValue::from(root);
        json_val.format().map_err(|e| e.into())
    }
}