use indexmap::IndexMap;
use serde::de::{MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use crate::define_overrides;
use crate::fluxis::skin_json::keymode::Keymodes;
use crate::fluxis::static_assets;

pub(crate) fn extract_keymode_column(s: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() < 2 { return None; }

    let keymode_part = parts[0];
    if !keymode_part.ends_with('k') { return None; }

    let keymode = keymode_part[..keymode_part.len() - 1].parse::<usize>().ok()?;
    let column = parts[1].parse::<usize>().ok()?;

    Some((keymode, column))
}

define_overrides!(
    HealthOverrides,
    (foreground, static_assets::Health::FOREGROUND),
    (background, static_assets::Health::BACKGROUND),
);

define_overrides!(
    StageOverrides,
    (border_left, static_assets::Stage::BORDER_LEFT),
    (border_right, static_assets::Stage::BORDER_RIGHT),
    (border_right_top, static_assets::Stage::BORDER_RIGHT_TOP),
    (border_right_bottom, static_assets::Stage::BORDER_RIGHT_BOTTOM),
    (border_left_top, static_assets::Stage::BORDER_LEFT_TOP),
    (border_left_bottom, static_assets::Stage::BORDER_LEFT_BOTTOM),
    (background, static_assets::Stage::BACKGROUND),
    (background_top, static_assets::Stage::BACKGROUND_TOP),
    (background_bottom, static_assets::Stage::BACKGROUND_BOTTOM),
    (hitline, static_assets::Stage::HITLINE),
);

define_overrides!(
    JudgementOverrides,
    (miss, static_assets::Judgement::MISS),
    (okay, static_assets::Judgement::OKAY),
    (alright, static_assets::Judgement::ALRIGHT),
    (great, static_assets::Judgement::GREAT),
    (perfect, static_assets::Judgement::PERFECT),
    (flawless, static_assets::Judgement::FLAWLESS),
);

define_overrides!(
    LightingOverrides,
    (column_lighting, static_assets::Lighting::COLUMN_LIGHTING),
);

define_overrides!(
    GameplayOverrides,
    (fail_flash, static_assets::Gameplay::FAIL_FLASH),
);

#[derive(Clone, Debug, Default)]
pub struct Overrides {
    pub health: HealthOverrides,
    pub stage: StageOverrides,
    pub judgement: JudgementOverrides,
    pub lighting: LightingOverrides,
    pub gameplay: GameplayOverrides,
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

        for (k, v) in self.health.serialize() {
            entries.push((k.to_string(), v.to_string()));
        }
        for (k, v) in self.stage.serialize() {
            entries.push((k.to_string(), v.to_string()));
        }
        for (k, v) in self.judgement.serialize() {
            entries.push((k.to_string(), v.to_string()));
        }
        for (k, v) in self.lighting.serialize() {
            entries.push((k.to_string(), v.to_string()));
        }
        for (k, v) in self.gameplay.serialize() {
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
                    if !overrides.health.set_field(&key, value.clone())
                        && !overrides.stage.set_field(&key, value.clone())
                        && !overrides.judgement.set_field(&key, value.clone())
                        && !overrides.lighting.set_field(&key, value.clone())
                        && !overrides.gameplay.set_field(&key, value.clone())
                    {
                        overrides.raw_overrides.insert(key, value);
                    }
                }

                Ok(overrides)
            }
        }

        deserializer.deserialize_map(OverridesVisitor)
    }
}
