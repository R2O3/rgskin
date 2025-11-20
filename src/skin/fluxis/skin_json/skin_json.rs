use serde::{Deserialize, Serialize};

use crate::{skin::fluxis::skin_json::{
    colors::{JudgementColors, SnapColors}, 
    info::Info, 
    keymode::Keymode, 
    overrides::Overrides
}, utils::serde::set_vec_element};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SkinJson {
    #[serde(default)]
    pub info: Info,
    
    #[serde(default)]
    pub judgements: JudgementColors,
    
    #[serde(rename = "snap-colors", default)]
    pub snap_colors: SnapColors,
    
    #[serde(default)]
    pub overrides: Overrides,

    #[serde(skip)]
    pub keymodes: Vec<Keymode>,
}

impl SkinJson {
    pub fn from_str(json_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut skin: SkinJson = serde_json::from_str(json_str)?;

        skin.parse_keymodes_from_overrides();

        Ok(skin)
    }

    pub fn to_str(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    pub fn parse_keymodes_from_overrides(&mut self) {
        self.keymodes.clear();
        for k in 1..=10 {
            let mut km = Keymode::default();
            km.keymode = k;
            Self::alloc_vecs(&mut km, k as usize);
            self.keymodes.push(km);
        }

        let raw_overrides = &self.overrides.raw_overrides;
        let keymodes = &mut self.keymodes;

        for (key, value) in raw_overrides {
            Self::parse_single_override(keymodes, key, value);
        }
    }

    fn parse_single_override(keymodes: &mut Vec<Keymode>, key: &str, value: &str) {
        let parts: Vec<&str> = key.split('/').collect();
        if parts.len() < 2 { return; }

        let category = parts[0];
        let identifier = parts.last().unwrap();

        if let Some((keymode_num, column_str)) = Self::extract_keymode_column(identifier) {
            if let Some(keymode) = keymodes.iter_mut().find(|km| km.keymode == keymode_num as u8) {
                let col_idx = column_str.saturating_sub(1);
                let sub_cat = if parts.len() > 2 { parts[1] } else { "" };

                match (category, sub_cat) {
                    ("Receptor", _) => {
                        if identifier.ends_with("-up") {
                            set_vec_element(&mut keymode.receptor_images, col_idx, value);
                        } else if identifier.ends_with("-down") {
                            set_vec_element(&mut keymode.receptor_images_down, col_idx, value);
                        }
                    },
                    ("HitObjects", "Note") => {
                        set_vec_element(&mut keymode.normal_note_images, col_idx, value);
                    },
                    ("HitObjects", "LongNoteStart") => {
                        set_vec_element(&mut keymode.long_note_head_images, col_idx, value);
                    },
                    ("HitObjects", "LongNoteBody") => {
                        set_vec_element(&mut keymode.long_note_body_images, col_idx, value);
                    },
                    ("HitObjects", "LongNoteEnd") => {
                        set_vec_element(&mut keymode.long_note_tail_images, col_idx, value);
                    },
                    ("HitObjects", "Tick") => {
                        if identifier.ends_with("-small") {
                            set_vec_element(&mut keymode.tick_images_small, col_idx, value);
                        } else {
                            set_vec_element(&mut keymode.tick_images, col_idx, value);
                        }
                    },
                    _ => {}
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

    pub fn sync_overrides_from_keymodes(&mut self) {
        for keymode in &self.keymodes {
            let k = keymode.keymode;
            
            let mut insert = |vec: &Vec<String>, prefix: &str, suffix: &str| {
                for (col_idx, img) in vec.iter().enumerate() {
                    if !img.is_empty() {
                        let key = format!("{}/{}k-{}{}", prefix, k, col_idx + 1, suffix);
                        self.overrides.raw_overrides.insert(key, img.clone());
                    }
                }
            };

            insert(&keymode.receptor_images, "Receptor", "-up");
            insert(&keymode.receptor_images_down, "Receptor", "-down");
            insert(&keymode.normal_note_images, "HitObjects/Note", "");
            insert(&keymode.long_note_head_images, "HitObjects/LongNoteStart", "");
            insert(&keymode.long_note_body_images, "HitObjects/LongNoteBody", "");
            insert(&keymode.long_note_tail_images, "HitObjects/LongNoteEnd", "");
            insert(&keymode.tick_images, "HitObjects/Tick", "");
            insert(&keymode.tick_images_small, "HitObjects/Tick", "-small");
        }
    }

    fn alloc_vecs(keymode: &mut Keymode, k: usize) {
        let resize = |v: &mut Vec<String>| {
            if v.len() != k {
                v.resize(k, String::new());
            }
        };

        resize(&mut keymode.receptor_images);
        resize(&mut keymode.receptor_images_down);
        resize(&mut keymode.normal_note_images);
        resize(&mut keymode.long_note_head_images);
        resize(&mut keymode.long_note_body_images);
        resize(&mut keymode.long_note_tail_images);
        resize(&mut keymode.tick_images);
        resize(&mut keymode.tick_images_small);
    }

    fn extract_keymode_column(s: &str) -> Option<(usize, usize)> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() < 2 { return None; }

        let keymode_part = parts[0];
        if !keymode_part.ends_with('k') { return None; }

        let keymode = keymode_part.trim_end_matches('k').parse::<usize>().ok()?;
        let column = parts[1].parse::<usize>().ok()?;
        
        Some((keymode, column))
    }
}
