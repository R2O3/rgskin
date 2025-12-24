use std::collections::HashSet;
use std::str::FromStr;
use crate::common::traits::{ManiaSkinConfig, SkinConfig};
use crate::osu::static_assets;
use crate::skin::osu::keymode::Keymode;
use crate::skin::osu::General;
use crate::ini::from_ini;

#[derive(Clone, Debug)]
pub struct SkinIni {
    pub general: General,
    pub keymodes: Vec<Keymode>
}

impl ToString for SkinIni {
    fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str("[General]\n");
        result.push_str(&self.general.to_str());
        result.push('\n');

        for keymode in &self.keymodes {
            result.push_str("[Mania]\n");
            result.push_str(&keymode.to_str());
            result.push('\n');
        }

        result
    }
}

impl FromStr for SkinIni {
    type Err = Box<dyn std::error::Error>;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut general = General::default();
        let mut keymodes = Vec::new();

        from_ini(str, |section, content| {
            match section {
                "General" => general = General::from_str(content)?,
                "Mania" => keymodes.push(Keymode::from_str(content)?),
                _ => { },
            }
            Ok(())
        })?;

        Ok(SkinIni { general, keymodes })
    }
}

impl SkinConfig for SkinIni {
    fn get_required_texture_paths(&self) -> HashSet<String> {
        let mut result = HashSet::new();

        for keymode in &self.keymodes {
            result.extend(keymode.get_texture_paths());
        }

        result
    }

    fn get_required_sample_paths(&self) -> HashSet<String> {
        let mut result: HashSet<String> = HashSet::new();
        result.extend(static_assets::Samples::iter_mapped(|s| s.to_string()));
        result
    }
}

impl ManiaSkinConfig for SkinIni {
    type Keymode = Keymode;

    fn get_keymode(&self, keymode: u8) -> Option<&Keymode> {
        for k in &self.keymodes {
            if k.keymode == keymode { return Some(k); }
        }
        None
    }
}