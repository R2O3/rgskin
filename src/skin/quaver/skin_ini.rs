use merge::Merge;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use std::str::FromStr;
use crate::ini::from_ini;
use crate::quaver::config::{General, Keymode, MainMenu, MenuBorder, Results, SongSelect};
use crate::quaver::{dynamic_assets, static_assets};
use crate::traits::SkinConfig;
use crate::{StringPattern, utils};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Default, Merge)]
pub struct QuaSkinIni {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub general: General,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub main_menu: MainMenu,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub menu_border: MenuBorder,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub song_select: SongSelect,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub results: Results,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(strategy = utils::merge::any::overwrite)] // TODO: implement keymode invariant support to quaver
    pub keymodes: Vec<Keymode>,
}

impl ToString for QuaSkinIni {
    fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str("[General]\n");
        result.push_str(&self.general.to_string());
        result.push('\n');

        result.push_str("[MainMenu]\n");
        result.push_str(&self.main_menu.to_string());
        result.push('\n');

        result.push_str("[MenuBorder]\n");
        result.push_str(&self.menu_border.to_string());
        result.push('\n');

        result.push_str("[SongSelect]\n");
        result.push_str(&self.song_select.to_string());
        result.push('\n');

        result.push_str("[Results]\n");
        result.push_str(&self.results.to_string());
        result.push('\n');

        for km in &self.keymodes {
            result.push_str(&format!("[{}K]\n", km.keymode));
            result.push_str(&km.to_str());
            result.push('\n');
        }

        result
    }
}

impl FromStr for QuaSkinIni {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut general = General::default();
        let mut main_menu = MainMenu::default();
        let mut menu_border = MenuBorder::default();
        let mut song_select = SongSelect::default();
        let mut results = Results::default();
        let mut keymodes: Vec<Keymode> = Vec::new();

        from_ini(s, |section, content| {
            match section {
                "General" => general = General::from_str(content)?,
                "MainMenu" => main_menu = MainMenu::from_str(content)?,
                "MenuBorder" => menu_border = MenuBorder::from_str(content)?,
                "SongSelect" => song_select = SongSelect::from_str(content)?,
                "Results" => results = Results::from_str(content)?,
                s if s.to_lowercase().ends_with('k') && s.len() > 1 => {
                    let mut km = Keymode::from_str(content)?;
                    km.keymode = section.trim_end_matches('K').parse().unwrap_or(4);
                    keymodes.push(km);
                }
                _ => {}
            }
            Ok(())
        })?;

        Ok(QuaSkinIni { general, main_menu, menu_border, song_select, results, keymodes })
    }
}

impl SkinConfig for QuaSkinIni {
    fn get_required_texture_paths(&self) -> Vec<StringPattern> {
        let mut result = Vec::new();

        // a lot of these are not needed as of now but I'm too lazy to figure out which ones to remove so here we are

        result.extend(dynamic_assets::Column::iter_mapped(|t| t));
        result.extend(dynamic_assets::LaneCover::iter_mapped(|t| t));
        result.extend(dynamic_assets::Lighting::iter_mapped(|t| t));
        result.extend(dynamic_assets::Notes::iter_mapped(|t| t));
        result.extend(dynamic_assets::Receptors::iter_mapped(|t| t));

        result.extend(dynamic_assets::Stage::iter_mapped(|t| t));
        result.extend(static_assets::Grades::iter_mapped(|t| t));
        result.extend(static_assets::HealthBar::iter_mapped(|t| t));
        result.extend(static_assets::HitBubbles::iter_mapped(|t| t));
        result.extend(static_assets::Judgements::iter_mapped(|t| t));
        result.extend(static_assets::Numbers::iter_mapped(|t| t));
        result.extend(static_assets::Scoreboard::iter_mapped(|t| t));
        result.extend(static_assets::SkipDisplay::iter_mapped(|t| t));
        result.extend(dynamic_assets::ComboAlerts::iter_mapped(|t| t));
        result.extend(static_assets::PauseScreen::iter_mapped(|t| t));
        result.extend(static_assets::BattleRoyale::iter_mapped(|t| t));
        result.extend(dynamic_assets::Background::iter_mapped(|t| t));

        result
    }

    // TODO: add samples for quaver
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl QuaSkinIni {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        QuaSkinIni::default()
    }

    #[wasm_bindgen(js_name = fromStr)]
    pub fn from_str_wasm(ini_str: &str) -> Result<Self, JsError> {
        Self::from_str(ini_str).map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string_wasm(&self) -> String {
        self.to_string()
    }

    #[wasm_bindgen(js_name = getKeymode)]
    pub fn wasm_get_keymode(&self, keymode: u8) -> Option<Keymode> {
        self.get_keymode(keymode).cloned()
    }
}

impl QuaSkinIni {
    pub fn get_keymode(&self, keymode: u8) -> Option<&Keymode> {
        self.keymodes.iter().find(|k| k.keymode == keymode)
    }
}