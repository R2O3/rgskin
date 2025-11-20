use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "x")]
    pub x: f32,
    #[serde(rename = "y")]
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "Anchor")]
    pub anchor: u8,
    #[serde(rename = "Origin")]
    pub origin: u8,
    #[serde(rename = "Scale")]
    pub scale: f32,
    #[serde(rename = "AnchorToPlayfield")]
    pub anchor_to_playfield: bool,
    #[serde(rename = "Settings")]
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComponentSettings {
    Empty(HashMap<String, serde_json::Value>),
    
    AttributeText {
        #[serde(rename = "type")]
        attribute_type: u32,
        text: Option<String>,
        size: f32,
        #[serde(rename = "max-width")]
        max_width: f32,
    },
    
    Custom(HashMap<String, SettingValue>),
}

impl ComponentSettings {
    pub fn empty() -> Self {
        ComponentSettings::Empty(HashMap::new())
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SettingValue {
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
}
