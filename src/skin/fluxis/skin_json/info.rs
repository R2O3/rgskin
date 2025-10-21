use std::collections::HashMap;
use crate::utils::serde::json::get_string;
use tinyjson::JsonValue;

#[derive(Clone, Debug)]
pub struct Info {
    pub name: String,
    pub creator: String,
    pub accent: String,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            name: String::new(),
            creator: String::new(),
            accent: String::from("#FFFFFF"),
        }
    }
}

impl Info {
    pub fn from_map(obj: &HashMap<String, JsonValue>) -> Self {
        let mut info = Self::default();
        
        if let Some(info_obj) = obj.get("info").and_then(|v| v.get::<HashMap<String, JsonValue>>()) {
            if let Some(name) = get_string(info_obj, "name") {
                info.name = name;
            }
            if let Some(creator) = get_string(info_obj, "creator") {
                info.creator = creator;
            }
            if let Some(accent) = get_string(info_obj, "accent") {
                info.accent = accent;
            }
        }
        
        info
    }

    pub fn to_json(&self) -> JsonValue {
        let mut map = HashMap::new();
        map.insert("name".to_string(), JsonValue::from(self.name.clone()));
        map.insert("creator".to_string(), JsonValue::from(self.creator.clone()));
        map.insert("accent".to_string(), JsonValue::from(self.accent.clone()));
        JsonValue::from(map)
    }
}