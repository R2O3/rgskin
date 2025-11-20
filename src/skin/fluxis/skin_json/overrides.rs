use indexmap::IndexMap;
use serde::de::{MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Clone, Debug, Default)]
pub struct Overrides {
    pub stage: StageOverrides,
    pub raw_overrides: IndexMap<String, String>,
}

#[derive(Clone, Debug, Default)]
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

impl Serialize for Overrides {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut entries: Vec<(&str, &str)> = Vec::new();
        let stage = &self.stage;

        macro_rules! add {
            ($key:expr, $val:expr) => {
                if !$val.is_empty() {
                    entries.push(($key, &$val));
                }
            };
        }

        add!("Health/foreground", stage.health_foreground);
        add!("Health/background", stage.health_background);
        add!("Stage/border-left", stage.border_left);
        add!("Stage/border-right", stage.border_right);
        add!("Stage/border-right-top", stage.border_right_top);
        add!("Stage/border-right-bottom", stage.border_right_bottom);
        add!("Stage/border-left-top", stage.border_left_top);
        add!("Stage/border-left-bottom", stage.border_left_bottom);
        add!("Stage/background-top", stage.background_top);
        add!("Stage/background-bottom", stage.background_bottom);
        add!("Stage/hitline", stage.hitline);
        add!("Lighting/column-lighting", stage.column_lighting);
        add!("Gameplay/fail-flash", stage.fail_flash);

        for (k, v) in &self.raw_overrides {
            entries.push((k.as_str(), v.as_str()));
        }

        entries.sort_by(|a, b| a.0.cmp(b.0));

        let mut map = serializer.serialize_map(Some(entries.len()))?;
        for (k, v) in entries {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Overrides {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OverridesVisitor;

        impl<'de> Visitor<'de> for OverridesVisitor {
            type Value = Overrides;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map of overrides")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut overrides = Overrides::default();

                while let Some((key, value)) = map.next_entry::<String, String>()? {
                    match key.as_str() {
                        "Health/foreground" => overrides.stage.health_foreground = value,
                        "Health/background" => overrides.stage.health_background = value,
                        "Stage/border-left" => overrides.stage.border_left = value,
                        "Stage/border-right" => overrides.stage.border_right = value,
                        "Stage/border-right-top" => overrides.stage.border_right_top = value,
                        "Stage/border-right-bottom" => overrides.stage.border_right_bottom = value,
                        "Stage/border-left-top" => overrides.stage.border_left_top = value,
                        "Stage/border-left-bottom" => overrides.stage.border_left_bottom = value,
                        "Stage/background-top" => overrides.stage.background_top = value,
                        "Stage/background-bottom" => overrides.stage.background_bottom = value,
                        "Stage/hitline" => overrides.stage.hitline = value,
                        "Lighting/column-lighting" => overrides.stage.column_lighting = value,
                        "Gameplay/fail-flash" => overrides.stage.fail_flash = value,
                        _ => {
                            overrides.raw_overrides.insert(key, value);
                        }
                    }
                }

                Ok(overrides)
            }
        }

        deserializer.deserialize_map(OverridesVisitor)
    }
}
