use std::collections::HashSet;
use crate::skin::osu::keymode::Keymode;
use crate::skin::osu::General;
use crate::ini::from_ini;

#[derive(Clone, Debug)]
pub struct SkinIni {
    pub general: General,
    pub keymodes: Vec<Keymode>
}

impl SkinIni {
    pub fn from_str(str: &str) -> Result<Self, Box<dyn std::error::Error>> {
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

    pub fn to_str(&self) -> String {
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

    pub fn get_mania_texture_paths(&self) -> HashSet<String> {
        let mut result = HashSet::new();

        for keymode in &self.keymodes {
            result.extend(keymode.get_texture_paths());
        }

        result
    }
}