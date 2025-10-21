use std::collections::HashMap;
use std::error::Error;

use tinyjson::JsonValue;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_json(json: &JsonValue) -> Result<Self, Box<dyn Error>> {
        let obj = json.get::<HashMap<String, JsonValue>>()
            .ok_or("Expected JSON object for position")?;
        
        let x = *obj.get("x")
            .and_then(|v| v.get::<f64>())
            .ok_or("Missing or invalid 'x' field")? as f32;
        
        let y = *obj.get("y")
            .and_then(|v| v.get::<f64>())
            .ok_or("Missing or invalid 'y' field")? as f32;
        
        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone)]
pub struct Component {
    pub position: Position,
    pub anchor: u8,
    pub origin: u8,
    pub scale: f32,
    pub anchor_to_playfield: bool,
    pub settings: ComponentSettings,
}

impl Component {
    pub fn new(
        position: Position,
        anchor: u8,
        origin: u8,
        scale: f32,
        anchor_to_playfield: bool,
        settings: ComponentSettings,
    ) -> Self {
        Self {
            position,
            anchor,
            origin,
            scale,
            anchor_to_playfield,
            settings,
        }
    }

    pub fn from_json(json: &JsonValue) -> Result<Self, &'static str> {
        let obj = json.get::<HashMap<String, JsonValue>>()
            .ok_or("Expected JSON object for component")?;
        
        let position = obj.get("position")
            .ok_or("Missing 'position' field")
            .and_then(|v| Position::from_json(v).map_err(|_| "Failed to parse position"))?;
        
        let anchor = *obj.get("anchor")
            .and_then(|v| v.get::<f64>())
            .ok_or("Missing or invalid 'anchor' field")? as u8;
        
        let origin = *obj.get("origin")
            .and_then(|v| v.get::<f64>())
            .ok_or("Missing or invalid 'origin' field")? as u8;
        
        let scale = *obj.get("scale")
            .and_then(|v| v.get::<f64>())
            .ok_or("Missing or invalid 'scale' field")? as f32;
        
        let anchor_to_playfield = *obj.get("anchor_to_playfield")
            .and_then(|v| v.get::<bool>())
            .ok_or("Missing or invalid 'anchor_to_playfield' field")?;
        
        let settings = obj.get("settings")
            .ok_or("Missing 'settings' field")
            .and_then(|v| ComponentSettings::from_json(v).map_err(|_| "Failed to parse settings"))?;
        
        Ok(Self {
            position,
            anchor,
            origin,
            scale,
            anchor_to_playfield,
            settings,
        })
    }

    pub fn to_json(&self) -> JsonValue {
        let mut map = std::collections::HashMap::new();
        
        let mut position_map = std::collections::HashMap::new();
        position_map.insert("x".to_string(), JsonValue::Number(self.position.x as f64));
        position_map.insert("y".to_string(), JsonValue::Number(self.position.y as f64));
        map.insert("position".to_string(), JsonValue::Object(position_map));
        
        map.insert("anchor".to_string(), JsonValue::Number(self.anchor as f64));
        map.insert("origin".to_string(), JsonValue::Number(self.origin as f64));
        map.insert("scale".to_string(), JsonValue::Number(self.scale as f64));
        map.insert("anchor_to_playfield".to_string(), JsonValue::Boolean(self.anchor_to_playfield));
        
        map.insert("settings".to_string(), self.settings.to_json());
        
        JsonValue::Object(map)
    }
}

#[derive(Debug, Clone)]
pub enum ComponentSettings {
    Empty,
    AttributeText {
        attribute_type: u32,
        text: Option<String>,
        size: f32,
        max_width: f32,
    },
    Custom(HashMap<String, SettingValue>),
}

impl ComponentSettings {
    pub fn empty() -> Self {
        ComponentSettings::Empty
    }
    
    pub fn attribute_text(
        attribute_type: u32,
        text: Option<String>,
        size: f32,
        max_width: f32,
    ) -> Self {
        ComponentSettings::AttributeText {
            attribute_type,
            text,
            size,
            max_width,
        }
    }
    
    pub fn custom(settings: HashMap<String, SettingValue>) -> Self {
        ComponentSettings::Custom(settings)
    }
}

