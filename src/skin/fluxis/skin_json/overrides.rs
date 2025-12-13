use indexmap::IndexMap;
use serde::de::{MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::define_stage_overrides;

define_stage_overrides!(
    (health_foreground, "Health/foreground"),
    (health_background, "Health/background"),
    (border_left, "Stage/border-left"),
    (border_right, "Stage/border-right"),
    (border_right_top, "Stage/border-right-top"),
    (border_right_bottom, "Stage/border-right-bottom"),
    (border_left_top, "Stage/border-left-top"),
    (border_left_bottom, "Stage/border-left-bottom"),
    (background_top, "Stage/background-top"),
    (background_bottom, "Stage/background-bottom"),
    (hitline, "Stage/hitline"),
    (column_lighting, "Lighting/column-lighting"),
    (fail_flash, "Gameplay/fail-flash"),
);

#[derive(Clone, Debug, Default)]
pub struct Overrides {
    pub stage: StageOverrides,
    pub raw_overrides: IndexMap<String, String>,
}

impl Serialize for Overrides {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut entries = self.stage.serialize();

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
                    if !overrides.stage.set_field(&key, value.clone()) {
                        overrides.raw_overrides.insert(key, value);
                    }
                }

                Ok(overrides)
            }
        }

        deserializer.deserialize_map(OverridesVisitor)
    }
}
