use indexmap::IndexMap;
use serde::de::{MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use crate::define_stage_overrides;
use crate::fluxis::skin_json::keymode::Keymodes;

pub(crate) fn extract_keymode_column(s: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() < 2 { return None; }

    let keymode_part = parts[0];
    if !keymode_part.ends_with('k') { return None; }

    let keymode = keymode_part[..keymode_part.len() - 1].parse::<usize>().ok()?;
    let column = parts[1].parse::<usize>().ok()?;

    Some((keymode, column))
}

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
        use std::collections::BTreeMap;

        let mut keymode_groups: BTreeMap<usize, Vec<(String, String)>> = BTreeMap::new();
        let mut non_keymode: Vec<(String, String)> = Vec::new();

        for (k, v) in &self.raw_overrides {
            if let Some(last) = k.rsplit('/').next() {
                if let Some((km, _col)) = extract_keymode_column(last) {
                    keymode_groups.entry(km).or_default().push((k.clone(), v.clone()));
                    continue;
                }
            }
            non_keymode.push((k.clone(), v.clone()));
        }

        let mut entries: Vec<(String, String)> = Vec::new();

        for (_km, group) in keymode_groups {
            let mut sorted_group = group;
            sorted_group.sort_by_key(|(k, _v)| {
                let parts: Vec<&str> = k.split('/').collect();
                let element = parts.get(0).unwrap_or(&"");
                let element_type = parts.get(1).unwrap_or(&"");
                
                Keymodes::order_by_type(element, element_type).unwrap_or(usize::MAX)
            });
            
            entries.extend(sorted_group);
        }

        non_keymode.sort_by(|a, b| a.0.cmp(&b.0));
        entries.extend(non_keymode);

        for (k, v) in self.stage.serialize() {
            entries.push((k.to_string(), v.to_string()));
        }

        let mut map = serializer.serialize_map(Some(entries.len()))?;
        for (k, v) in entries {
            map.serialize_entry(&k, &v)?;
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
