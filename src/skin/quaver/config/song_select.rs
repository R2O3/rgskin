use merge::Merge;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::common::color::Rgba;
use crate::common::vector::Vector2;
use crate::utils;
use crate::utils::serde::{add_key_value, add_key_value_if_not_default, parse_bool, parse_key_value_eq, serialize_bool};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Merge)]
pub struct SongSelect {
    // Leaderboard Panel
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub leaderboard_score_color_even: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub leaderboard_score_color_odd: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub leaderboard_score_rank_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub leaderboard_score_rating_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub leaderboard_score_accuracy_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub leaderboard_score_username_self_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub leaderboard_score_username_other_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub leaderboard_title_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub leaderboard_ranking_title_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub leaderboard_dropdown_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub leaderboard_status_text_color: Rgba,

    // Personal Best Panel
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub personal_best_title_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub personal_best_trophy_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub personal_best_rank_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub no_personal_best_color: Rgba,

    // Mapset Panel
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub mapset_panel_song_title_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub mapset_panel_song_artist_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub mapset_panel_creator_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub mapset_panel_by_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub mapset_panel_banner_size: Vector2<u32>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(strategy = utils::merge::any::overwrite)]   
    pub mapset_panel_hovering_alpha: f32,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(strategy = utils::merge::any::overwrite)]   
    pub map_background_brightness: u8,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    #[merge(strategy = utils::merge::any::overwrite)]
    pub display_map_background: bool,
}

impl Default for SongSelect {
    fn default() -> Self {
        Self {
            leaderboard_score_color_even: Rgba { red: 54, green: 54, blue: 54, alpha: 255 },
            leaderboard_score_color_odd: Rgba { red: 36, green: 36, blue: 36, alpha: 255 },
            leaderboard_score_rank_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            leaderboard_score_rating_color: Rgba { red: 233, green: 183, blue: 54, alpha: 255 },
            leaderboard_score_accuracy_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            leaderboard_score_username_self_color: Rgba { red: 81, green: 197, blue: 249, alpha: 255 },
            leaderboard_score_username_other_color: Rgba { red: 251, green: 255, blue: 182, alpha: 255 },
            leaderboard_title_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            leaderboard_ranking_title_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            leaderboard_dropdown_color: Rgba { red: 16, green: 200, blue: 246, alpha: 255 },
            leaderboard_status_text_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            personal_best_title_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            personal_best_trophy_color: Rgba { red: 233, green: 183, blue: 54, alpha: 255 },
            personal_best_rank_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            no_personal_best_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            mapset_panel_song_title_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            mapset_panel_song_artist_color: Rgba { red: 5, green: 151, blue: 229, alpha: 255 },
            mapset_panel_creator_color: Rgba { red: 5, green: 135, blue: 229, alpha: 255 },
            mapset_panel_by_color: Rgba { red: 117, green: 117, blue: 117, alpha: 255 },
            mapset_panel_banner_size: Vector2::new(421, 82),
            mapset_panel_hovering_alpha: 0.35,
            map_background_brightness: 15,
            display_map_background: false,
        }
    }
}

