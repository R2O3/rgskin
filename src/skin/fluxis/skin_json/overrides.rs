use std::collections::HashMap;
use tinyjson::JsonValue;

use crate::utils::serde::json::get_string;

#[derive(Clone, Debug)]
pub struct Overrides {
    pub stage: StageOverrides,
    pub raw_overrides: HashMap<String, String>,
}

impl Default for Overrides {
    fn default() -> Self {
        Self {
            stage: StageOverrides::default(),
            raw_overrides: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StageOverrides {
    pub health_foreground: String,
    pub health_background: String,
    pub border_left: String,
    pub border_right: String,
    pub border_right_top: String,
    pub border_right_bottom: String,
    pub border_left_top: String,
    pub border_left_bottom: String,
    pub background_top: String,
    pub background_bottom: String,
    pub hitline: String,
    pub column_lighting: String,
    pub fail_flash: String,
}

impl Default for StageOverrides {
    fn default() -> Self {
        Self {
            health_foreground: String::new(),
            health_background: String::new(),
            border_left: String::new(),
            border_right: String::new(),
            border_right_top: String::new(),
            border_right_bottom: String::new(),
            border_left_top: String::new(),
            border_left_bottom: String::new(),
            background_top: String::new(),
            background_bottom: String::new(),
            hitline: String::new(),
            column_lighting: String::new(),
            fail_flash: String::new(),
        }
    }
}

impl Overrides {
    pub fn from_map(root: &HashMap<String, JsonValue>) -> Self {
        let mut overrides = Self::default();
        
        let Some(overrides_obj) = root.get("overrides")
            .and_then(|v| v.get::<HashMap<String, JsonValue>>()) else {
            return overrides;
        };
        
        for (key, value) in overrides_obj.iter() {
            let Some(value_str) = get_string(overrides_obj, key) else {
                continue;
            };
            
            overrides.raw_overrides.insert(key.clone(), value_str.clone());
            
            match key.as_str() {
                "Health/foreground" => overrides.stage.health_foreground = value_str,
                "Health/background" => overrides.stage.health_background = value_str,
                "Stage/border-left" => overrides.stage.border_left = value_str,
                "Stage/border-right" => overrides.stage.border_right = value_str,
                "Stage/border-right-top" => overrides.stage.border_right_top = value_str,
                "Stage/border-right-bottom" => overrides.stage.border_right_bottom = value_str,
                "Stage/border-left-top" => overrides.stage.border_left_top = value_str,
                "Stage/border-left-bottom" => overrides.stage.border_left_bottom = value_str,
                "Stage/background-top" => overrides.stage.background_top = value_str,
                "Stage/background-bottom" => overrides.stage.background_bottom = value_str,
                "Stage/hitline" => overrides.stage.hitline = value_str,
                "Lighting/column-lighting" => overrides.stage.column_lighting = value_str,
                "Gameplay/fail-flash" => overrides.stage.fail_flash = value_str,
                _ => {}
            }
        }
        
        overrides
    }
    
    pub fn to_json(&self) -> JsonValue {
        let mut map = HashMap::new();
        for (key, value) in &self.raw_overrides {
            map.insert(key.clone(), JsonValue::from(value.clone()));
        }
        JsonValue::from(map)
    }
}