use std::collections::HashMap;
use tinyjson::JsonValue;
use crate::skin::fluxis::layout_json::component::{Component, ComponentSettings, Position, SettingValue};

#[derive(Debug, Clone)]
pub struct Gameplay {
    pub components: HashMap<String, Component>,
}

impl Gameplay {
    pub fn from_json(json: &JsonValue) -> Result<Self, &'static str> {
        let obj = json.get::<HashMap<String, JsonValue>>()
            .ok_or("Expected JSON object for gameplay")?;
        
        let components_json = obj.get("components")
            .ok_or("Missing 'components' field")?
            .get::<HashMap<String, JsonValue>>()
            .ok_or("Expected object for components")?;
        
        let mut components = HashMap::new();
        
        for (key, value) in components_json {
            match Component::from_json(value) {
                Ok(component) => {
                    components.insert(key.clone(), component);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse component '{}': {}", key, e);
                }
            }
        }
        
        Ok(Self { components })
    }

    pub fn to_json(&self) -> JsonValue {        
        let mut components_map = std::collections::HashMap::new();
        for (name, component) in &self.components {
            components_map.insert(name.clone(), component.to_json());
        }
        
        JsonValue::Object(components_map)
    }
}

#[derive(Debug, Clone)]
pub struct Accuracy {
    pub component: Component,
}

#[derive(Debug, Clone)]
pub struct AttributeText {
    pub component: Component,
    pub attribute_type: u32,
    pub text: Option<String>,
    pub size: f32,
    pub max_width: f32,
}

#[derive(Debug, Clone)]
pub struct Combo {
    pub component: Component,
    pub scale_additive: bool,
}

#[derive(Debug, Clone)]
pub struct PerformanceRating {
    pub component: Component,
    pub suffix: bool,
    pub decimals: bool,
}

#[derive(Debug, Clone)]
pub struct KeysPerSecond {
    pub component: Component,
    pub suffix: bool,
}

#[derive(Debug, Clone)]
pub struct Health {
    pub component: Component,
}

#[derive(Debug, Clone)]
pub struct HitError {
    pub component: Component,
}

#[derive(Debug, Clone)]
pub struct Judgement {
    pub component: Component,
}

#[derive(Debug, Clone)]
pub struct JudgementCounter {
    pub component: Component,
}

#[derive(Debug, Clone)]
pub struct Progress {
    pub component: Component,
}

impl Accuracy {
    pub fn new(position: Position, anchor: u8, origin: u8, anchor_to_playfield: bool) -> Self {
        Self {
            component: Component::new(
                position,
                anchor,
                origin,
                1.0,
                anchor_to_playfield,
                ComponentSettings::empty(),
            ),
        }
    }
    
    pub fn default() -> Self {
        Self::new(Position::new(0.0, 0.0), 18, 17, true)
    }
}

impl AttributeText {
    pub fn new(
        position: Position,
        anchor: u8,
        origin: u8,
        attribute_type: u32,
        text: Option<String>,
        size: f32,
        max_width: f32,
    ) -> Self {
        Self {
            component: Component::new(
                position,
                anchor,
                origin,
                1.0,
                false,
                ComponentSettings::attribute_text(attribute_type, text.clone(), size, max_width),
            ),
            attribute_type,
            text,
            size,
            max_width,
        }
    }
    
    pub fn title() -> Self {
        Self::new(Position::new(20.0, -10.0), 12, 12, 0, None, 32.0, 512.0)
    }
    
    pub fn artist() -> Self {
        Self::new(
            Position::new(20.0, -52.0),
            12,
            12,
            1,
            Some("by {value}".to_string()),
            24.0,
            512.0,
        )
    }
    
    pub fn difficulty() -> Self {
        Self::new(Position::new(-20.0, -10.0), 36, 36, 2, None, 32.0, 512.0)
    }
    
    pub fn mapper() -> Self {
        Self::new(
            Position::new(-20.0, -50.0),
            36,
            36,
            3,
            Some("mapped by {value}".to_string()),
            24.0,
            512.0,
        )
    }
}

