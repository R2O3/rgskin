use indexmap::IndexMap;
use crate::{
    fluxis::skin_json::{
        colors::{JudgementColors, SnapColors},
        info::Info,
        keymode::{Keymode, Keymodes},
        overrides::Overrides,
    },
    utils::serde::set_vec_element,
};
use serde::{
    ser::Serializer,
    Deserialize, Serialize,
};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SkinJson {
    #[serde(default)]
    pub info: Info,
    
    #[serde(default)]
    pub judgements: JudgementColors,
    
    #[serde(rename = "snap-colors", default)]
    pub snap_colors: SnapColors,
    
    #[serde(default)]
    pub overrides: Overrides,

    #[serde(skip)]
    pub keymodes: Vec<Keymode>,
}

impl Serialize for SkinJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let keymode_map: IndexMap<String, &Keymode> = self
            .keymodes
            .iter()
            .map(|km| (format!("{}k", km.keymode), km))
            .collect();

        #[derive(Serialize)]
        struct SkinJsonSerialized<'a> {
            info: &'a Info,

            #[serde(flatten)]
            keymode_map: &'a IndexMap<String, &'a Keymode>,

            judgements: &'a JudgementColors,

            #[serde(rename = "snap-colors")]
            snap_colors: &'a SnapColors,

            overrides: &'a Overrides,
        }

        let temp = SkinJsonSerialized {
            info: &self.info,
            judgements: &self.judgements,
            snap_colors: &self.snap_colors,
            keymode_map: &keymode_map,
            overrides: &self.overrides,
        };

        temp.serialize(serializer)
    }
}

impl SkinJson {
    pub fn from_str(json_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut skin: SkinJson = serde_json::from_str(json_str)?;
        skin.parse_keymodes_from_overrides();
        Ok(skin)
    }

    pub fn to_str(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    pub fn parse_keymodes_from_overrides(&mut self) {
        self.keymodes.clear();
        for k in 1..=10 {
            let mut km = Keymode::default();
            km.keymode = k;
            Self::alloc_vecs(&mut km, k as usize);
            self.keymodes.push(km);
        }

        let raw_overrides = &self.overrides.raw_overrides;
        let keymodes = &mut self.keymodes;

        for (key, value) in raw_overrides {
            Self::parse_single_override(keymodes, key, value);
        }
    }

    fn parse_single_override(keymodes: &mut Vec<Keymode>, key: &str, value: &str) {
        let parts: Vec<&str> = key.split('/').collect();
        if parts.len() < 2 { return; }

        let element = parts[0];
        let identifier = parts.last().unwrap();
        let element_type = if parts.len() > 2 { parts[1] } else { "" };

        if let Some((keymode_num, column_str)) = Self::extract_keymode_column(identifier) {
            if let Some(keymode) = keymodes.iter_mut().find(|km| km.keymode == keymode_num as u8) {
                let col_idx = column_str.saturating_sub(1);
                
                let suffix = if identifier.ends_with("-up") {
                    "-up"
                } else if identifier.ends_with("-down") {
                    "-down"
                } else if identifier.ends_with("-small") {
                    "-small"
                } else {
                    ""
                };

                if let Some(field) = Keymodes::get_field_mut(keymode, element, element_type, suffix) {
                    set_vec_element(field, col_idx, value);
                }
            }
        }
    }

    pub fn sync_overrides_from_stage(&mut self) {
        for (key, value) in self.overrides.stage.get_fields() {
            if !value.is_empty() {
                self.overrides.raw_overrides.insert(
                    key.replace('/', "_").to_string(),
                    value.clone()
                );
            }
        }
    }

    pub fn sync_overrides_from_keymodes(&mut self) {
        for keymode in &self.keymodes {
            let k = keymode.keymode;
            
            Keymodes::iter(keymode, |vec, category, subcategory, suffix| {
                for (col_idx, img) in vec.iter().enumerate() {
                    if !img.is_empty() {
                        let prefix = if subcategory.is_empty() {
                            category.to_string()
                        } else {
                            format!("{}/{}", category, subcategory)
                        };
                        let key = format!("{}/{}k-{}{}", prefix, k, col_idx + 1, suffix);
                        self.overrides.raw_overrides.insert(key, img.clone());
                    }
                }
            });
        }
    }

    fn alloc_vecs(keymode: &mut Keymode, k: usize) {
        Keymodes::iter_mut(keymode, |vec, _, _, _| {
            if vec.len() != k {
                vec.resize(k, String::new());
            }
        });
    }

    fn extract_keymode_column(s: &str) -> Option<(usize, usize)> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() < 2 { return None; }

        let keymode_part = parts[0];
        if !keymode_part.ends_with('k') { return None; }

        let keymode = keymode_part.trim_end_matches('k').parse::<usize>().ok()?;
        let column = parts[1].parse::<usize>().ok()?;
        
        Some((keymode, column))
    }
}