impl ComponentSettings {
    pub fn from_json(json: &JsonValue) -> Result<Self, Box<dyn Error>> {
        let obj = json.get::<HashMap<String, JsonValue>>()
            .ok_or("Expected JSON object for settings")?;
        
        let settings_type = obj.get("type")
            .and_then(|v| v.get::<String>())
            .ok_or("Missing or invalid 'type' field in settings")?;
        
        match settings_type.as_str() {
            "empty" => Ok(ComponentSettings::Empty),
            
            "attribute_text" => {
                let attribute_type = *obj.get("attribute_type")
                    .and_then(|v| v.get::<f64>())
                    .ok_or("Missing or invalid 'attribute_type' field")? as u32;
                
                let text = obj.get("text")
                    .and_then(|v| match v {
                        JsonValue::String(s) => Some(s.clone()),
                        JsonValue::Null => None,
                        _ => None,
                    });
                
                let size = *obj.get("size")
                    .and_then(|v| v.get::<f64>())
                    .ok_or("Missing or invalid 'size' field")? as f32;
                
                let max_width = *obj.get("max_width")
                    .and_then(|v| v.get::<f64>())
                    .ok_or("Missing or invalid 'max_width' field")? as f32;
                
                Ok(ComponentSettings::AttributeText {
                    attribute_type,
                    text,
                    size,
                    max_width,
                })
            }
            
            "custom" => {
                let settings_obj = obj.get("settings")
                    .ok_or("Missing 'settings' field in custom settings")?
                    .get::<HashMap<String, JsonValue>>()
                    .ok_or("Expected object for custom settings")?;
                
                let mut settings = HashMap::new();
                for (key, value) in settings_obj {
                    let setting_value = SettingValue::from_json(value)?;
                    settings.insert(key.clone(), setting_value);
                }
                
                Ok(ComponentSettings::Custom(settings))
            }
            
            _ => Err("Unknown settings type".into())
        }
    }

    pub fn to_json(&self) -> JsonValue {
        match self {
            ComponentSettings::Empty => {
                let mut map = std::collections::HashMap::new();
                map.insert("type".to_string(), JsonValue::String("empty".to_string()));
                JsonValue::Object(map)
            }
            ComponentSettings::AttributeText { attribute_type, text, size, max_width } => {
                let mut map = std::collections::HashMap::new();
                map.insert("type".to_string(), JsonValue::String("attribute_text".to_string()));
                map.insert("attribute_type".to_string(), JsonValue::Number(*attribute_type as f64));
                map.insert("text".to_string(), match text {
                    Some(t) => JsonValue::String(t.clone()),
                    None => JsonValue::Null,
                });
                map.insert("size".to_string(), JsonValue::Number(*size as f64));
                map.insert("max_width".to_string(), JsonValue::Number(*max_width as f64));
                JsonValue::Object(map)
            }
            ComponentSettings::Custom(settings) => {
                let mut map = std::collections::HashMap::new();
                map.insert("type".to_string(), JsonValue::String("custom".to_string()));
                
                let mut settings_map = std::collections::HashMap::new();
                for (key, value) in settings {
                    settings_map.insert(key.clone(), value.to_json());
                }
                map.insert("settings".to_string(), JsonValue::Object(settings_map));
                
                JsonValue::Object(map)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SettingValue {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

impl SettingValue {
    pub fn from_json(json: &JsonValue) -> Result<Self, Box<dyn Error>> {
        match json {
            JsonValue::Number(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    Ok(SettingValue::Integer(*n as i64))
                } else {
                    Ok(SettingValue::Float(*n))
                }
            }
            JsonValue::String(s) => Ok(SettingValue::String(s.clone())),
            JsonValue::Boolean(b) => Ok(SettingValue::Bool(*b)),
            _ => Err("Invalid setting value type".into())
        }
    }

    pub fn to_json(&self) -> JsonValue {
        match self {
            SettingValue::Integer(i) => JsonValue::Number(*i as f64),
            SettingValue::Float(f) => JsonValue::Number(*f),
            SettingValue::String(s) => JsonValue::String(s.clone()),
            SettingValue::Bool(b) => JsonValue::Boolean(*b),
        }
    }
}