impl SongSelect {
    pub fn from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut ss = Self::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('[') {
                continue;
            }
            let (key_str, value_str) = parse_key_value_eq(line).unwrap_or_default();
            match key_str {
                "LeaderboardScoreColorEven" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.leaderboard_score_color_even = c; }
                }
                "LeaderboardScoreColorOdd" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.leaderboard_score_color_odd = c; }
                }
                "LeaderboardScoreRankColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.leaderboard_score_rank_color = c; }
                }
                "LeaderboardScoreRatingColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.leaderboard_score_rating_color = c; }
                }
                "LeaderboardScoreAccuracyColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.leaderboard_score_accuracy_color = c; }
                }
                "LeaderboardScoreUsernameSelfColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.leaderboard_score_username_self_color = c; }
                }
                "LeaderboardScoreUsernameOtherColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.leaderboard_score_username_other_color = c; }
                }
                "LeaderboardTitleColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.leaderboard_title_color = c; }
                }
                "LeaderboardRankingTitleColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.leaderboard_ranking_title_color = c; }
                }
                "LeaderboardDropdownColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.leaderboard_dropdown_color = c; }
                }
                "LeaderboardStatusTextColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.leaderboard_status_text_color = c; }
                }
                "PersonalBestTitleColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.personal_best_title_color = c; }
                }
                "PersonalBestTrophyColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.personal_best_trophy_color = c; }
                }
                "PersonalBestRankColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.personal_best_rank_color = c; }
                }
                "NoPersonalBestColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.no_personal_best_color = c; }
                }
                "MapsetPanelSongTitleColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.mapset_panel_song_title_color = c; }
                }
                "MapsetPanelSongArtistColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.mapset_panel_song_artist_color = c; }
                }
                "MapsetPanelCreatorColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.mapset_panel_creator_color = c; }
                }
                "MapsetPanelByColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { ss.mapset_panel_by_color = c; }
                }
                "MapsetPanelBannerSize" => {
                    // Format: "width,height"
                    let parts: Vec<&str> = value_str.splitn(2, ',').collect();
                    if parts.len() == 2 {
                        if let (Ok(w), Ok(h)) = (parts[0].trim().parse(), parts[1].trim().parse()) {
                            ss.mapset_panel_banner_size = Vector2 { x: w, y: h };
                        }
                    }
                }
                "MapsetPanelHoveringAlpha" => {
                    ss.mapset_panel_hovering_alpha = value_str.parse().unwrap_or(0.35);
                }
                "MapBackgroundBrightness" => {
                    ss.map_background_brightness = value_str.parse().unwrap_or(15);
                }
                "DisplayMapBackground" => {
                    ss.display_map_background = parse_bool(value_str);
                }
                _ => {}
            }
        }

        Ok(ss)
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let default = SongSelect::default();

        // Leaderboard
        macro_rules! wc {
            ($field:ident, $key:literal) => {
                if self.$field != default.$field {
                    add_key_value(&mut result, $key, " = ", &self.$field.to_str(), "\n");
                }
            };
        }

        wc!(leaderboard_score_color_even, "LeaderboardScoreColorEven");
        wc!(leaderboard_score_color_odd, "LeaderboardScoreColorOdd");
        wc!(leaderboard_score_rank_color, "LeaderboardScoreRankColor");
        wc!(leaderboard_score_rating_color, "LeaderboardScoreRatingColor");
        wc!(leaderboard_score_accuracy_color, "LeaderboardScoreAccuracyColor");
        wc!(leaderboard_score_username_self_color, "LeaderboardScoreUsernameSelfColor");
        wc!(leaderboard_score_username_other_color, "LeaderboardScoreUsernameOtherColor");
        wc!(leaderboard_title_color, "LeaderboardTitleColor");
        wc!(leaderboard_ranking_title_color, "LeaderboardRankingTitleColor");
        wc!(leaderboard_dropdown_color, "LeaderboardDropdownColor");
        wc!(leaderboard_status_text_color, "LeaderboardStatusTextColor");

        // Personal Best
        wc!(personal_best_title_color, "PersonalBestTitleColor");
        wc!(personal_best_trophy_color, "PersonalBestTrophyColor");
        wc!(personal_best_rank_color, "PersonalBestRankColor");
        wc!(no_personal_best_color, "NoPersonalBestColor");

        // Mapset Panel
        wc!(mapset_panel_song_title_color, "MapsetPanelSongTitleColor");
        wc!(mapset_panel_song_artist_color, "MapsetPanelSongArtistColor");
        wc!(mapset_panel_creator_color, "MapsetPanelCreatorColor");
        wc!(mapset_panel_by_color, "MapsetPanelByColor");

        if self.mapset_panel_banner_size != default.mapset_panel_banner_size {
            add_key_value(
                &mut result,
                "MapsetPanelBannerSize",
                " = ",
                &format!("{},{}", self.mapset_panel_banner_size.x, self.mapset_panel_banner_size.y),
                "\n",
            );
        }
        add_key_value_if_not_default::<f32>(&mut result, "MapsetPanelHoveringAlpha", " = ", &self.mapset_panel_hovering_alpha, &default.mapset_panel_hovering_alpha);
        add_key_value_if_not_default::<u8>(&mut result, "MapBackgroundBrightness", " = ", &self.map_background_brightness, &default.map_background_brightness);
        if self.display_map_background != default.display_map_background {
            add_key_value(&mut result, "DisplayMapBackground", " = ", &serialize_bool(self.display_map_background).to_string(), "\n");
        }

        result
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl SongSelect {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromStr))]
    pub fn wasm_from_str(content: &str) -> Result<SongSelect, String> {
        Self::from_str(content).map_err(|e| e.to_string())
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toString))]
    pub fn wasm_to_string(&self) -> String {
        self.to_string()
    }
}
