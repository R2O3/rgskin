use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::skin::fluxis::layout_json::component::{Component, ComponentSettings, Position, SettingValue};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Gameplay {
    pub components: HashMap<String, Component>,
}

macro_rules! component {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub component: Component,
        }

        impl $name {
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
        }
    };
}

macro_rules! component_with_settings {
    ($name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub component: Component,
            $(pub $field: $type,)*
        }

        impl $name {
            pub fn new(
                position: Position,
                anchor: u8,
                origin: u8,
                anchor_to_playfield: bool,
                $($field: $type,)*
            ) -> Self {
                let mut settings = HashMap::new();
                $(
                    settings.insert(
                        stringify!($field).replace('_', "-"),
                        SettingValue::Bool($field)
                    );
                )*
                
                Self {
                    component: Component::new(
                        position,
                        anchor,
                        origin,
                        1.0,
                        anchor_to_playfield,
                        ComponentSettings::custom(settings),
                    ),
                    $($field,)*
                }
            }
        }
    };
}

component!(Accuracy);
component!(Health);
component!(HitError);
component!(Judgement);
component!(JudgementCounter);
component!(Progress);

component_with_settings!(Combo {
    scale_additive: bool,
});

component_with_settings!(PerformanceRating {
    suffix: bool,
    decimals: bool,
});

component_with_settings!(KeysPerSecond {
    suffix: bool,
});

#[derive(Debug, Clone)]
pub struct AttributeText {
    pub component: Component,
    pub attribute_type: u32,
    pub text: Option<String>,
    pub size: f32,
    pub max_width: f32,
}

impl Accuracy {
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
    pub fn default() -> Self {
        Self::new(Position::new(0.0, -32.0), 18, 18, true, true)
    }
}

impl PerformanceRating {
    pub fn default() -> Self {
        Self::new(Position::new(0.0, 15.0), 18, 18, false, true, false)
    }
}

impl KeysPerSecond {
    pub fn default() -> Self {
        Self::new(Position::new(0.0, 105.0), 18, 18, false, true)
    }
}

impl Health {
    pub fn default() -> Self {
        Self::new(Position::new(20.0, -40.0), 36, 12, true)
    }
}

impl HitError {
    pub fn default() -> Self {
        Self::new(Position::new(0.0, 50.0), 18, 17, true)
    }
}

impl Judgement {
    pub fn default() -> Self {
        Self::new(Position::new(0.0, 150.0), 18, 18, true)
    }
}

impl JudgementCounter {
    pub fn default() -> Self {
        Self::new(Position::new(-20.0, 0.0), 34, 34, false)
    }
}

impl Progress {
    pub fn default() -> Self {
        Self::new(Position::new(0.0, 0.0), 9, 9, false)
    }
}