impl Combo {
    pub fn new(position: Position, anchor: u8, origin: u8, anchor_to_playfield: bool, scale_additive: bool) -> Self {
        let mut settings = HashMap::new();
        settings.insert("scale-additive".to_string(), SettingValue::Bool(scale_additive));
        
        Self {
            component: Component::new(
                position,
                anchor,
                origin,
                1.0,
                anchor_to_playfield,
                ComponentSettings::custom(settings),
            ),
            scale_additive,
        }
    }
    
    pub fn default() -> Self {
        Self::new(Position::new(0.0, -32.0), 18, 18, true, true)
    }
}

impl PerformanceRating {
    pub fn new(position: Position, anchor: u8, origin: u8, anchor_to_playfield: bool, suffix: bool, decimals: bool) -> Self {
        let mut settings = HashMap::new();
        settings.insert("suffix".to_string(), SettingValue::Bool(suffix));
        settings.insert("decimals".to_string(), SettingValue::Bool(decimals));
        
        Self {
            component: Component::new(
                position,
                anchor,
                origin,
                1.0,
                anchor_to_playfield,
                ComponentSettings::custom(settings),
            ),
            suffix,
            decimals,
        }
    }
    
    pub fn default() -> Self {
        Self::new(Position::new(0.0, 15.0), 18, 18, false, true, false)
    }
}

impl KeysPerSecond {
    pub fn new(position: Position, anchor: u8, origin: u8, anchor_to_playfield: bool, suffix: bool) -> Self {
        let mut settings = HashMap::new();
        settings.insert("suffix".to_string(), SettingValue::Bool(suffix));
        
        Self {
            component: Component::new(
                position,
                anchor,
                origin,
                1.0,
                anchor_to_playfield,
                ComponentSettings::custom(settings),
            ),
            suffix,
        }
    }
    
    pub fn default() -> Self {
        Self::new(Position::new(0.0, 105.0), 18, 18, false, true)
    }
}

impl Health {
    pub fn new(position: Position, anchor: u8, origin: u8, anchor_to_playfield: bool) -> Self {
        Self {
            component: Component::new(
                position,
                anchor,
                origin,
                1.0,
                anchor_to_playfield,
                ComponentSettings::empty(),
            ),
        }
    }
    
    pub fn default() -> Self {
        Self::new(Position::new(20.0, -40.0), 36, 12, true)
    }
}

impl HitError {
    pub fn new(position: Position, anchor: u8, origin: u8, anchor_to_playfield: bool) -> Self {
        Self {
            component: Component::new(
                position,
                anchor,
                origin,
                1.0,
                anchor_to_playfield,
                ComponentSettings::empty(),
            ),
        }
    }
    
    pub fn default() -> Self {
        Self::new(Position::new(0.0, 50.0), 18, 17, true)
    }
}

impl Judgement {
    pub fn new(position: Position, anchor: u8, origin: u8, anchor_to_playfield: bool) -> Self {
        Self {
            component: Component::new(
                position,
                anchor,
                origin,
                1.0,
                anchor_to_playfield,
                ComponentSettings::empty(),
            ),
        }
    }
    
    pub fn default() -> Self {
        Self::new(Position::new(0.0, 150.0), 18, 18, true)
    }
}

impl JudgementCounter {
    pub fn new(position: Position, anchor: u8, origin: u8, anchor_to_playfield: bool) -> Self {
        Self {
            component: Component::new(
                position,
                anchor,
                origin,
                1.0,
                anchor_to_playfield,
                ComponentSettings::empty(),
            ),
        }
    }
    
    pub fn default() -> Self {
        Self::new(Position::new(-20.0, 0.0), 34, 34, false)
    }
}

impl Progress {
    pub fn new(position: Position, anchor: u8, origin: u8, anchor_to_playfield: bool) -> Self {
        Self {
            component: Component::new(
                position,
                anchor,
                origin,
                1.0,
                anchor_to_playfield,
                ComponentSettings::empty(),
            ),
        }
    }
    
    pub fn default() -> Self {
        Self::new(Position::new(0.0, 0.0), 9, 9, false)
    }
}
