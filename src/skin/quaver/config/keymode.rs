use rgskin_derive::MergeDefault;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::StringPattern;
use crate::common::color::Rgba;
use crate::quaver::dynamic_assets;
use crate::traits::{KeymodeInvariant, LaneFallback};
use crate::utils::serde::{
    add_key_value, add_key_value_if_not_default, parse_key_value_eq, serialize_bool,
};

fn parse_bool(value: &str) -> Result<bool, std::str::ParseBoolError> {
    value.trim().to_lowercase().parse()
}

fn parse_list<T: std::str::FromStr>(value: &str) -> Vec<T> {
    value.split(',').filter_map(|s| s.trim().parse::<T>().ok()).collect()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq)]
pub enum DefaultSkin {
    Arrow,
    Bar,
    Circle,
}

impl Default for DefaultSkin {
    fn default() -> Self { Self::Bar }
}

impl DefaultSkin {
    pub fn from_str(s: &str) -> Self {
        match s.trim() {
            "Arrow" => Self::Arrow,
            "Circle" => Self::Circle,
            _ => Self::Bar,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Arrow => "Arrow",
            Self::Bar => "Bar",
            Self::Circle => "Circle",
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq)]
pub enum HealthBarKeysAlignment {
    LeftStage,
    RightStage,
    TopLeft,
}

impl Default for HealthBarKeysAlignment {
    fn default() -> Self { Self::RightStage }
}

impl HealthBarKeysAlignment {
    pub fn from_str(s: &str) -> Self {
        match s.trim() {
            "LeftStage" => Self::LeftStage,
            "TopLeft" => Self::TopLeft,
            _ => Self::RightStage,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::LeftStage => "LeftStage",
            Self::RightStage => "RightStage",
            Self::TopLeft => "TopLeft",
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq)]
pub enum HealthBarType {
    Horizontal,
    Vertical,
}

impl Default for HealthBarType {
    fn default() -> Self { Self::Vertical }
}

impl HealthBarType {
    pub fn from_str(s: &str) -> Self {
        match s.trim() {
            "Horizontal" => Self::Horizontal,
            _ => Self::Vertical,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "Horizontal",
            Self::Vertical => "Vertical",
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq)]
pub enum HitBubblesAlignment {
    LeftStage,
    RightStage,
    BelowStage,
}

impl Default for HitBubblesAlignment {
    fn default() -> Self { Self::LeftStage }
}

impl HitBubblesAlignment {
    pub fn from_str(s: &str) -> Self {
        match s.trim() {
            "RightStage" => Self::RightStage,
            "BelowStage" => Self::BelowStage,
            _ => Self::LeftStage,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::LeftStage => "LeftStage",
            Self::RightStage => "RightStage",
            Self::BelowStage => "BelowStage",
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq)]
pub enum HitBubblesType {
    FallDown,
    FallUp,
    FallLeft,
    FallRight,
}

impl Default for HitBubblesType {
    fn default() -> Self { Self::FallDown }
}

impl HitBubblesType {
    pub fn from_str(s: &str) -> Self {
        match s.trim() {
            "FallUp" => Self::FallUp,
            "FallLeft" => Self::FallLeft,
            "FallRight" => Self::FallRight,
            _ => Self::FallDown,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::FallDown => "FallDown",
            Self::FallUp => "FallUp",
            Self::FallLeft => "FallLeft",
            Self::FallRight => "FallRight",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HitBubblesRecordedJudgements {
    Marv,
    Perf,
    Great,
    Good,
    Okay,
    Miss,
    NoMarv,
    All,
    Default,
    Custom(Vec<String>),
}

impl Default for HitBubblesRecordedJudgements {
    fn default() -> Self { Self::NoMarv }
}

impl HitBubblesRecordedJudgements {
    pub fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split(',').map(str::trim).collect();
        if parts.len() == 1 {
            match parts[0] {
                "Marv" => return Self::Marv,
                "Perf" => return Self::Perf,
                "Great" => return Self::Great,
                "Good" => return Self::Good,
                "Okay" => return Self::Okay,
                "Miss" => return Self::Miss,
                "NoMarv" => return Self::NoMarv,
                "All" => return Self::All,
                "Default" => return Self::Default,
                _ => {}
            }
        }
        Self::Custom(parts.iter().map(|s| s.to_string()).collect())
    }

    pub fn to_str(&self) -> String {
        match self {
            Self::Marv => "Marv".to_string(),
            Self::Perf => "Perf".to_string(),
            Self::Great => "Great".to_string(),
            Self::Good => "Good".to_string(),
            Self::Okay => "Okay".to_string(),
            Self::Miss => "Miss".to_string(),
            Self::NoMarv => "NoMarv".to_string(),
            Self::All => "All".to_string(),
            Self::Default => "Default".to_string(),
            Self::Custom(v) => v.join(","),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = QuaKeymode))]
#[derive(Clone, Debug, MergeDefault)]
pub struct Keymode {
    #[merge(skip)]
    pub keymode: u8,

    // notes
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub default_skin: DefaultSkin,
    pub color_objects_by_snap_distance: bool,
    pub use_hit_object_sheet: bool,
    pub rotate_hit_objects_by_column: bool,
    pub flip_note_images_on_upscroll: bool,
    pub flip_note_end_images_on_upscroll: bool,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub dead_note_color: Rgba,
    pub draw_long_note_end: bool,
    pub note_padding: i32,
    pub width_for_note_height_scale: i32,

    // playfield
    pub bg_mask_alpha: f32,
    pub bg_mask_padding: i32,
    pub column_alignment: i32, // uses percentage
    pub column_size: i32,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub column_colors: Vec<Rgba>,
    pub column_lighting_offset_y: i32,
    pub column_lighting_scale: f32,
    pub hit_pos_offset_y: i32,
    pub receptor_pos_offset_y: i32,
    pub receptors_over_hit_objects: bool,
    pub stage_receptor_padding: i32,
    pub coop_playfield_padding: i32,

    // judgement / hit error
    pub hit_error_chevron_size: i32,
    pub hit_error_height: i32,
    pub hit_error_pos_x: i32,
    pub hit_error_pos_y: i32,
    pub hit_error_alpha: f32,
    pub judgement_hit_burst_fps: i32,
    pub judgement_burst_pos_y: i32,
    pub judgement_hit_burst_bump_y: i32,
    pub judgement_hit_burst_bump_time: i32,
    pub judgement_hit_burst_scale: u8,

    // lighting
    pub hit_lighting_x: i32,
    pub hit_lighting_y: i32,
    pub hit_lighting_fps: i32,
    pub hit_lighting_scale: i32,
    pub hit_lighting_column_rotation: bool,
    pub hold_lighting_fps: i32,
    pub hold_lighting_scale: f32,
    pub hold_lighting_column_rotation: bool,

    // health bar
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub health_bar_keys_alignment: HealthBarKeysAlignment,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub health_bar_type: HealthBarType,
    pub health_bar_pos_offset_x: i32,
    pub health_bar_pos_offset_y: i32,
    pub health_bar_scale: i32,

    // hit bubbles
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub hit_bubbles_alignment: HitBubblesAlignment,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub hit_bubbles_type: HitBubblesType,
    pub hit_bubbles_pos_x: i32,
    pub hit_bubbles_pos_y: i32,
    pub hit_bubbles_scale: f32,
    pub hit_bubble_scale: f32,
    pub hit_bubble_border_padding: f32,
    pub hit_bubble_padding: f32,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub hit_bubbles_recorded_judgements: HitBubblesRecordedJudgements,

    // combo
    pub combo_display_scale: i32,
    pub combo_pos_x: i32,
    pub combo_pos_y: i32,
    pub combo_display_bump_y: i32,
    pub combo_display_bump_time: i32,

    // rating
    pub rating_display_scale: i32,
    pub rating_display_pos_x: i32,
    pub rating_display_pos_y: i32,

    // accuracy
    pub accuracy_display_scale: i32,
    pub accuracy_display_pos_x: i32,
    pub accuracy_display_pos_y: i32,

    // kps
    pub kps_display_scale: i32,
    pub kps_display_pos_x: i32,
    pub kps_display_pos_y: i32,

    // score display
    pub score_display_scale: i32,
    pub score_display_pos_x: i32,
    pub score_display_pos_y: i32,

    // multiplayer
    pub battle_royale_alert_pos_x: i32,
    pub battle_royale_alert_pos_y: i32,
    pub battle_royale_alert_scale: i32,
    pub battle_royale_eliminated_pos_x: i32,
    pub battle_royale_eliminated_pos_y: i32,

    // judgement counter
    pub judgement_counter_alpha: f32,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub judgement_counter_font_color: Rgba,
    pub judgement_counter_size: i32,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub judge_color_marv: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub judge_color_perf: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub judge_color_great: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub judge_color_good: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub judge_color_okay: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub judge_color_miss: Rgba,
    pub judgement_counter_pos_x: i32,
    pub judgement_counter_pos_y: i32,
    pub judgement_counter_padding: i32,
    pub judgement_counter_horizontal: bool,
    pub judgement_counter_fade_to_alpha: bool,
    pub use_judgement_color_for_numbers: bool,

    // stage timing bar
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub song_time_progress_active_color: Rgba,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub song_time_progress_inactive_color: Rgba,
    pub song_time_progress_scale: i32,
    pub song_time_progress_position_at_top: bool,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub timing_line_color: Rgba,

    // mini progress bar
    pub show_mini_song_bar: bool,
    pub mini_song_bar_display_pos_x: i32,
    pub mini_song_bar_display_pos_y: i32,
    pub mini_song_bar_display_width_factor: i32,
    pub mini_song_bar_display_height: i32,

    pub use_fallback: bool,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub hitobject_fallbacks: Vec<u8>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub holdbody_fallbacks: Vec<u8>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub holdend_fallbacks: Vec<u8>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub receptor_fallbacks: Vec<u8>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub hitobject_rotations: Vec<u8>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub receptor_rotations: Vec<u8>,
}

impl Default for Keymode {
    fn default() -> Self {
        Self {
            keymode: 0,
            default_skin: DefaultSkin::default(),
            color_objects_by_snap_distance: false,
            use_hit_object_sheet: false,
            rotate_hit_objects_by_column: false,
            flip_note_images_on_upscroll: false,
            flip_note_end_images_on_upscroll: false,
            dead_note_color: Rgba { red: 200, green: 200, blue: 200, alpha: 255 },
            draw_long_note_end: true,
            note_padding: 0,
            width_for_note_height_scale: 0,
            bg_mask_alpha: 1.0,
            bg_mask_padding: 0,
            column_alignment: 0,
            column_size: 90,
            column_colors: Vec::new(),
            column_lighting_offset_y: 0,
            column_lighting_scale: 1.0,
            hit_pos_offset_y: 0,
            receptor_pos_offset_y: 0,
            receptors_over_hit_objects: true,
            stage_receptor_padding: 0,
            coop_playfield_padding: 92,
            hit_error_chevron_size: 8,
            hit_error_height: 10,
            hit_error_pos_x: 0,
            hit_error_pos_y: 45,
            hit_error_alpha: 0.5,
            judgement_hit_burst_fps: 60,
            judgement_burst_pos_y: 0,
            judgement_hit_burst_bump_y: -5,
            judgement_hit_burst_bump_time: 183,
            judgement_hit_burst_scale: 100,
            hit_lighting_x: 0,
            hit_lighting_y: 0,
            hit_lighting_fps: 60,
            hit_lighting_scale: 100,
            hit_lighting_column_rotation: false,
            hold_lighting_fps: 60,
            hold_lighting_scale: 100.0,
            hold_lighting_column_rotation: false,
            health_bar_keys_alignment: HealthBarKeysAlignment::default(),
            health_bar_type: HealthBarType::default(),
            health_bar_pos_offset_x: 5,
            health_bar_pos_offset_y: 5,
            health_bar_scale: 100,
            hit_bubbles_alignment: HitBubblesAlignment::default(),
            hit_bubbles_type: HitBubblesType::default(),
            hit_bubbles_pos_x: -10,
            hit_bubbles_pos_y: 170,
            hit_bubbles_scale: 1.0,
            hit_bubble_scale: 1.0,
            hit_bubble_border_padding: 7.0,
            hit_bubble_padding: 3.0,
            hit_bubbles_recorded_judgements: HitBubblesRecordedJudgements::default(),
            combo_display_scale: 100,
            combo_pos_x: 0,
            combo_pos_y: -40,
            combo_display_bump_y: -5,
            combo_display_bump_time: 370,
            rating_display_scale: 100,
            rating_display_pos_x: 10,
            rating_display_pos_y: 15,
            accuracy_display_scale: 100,
            accuracy_display_pos_x: -10,
            accuracy_display_pos_y: 5,
            kps_display_scale: 100,
            kps_display_pos_x: -10,
            kps_display_pos_y: 15,
            score_display_scale: 100,
            score_display_pos_x: 10,
            score_display_pos_y: 5,
            battle_royale_alert_pos_x: 0,
            battle_royale_alert_pos_y: -150,
            battle_royale_alert_scale: 5,
            battle_royale_eliminated_pos_x: 0,
            battle_royale_eliminated_pos_y: -115,
            judgement_counter_alpha: 1.0,
            judgement_counter_font_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            judgement_counter_size: 40,
            judge_color_marv: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            judge_color_perf: Rgba { red: 255, green: 231, blue: 107, alpha: 255 },
            judge_color_great: Rgba { red: 86, green: 254, blue: 110, alpha: 255 },
            judge_color_good: Rgba { red: 0, green: 209, blue: 255, alpha: 255 },
            judge_color_okay: Rgba { red: 217, green: 107, blue: 206, alpha: 255 },
            judge_color_miss: Rgba { red: 249, green: 100, blue: 93, alpha: 255 },
            judgement_counter_pos_x: 0,
            judgement_counter_pos_y: 0,
            judgement_counter_padding: 0,
            judgement_counter_horizontal: false,
            judgement_counter_fade_to_alpha: false,
            use_judgement_color_for_numbers: false,
            song_time_progress_active_color: Rgba { red: 255, green: 231, blue: 107, alpha: 255 },
            song_time_progress_inactive_color: Rgba { red: 136, green: 136, blue: 136, alpha: 255 },
            song_time_progress_scale: 45,
            song_time_progress_position_at_top: false,
            timing_line_color: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            show_mini_song_bar: false,
            mini_song_bar_display_pos_x: 75,
            mini_song_bar_display_pos_y: 0,
            mini_song_bar_display_width_factor: 30,
            mini_song_bar_display_height: 4,
            use_fallback: false,
            hitobject_fallbacks: Vec::new(),
            holdbody_fallbacks: Vec::new(),
            holdend_fallbacks: Vec::new(),
            receptor_fallbacks: Vec::new(),
            hitobject_rotations: Vec::new(),
            receptor_rotations: Vec::new(),
        }
    }
}

impl Keymode {
    pub fn from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut km = Self::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('[') {
                continue;
            }
            let (key_str, value_str) = parse_key_value_eq(line).unwrap_or_default();

            match key_str {
                // notes
                "DefaultSkin" => km.default_skin = DefaultSkin::from_str(value_str),
                "ColorObjectsBySnapDistance" => km.color_objects_by_snap_distance = parse_bool(value_str).unwrap_or(km.color_objects_by_snap_distance),
                "UseHitObjectSheet" => km.use_hit_object_sheet = parse_bool(value_str).unwrap_or(km.use_hit_object_sheet),
                "RotateHitObjectsByColumn" => km.rotate_hit_objects_by_column = parse_bool(value_str).unwrap_or(km.rotate_hit_objects_by_column),
                "FlipNoteImagesOnUpscroll" => km.flip_note_images_on_upscroll = parse_bool(value_str).unwrap_or(km.flip_note_images_on_upscroll),
                "FlipNoteEndImagesOnUpscroll" => km.flip_note_end_images_on_upscroll = parse_bool(value_str).unwrap_or(km.flip_note_end_images_on_upscroll),
                "DeadNoteColor" => { if let Ok(c) = Rgba::from_str(value_str) { km.dead_note_color = c; } }
                "DrawLongNoteEnd" => km.draw_long_note_end = parse_bool(value_str).unwrap_or(km.draw_long_note_end),
                "NotePadding" => km.note_padding = value_str.parse().unwrap_or(km.note_padding),
                "WidthForNoteHeightScale" => km.width_for_note_height_scale = value_str.parse().unwrap_or(km.width_for_note_height_scale),

                // playfield
                "BgMaskAlpha" => km.bg_mask_alpha = value_str.parse().unwrap_or(km.bg_mask_alpha),
                "BgMaskPadding" => km.bg_mask_padding = value_str.parse().unwrap_or(km.bg_mask_padding),
                "ColumnAlignment" => km.column_alignment = value_str.parse().unwrap_or(km.column_alignment),
                "ColumnSize" => km.column_size = value_str.parse().unwrap_or(km.column_size),
                "ColumnLightingOffsetY" => km.column_lighting_offset_y = value_str.parse().unwrap_or(km.column_lighting_offset_y),
                "ColumnLightingScale" => km.column_lighting_scale = value_str.parse().unwrap_or(km.column_lighting_scale),
                "HitPosOffsetY" => km.hit_pos_offset_y = value_str.parse().unwrap_or(km.hit_pos_offset_y),
                "ReceptorPosOffsetY" => km.receptor_pos_offset_y = value_str.parse().unwrap_or(km.receptor_pos_offset_y),
                "ReceptorsOverHitObjects" => km.receptors_over_hit_objects = parse_bool(value_str).unwrap_or(km.receptors_over_hit_objects),
                "StageReceptorPadding" => km.stage_receptor_padding = value_str.parse().unwrap_or(km.stage_receptor_padding),
                "CoopPlayfieldPadding" => km.coop_playfield_padding = value_str.parse().unwrap_or(km.coop_playfield_padding),

                k if k.starts_with("ColumnColor") => {
                    let idx_str = &k["ColumnColor".len()..];
                    if let Ok(idx) = idx_str.parse::<usize>() {
                        if idx >= 1 {
                            let i = idx - 1;
                            while km.column_colors.len() <= i {
                                km.column_colors.push(Rgba { red: 255, green: 255, blue: 255, alpha: 255 });
                            }
                            if let Ok(c) = Rgba::from_str(value_str) {
                                km.column_colors[i] = c;
                            }
                        }
                    }
                }

                // judgement / hit error
                "HitErrorChevronSize" => km.hit_error_chevron_size = value_str.parse().unwrap_or(km.hit_error_chevron_size),
                "HitErrorHeight" => km.hit_error_height = value_str.parse().unwrap_or(km.hit_error_height),
                "HitErrorPosX" => km.hit_error_pos_x = value_str.parse().unwrap_or(km.hit_error_pos_x),
                "HitErrorPosY" => km.hit_error_pos_y = value_str.parse().unwrap_or(km.hit_error_pos_y),
                "HitErrorAlpha" => km.hit_error_alpha = value_str.parse().unwrap_or(km.hit_error_alpha),
                "JudgementHitBurstFps" => km.judgement_hit_burst_fps = value_str.parse().unwrap_or(km.judgement_hit_burst_fps),
                "JudgementBurstPosY" => km.judgement_burst_pos_y = value_str.parse().unwrap_or(km.judgement_burst_pos_y),
                "JudgementHitBurstBumpY" => km.judgement_hit_burst_bump_y = value_str.parse().unwrap_or(km.judgement_hit_burst_bump_y),
                "JudgementHitBurstBumpTime" => km.judgement_hit_burst_bump_time = value_str.parse().unwrap_or(km.judgement_hit_burst_bump_time),
                "JudgementHitBurstScale" => km.judgement_hit_burst_scale = value_str.parse().unwrap_or(km.judgement_hit_burst_scale),

                // lighting
                "HitLightingX" => km.hit_lighting_x = value_str.parse().unwrap_or(km.hit_lighting_x),
                "HitLightingY" => km.hit_lighting_y = value_str.parse().unwrap_or(km.hit_lighting_y),
                "HitLightingFps" => km.hit_lighting_fps = value_str.parse().unwrap_or(km.hit_lighting_fps),
                "HitLightingScale" => km.hit_lighting_scale = value_str.parse().unwrap_or(km.hit_lighting_scale),
                "HitLightingColumnRotation" => km.hit_lighting_column_rotation = parse_bool(value_str).unwrap_or(km.hit_lighting_column_rotation),
                "HoldLightingFps" => km.hold_lighting_fps = value_str.parse().unwrap_or(km.hold_lighting_fps),
                "HoldLightingScale" => km.hold_lighting_scale = value_str.parse().unwrap_or(km.hold_lighting_scale),
                "HoldLightingColumnRotation" => km.hold_lighting_column_rotation = parse_bool(value_str).unwrap_or(km.hold_lighting_column_rotation),

                // health bar
                "HealthBarKeysAlignment" => km.health_bar_keys_alignment = HealthBarKeysAlignment::from_str(value_str),
                "HealthBarType" => km.health_bar_type = HealthBarType::from_str(value_str),
                "HealthBarPosOffsetX" => km.health_bar_pos_offset_x = value_str.parse().unwrap_or(km.health_bar_pos_offset_x),
                "HealthBarPosOffsetY" => km.health_bar_pos_offset_y = value_str.parse().unwrap_or(km.health_bar_pos_offset_y),
                "HealthBarScale" => km.health_bar_scale = value_str.parse().unwrap_or(km.health_bar_scale),

                // hit bubbles
                "HitBubblesAlignment" => km.hit_bubbles_alignment = HitBubblesAlignment::from_str(value_str),
                "HitBubblesType" => km.hit_bubbles_type = HitBubblesType::from_str(value_str),
                "HitBubblesPosX" => km.hit_bubbles_pos_x = value_str.parse().unwrap_or(km.hit_bubbles_pos_x),
                "HitBubblesPosY" => km.hit_bubbles_pos_y = value_str.parse().unwrap_or(km.hit_bubbles_pos_y),
                "HitBubblesScale" => km.hit_bubbles_scale = value_str.parse().unwrap_or(km.hit_bubbles_scale),
                "HitBubbleScale" => km.hit_bubble_scale = value_str.parse().unwrap_or(km.hit_bubble_scale),
                "HitBubbleBorderPadding" => km.hit_bubble_border_padding = value_str.parse().unwrap_or(km.hit_bubble_border_padding),
                "HitBubblePadding" => km.hit_bubble_padding = value_str.parse().unwrap_or(km.hit_bubble_padding),
                "HitBubblesRecordedJudgements" => {
                    km.hit_bubbles_recorded_judgements = HitBubblesRecordedJudgements::from_str(value_str);
                }

                // combo
                "ComboDisplayScale" => km.combo_display_scale = value_str.parse().unwrap_or(km.combo_display_scale),
                "ComboPosX" => km.combo_pos_x = value_str.parse().unwrap_or(km.combo_pos_x),
                "ComboPosY" => km.combo_pos_y = value_str.parse().unwrap_or(km.combo_pos_y),
                "ComboDisplayBumpY" => km.combo_display_bump_y = value_str.parse().unwrap_or(km.combo_display_bump_y),
                "ComboDisplayBumpTime" => km.combo_display_bump_time = value_str.parse().unwrap_or(km.combo_display_bump_time),

                // rating
                "RatingDisplayScale" => km.rating_display_scale = value_str.parse().unwrap_or(km.rating_display_scale),
                "RatingDisplayPosX" => km.rating_display_pos_x = value_str.parse().unwrap_or(km.rating_display_pos_x),
                "RatingDisplayPosY" => km.rating_display_pos_y = value_str.parse().unwrap_or(km.rating_display_pos_y),

                // accuracy
                "AccuracyDisplayScale" => km.accuracy_display_scale = value_str.parse().unwrap_or(km.accuracy_display_scale),
                "AccuracyDisplayPosX" => km.accuracy_display_pos_x = value_str.parse().unwrap_or(km.accuracy_display_pos_x),
                "AccuracyDisplayPosY" => km.accuracy_display_pos_y = value_str.parse().unwrap_or(km.accuracy_display_pos_y),

                // kps
                "KpsDisplayScale" => km.kps_display_scale = value_str.parse().unwrap_or(km.kps_display_scale),
                "KpsDisplayPosX" => km.kps_display_pos_x = value_str.parse().unwrap_or(km.kps_display_pos_x),
                "KpsDisplayPosY" => km.kps_display_pos_y = value_str.parse().unwrap_or(km.kps_display_pos_y),

                // score
                "ScoreDisplayScale" => km.score_display_scale = value_str.parse().unwrap_or(km.score_display_scale),
                "ScoreDisplayPosX" => km.score_display_pos_x = value_str.parse().unwrap_or(km.score_display_pos_x),
                "ScoreDisplayPosY" => km.score_display_pos_y = value_str.parse().unwrap_or(km.score_display_pos_y),

                // multiplayer
                "BattleRoyaleAlertPosX" => km.battle_royale_alert_pos_x = value_str.parse().unwrap_or(km.battle_royale_alert_pos_x),
                "BattleRoyaleAlertPosY" => km.battle_royale_alert_pos_y = value_str.parse().unwrap_or(km.battle_royale_alert_pos_y),
                "BattleRoyaleAlertScale" => km.battle_royale_alert_scale = value_str.parse().unwrap_or(km.battle_royale_alert_scale),
                "BattleRoyaleEliminatedPosX" => km.battle_royale_eliminated_pos_x = value_str.parse().unwrap_or(km.battle_royale_eliminated_pos_x),
                "BattleRoyaleEliminatedPosY" => km.battle_royale_eliminated_pos_y = value_str.parse().unwrap_or(km.battle_royale_eliminated_pos_y),

                // judgement counter
                "JudgementCounterAlpha" => km.judgement_counter_alpha = value_str.parse().unwrap_or(km.judgement_counter_alpha),
                "JudgementCounterFontColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { km.judgement_counter_font_color = c; }
                }
                "JudgementCounterSize" => km.judgement_counter_size = value_str.parse().unwrap_or(km.judgement_counter_size),
                "JudgeColorMarv" => { if let Ok(c) = Rgba::from_str(value_str) { km.judge_color_marv = c; } }
                "JudgeColorPerf" => { if let Ok(c) = Rgba::from_str(value_str) { km.judge_color_perf = c; } }
                "JudgeColorGreat" => { if let Ok(c) = Rgba::from_str(value_str) { km.judge_color_great = c; } }
                "JudgeColorGood" => { if let Ok(c) = Rgba::from_str(value_str) { km.judge_color_good = c; } }
                "JudgeColorOkay" => { if let Ok(c) = Rgba::from_str(value_str) { km.judge_color_okay = c; } }
                "JudgeColorMiss" => { if let Ok(c) = Rgba::from_str(value_str) { km.judge_color_miss = c; } }
                "JudgementCounterPosX" => km.judgement_counter_pos_x = value_str.parse().unwrap_or(km.judgement_counter_pos_x),
                "JudgementCounterPosY" => km.judgement_counter_pos_y = value_str.parse().unwrap_or(km.judgement_counter_pos_y),
                "JudgementCounterPadding" => km.judgement_counter_padding = value_str.parse().unwrap_or(km.judgement_counter_padding),
                "JudgementCounterHorizontal" => km.judgement_counter_horizontal = parse_bool(value_str).unwrap_or(km.judgement_counter_horizontal),
                "JudgementCounterFadeToAlpha" => km.judgement_counter_fade_to_alpha = parse_bool(value_str).unwrap_or(km.judgement_counter_fade_to_alpha),
                "UseJudgementColorForNumbers" => km.use_judgement_color_for_numbers = parse_bool(value_str).unwrap_or(km.use_judgement_color_for_numbers),

                // stage timing bar
                "SongTimeProgressActiveColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { km.song_time_progress_active_color = c; }
                }
                "SongTimeProgressInactiveColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { km.song_time_progress_inactive_color = c; }
                }
                "SongTimeProgressScale" => km.song_time_progress_scale = value_str.parse().unwrap_or(km.song_time_progress_scale),
                "SongTimeProgressPositionAtTop" => km.song_time_progress_position_at_top = parse_bool(value_str).unwrap_or(km.song_time_progress_position_at_top),
                "TimingLineColor" => {
                    if let Ok(c) = Rgba::from_str(value_str) { km.timing_line_color = c; }
                }

                // mini progress bar
                "ShowMiniSongBar" => km.show_mini_song_bar = parse_bool(value_str).unwrap_or(km.show_mini_song_bar),
                "MiniSongBarDisplayPosX" => km.mini_song_bar_display_pos_x = value_str.parse().unwrap_or(km.mini_song_bar_display_pos_x),
                "MiniSongBarDisplayPosY" => km.mini_song_bar_display_pos_y = value_str.parse().unwrap_or(km.mini_song_bar_display_pos_y),
                "MiniSongBarDisplayWidthFactor" => km.mini_song_bar_display_width_factor = value_str.parse().unwrap_or(km.mini_song_bar_display_width_factor),
                "MiniSongBarDisplayHeight" => km.mini_song_bar_display_height = value_str.parse().unwrap_or(km.mini_song_bar_display_height),

                // fallbacks
                "UseFallback" => km.use_fallback = parse_bool(value_str).unwrap_or(km.use_fallback),
                "HitObjectFallbacks" => km.hitobject_fallbacks = parse_list::<u8>(value_str),
                "HoldBodyFallbacks" => km.holdbody_fallbacks = parse_list::<u8>(value_str),
                "HoldEndFallbacks" => km.holdend_fallbacks = parse_list::<u8>(value_str),
                "ReceptorFallbacks" => km.receptor_fallbacks = parse_list::<u8>(value_str),
                "HitObjectRotations" => km.hitobject_rotations = parse_list::<u8>(value_str),
                "ReceptorRotations" => km.receptor_rotations = parse_list::<u8>(value_str),

                _ => {}
            }
        }

        Ok(km)
    }

    pub fn to_str(&self) -> String {
        let mut result = String::new();
        let d = Keymode::default();

        macro_rules! wi {
            ($key:literal, $field:ident) => {
                add_key_value_if_not_default(&mut result, $key, " = ", &self.$field, &d.$field);
            };
        }
        macro_rules! wc {
            ($key:literal, $field:ident) => {
                if self.$field != d.$field {
                    add_key_value(&mut result, $key, " = ", &self.$field.to_str(), "\n");
                }
            };
        }
        macro_rules! wb {
            ($key:literal, $field:ident) => {
                if self.$field != d.$field {
                    add_key_value(&mut result, $key, " = ", &serialize_bool(self.$field).to_string(), "\n");
                }
            };
        }
        macro_rules! wl {
            ($key:literal, $field:ident) => {{
                let joined = self.$field.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",");
                let joined_d = d.$field.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",");
                add_key_value_if_not_default(&mut result, $key, " = ", &joined, &joined_d);
            }};
        }

        // notes
        if self.default_skin != d.default_skin {
            add_key_value(&mut result, "DefaultSkin", " = ", self.default_skin.to_str(), "\n");
        }
        wb!("ColorObjectsBySnapDistance", color_objects_by_snap_distance);
        wb!("UseHitObjectSheet", use_hit_object_sheet);
        wb!("RotateHitObjectsByColumn", rotate_hit_objects_by_column);
        wb!("FlipNoteImagesOnUpscroll", flip_note_images_on_upscroll);
        wb!("FlipNoteEndImagesOnUpscroll", flip_note_end_images_on_upscroll);
        wc!("DeadNoteColor", dead_note_color);
        if !self.draw_long_note_end {
            add_key_value(&mut result, "DrawLongNoteEnd", " = ", "False", "\n");
        }
        wi!("NotePadding", note_padding);
        wi!("WidthForNoteHeightScale", width_for_note_height_scale);

        // playfield
        wi!("BgMaskAlpha", bg_mask_alpha);
        wi!("BgMaskPadding", bg_mask_padding);
        wi!("ColumnAlignment", column_alignment);
        wi!("ColumnSize", column_size);
        for (i, color) in self.column_colors.iter().enumerate() {
            add_key_value(&mut result, &format!("ColumnColor{}", i + 1), " = ", &color.to_str(), "\n");
        }
        wi!("ColumnLightingOffsetY", column_lighting_offset_y);
        wi!("ColumnLightingScale", column_lighting_scale);
        wi!("HitPosOffsetY", hit_pos_offset_y);
        wi!("ReceptorPosOffsetY", receptor_pos_offset_y);
        wb!("ReceptorsOverHitObjects", receptors_over_hit_objects);
        wi!("StageReceptorPadding", stage_receptor_padding);
        wi!("CoopPlayfieldPadding", coop_playfield_padding);

        // judgement / hit error
        wi!("HitErrorChevronSize", hit_error_chevron_size);
        wi!("HitErrorHeight", hit_error_height);
        wi!("HitErrorPosX", hit_error_pos_x);
        wi!("HitErrorPosY", hit_error_pos_y);
        wi!("HitErrorAlpha", hit_error_alpha);
        wi!("JudgementHitBurstFps", judgement_hit_burst_fps);
        wi!("JudgementBurstPosY", judgement_burst_pos_y);
        wi!("JudgementHitBurstBumpY", judgement_hit_burst_bump_y);
        wi!("JudgementHitBurstBumpTime", judgement_hit_burst_bump_time);
        wi!("JudgementHitBurstScale", judgement_hit_burst_scale);

        // lighting
        wi!("HitLightingX", hit_lighting_x);
        wi!("HitLightingY", hit_lighting_y);
        wi!("HitLightingFps", hit_lighting_fps);
        wi!("HitLightingScale", hit_lighting_scale);
        wb!("HitLightingColumnRotation", hit_lighting_column_rotation);
        wi!("HoldLightingFps", hold_lighting_fps);
        wi!("HoldLightingScale", hold_lighting_scale);
        wb!("HoldLightingColumnRotation", hold_lighting_column_rotation);

        // health bar
        if self.health_bar_keys_alignment != d.health_bar_keys_alignment {
            add_key_value(&mut result, "HealthBarKeysAlignment", " = ", self.health_bar_keys_alignment.to_str(), "\n");
        }
        if self.health_bar_type != d.health_bar_type {
            add_key_value(&mut result, "HealthBarType", " = ", self.health_bar_type.to_str(), "\n");
        }
        wi!("HealthBarPosOffsetX", health_bar_pos_offset_x);
        wi!("HealthBarPosOffsetY", health_bar_pos_offset_y);
        wi!("HealthBarScale", health_bar_scale);

        // hit bubbles
        if self.hit_bubbles_alignment != d.hit_bubbles_alignment {
            add_key_value(&mut result, "HitBubblesAlignment", " = ", self.hit_bubbles_alignment.to_str(), "\n");
        }
        if self.hit_bubbles_type != d.hit_bubbles_type {
            add_key_value(&mut result, "HitBubblesType", " = ", self.hit_bubbles_type.to_str(), "\n");
        }
        wi!("HitBubblesPosX", hit_bubbles_pos_x);
        wi!("HitBubblesPosY", hit_bubbles_pos_y);
        wi!("HitBubblesScale", hit_bubbles_scale);
        wi!("HitBubbleScale", hit_bubble_scale);
        wi!("HitBubbleBorderPadding", hit_bubble_border_padding);
        wi!("HitBubblePadding", hit_bubble_padding);
        if self.hit_bubbles_recorded_judgements != d.hit_bubbles_recorded_judgements {
            add_key_value(&mut result, "HitBubblesRecordedJudgements", " = ", &self.hit_bubbles_recorded_judgements.to_str(), "\n");
        }

        // combo
        wi!("ComboDisplayScale", combo_display_scale);
        wi!("ComboPosX", combo_pos_x);
        wi!("ComboPosY", combo_pos_y);
        wi!("ComboDisplayBumpY", combo_display_bump_y);
        wi!("ComboDisplayBumpTime", combo_display_bump_time);

        // rating
        wi!("RatingDisplayScale", rating_display_scale);
        wi!("RatingDisplayPosX", rating_display_pos_x);
        wi!("RatingDisplayPosY", rating_display_pos_y);

        // accuracy
        wi!("AccuracyDisplayScale", accuracy_display_scale);
        wi!("AccuracyDisplayPosX", accuracy_display_pos_x);
        wi!("AccuracyDisplayPosY", accuracy_display_pos_y);

        // kps
        wi!("KpsDisplayScale", kps_display_scale);
        wi!("KpsDisplayPosX", kps_display_pos_x);
        wi!("KpsDisplayPosY", kps_display_pos_y);

        // score display
        wi!("ScoreDisplayScale", score_display_scale);
        wi!("ScoreDisplayPosX", score_display_pos_x);
        wi!("ScoreDisplayPosY", score_display_pos_y);

        // multiplayer
        wi!("BattleRoyaleAlertPosX", battle_royale_alert_pos_x);
        wi!("BattleRoyaleAlertPosY", battle_royale_alert_pos_y);
        wi!("BattleRoyaleAlertScale", battle_royale_alert_scale);
        wi!("BattleRoyaleEliminatedPosX", battle_royale_eliminated_pos_x);
        wi!("BattleRoyaleEliminatedPosY", battle_royale_eliminated_pos_y);

        // judgement counter
        wi!("JudgementCounterAlpha", judgement_counter_alpha);
        wc!("JudgementCounterFontColor", judgement_counter_font_color);
        wi!("JudgementCounterSize", judgement_counter_size);
        wc!("JudgeColorMarv", judge_color_marv);
        wc!("JudgeColorPerf", judge_color_perf);
        wc!("JudgeColorGreat", judge_color_great);
        wc!("JudgeColorGood", judge_color_good);
        wc!("JudgeColorOkay", judge_color_okay);
        wc!("JudgeColorMiss", judge_color_miss);
        wi!("JudgementCounterPosX", judgement_counter_pos_x);
        wi!("JudgementCounterPosY", judgement_counter_pos_y);
        wi!("JudgementCounterPadding", judgement_counter_padding);
        wb!("JudgementCounterHorizontal", judgement_counter_horizontal);
        wb!("JudgementCounterFadeToAlpha", judgement_counter_fade_to_alpha);
        wb!("UseJudgementColorForNumbers", use_judgement_color_for_numbers);

        // stage timing bar
        wc!("SongTimeProgressActiveColor", song_time_progress_active_color);
        wc!("SongTimeProgressInactiveColor", song_time_progress_inactive_color);
        wi!("SongTimeProgressScale", song_time_progress_scale);
        wb!("SongTimeProgressPositionAtTop", song_time_progress_position_at_top);
        wc!("TimingLineColor", timing_line_color);

        // mini progress bar
        wb!("ShowMiniSongBar", show_mini_song_bar);
        wi!("MiniSongBarDisplayPosX", mini_song_bar_display_pos_x);
        wi!("MiniSongBarDisplayPosY", mini_song_bar_display_pos_y);
        wi!("MiniSongBarDisplayWidthFactor", mini_song_bar_display_width_factor);
        wi!("MiniSongBarDisplayHeight", mini_song_bar_display_height);

        wb!("UseFallback", use_fallback);
        wl!("HitobjectFallbacks", hitobject_fallbacks);
        wl!("HoldbodyFallbacks", holdbody_fallbacks);
        wl!("HoldendFallbacks", holdend_fallbacks);
        wl!("ReceptorFallbacks", receptor_fallbacks);

        result
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_class = QuaKeymode))]
impl Keymode {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromStr))]
    pub fn wasm_from_str(content: &str) -> Result<Keymode, String> {
        Self::from_str(content).map_err(|e| e.to_string())
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toStr))]
    pub fn wasm_to_str(&self) -> String {
        self.to_str()
    }
}

impl KeymodeInvariant for Keymode {
    fn get_keymode(&self) -> u8 { self.keymode }

    fn shared_km_str() -> StringPattern {
        StringPattern::new("shared")
    }

    fn get_receptors(&self) -> Vec<String> {
        self.saturate_per_lane(&dynamic_assets::Receptors::UP)
    }
    fn get_receptors_down(&self) -> Vec<String> {
        self.saturate_per_lane(&dynamic_assets::Receptors::DOWN)
    }
    fn get_normal_notes(&self) -> Vec<String> {
        self.saturate_per_lane(&dynamic_assets::Notes::HIT_OBJECT)
    }
    fn get_long_note_heads(&self) -> Vec<String> {
        self.saturate_per_lane(&dynamic_assets::Notes::HOLD_HIT_OBJECT)
    }
    fn get_long_note_bodies(&self) -> Vec<String> {
        self.saturate_per_lane(&dynamic_assets::Notes::HOLD_BODY)
    }
    fn get_long_note_tails(&self) -> Vec<String> {
        self.saturate_per_lane(&dynamic_assets::Notes::HOLD_END)
    }
    fn get_normal_mines(&self) -> Vec<String> {
        self.saturate_per_lane(&dynamic_assets::Mines::MINE)
    }

    fn primary_fallback(&self, lane: usize) -> LaneFallback {
        self.fallback_for_lane(lane)
    }
    fn secondary_fallback(&self, lane: usize) -> LaneFallback {
        self.fallback_for_lane(lane)
    }
    fn middle_fallback(&self, lane: usize) -> LaneFallback {
        self.fallback_for_lane(lane)
    }
}

impl Keymode {
    fn saturate_per_lane(&self, pattern: &StringPattern) -> Vec<String> {
        let pattern_str = pattern.raw();
        let keys_str = self.keymode.to_string();
        
        (1..=self.keymode as usize)
            .map(|lane| {
                pattern_str
                    .replace("{keys}", &keys_str)
                    .replace("{lane}", &lane.to_string())
            })
            .collect()
    }

    fn fallback_for_lane(&self, lane: usize) -> LaneFallback {
        let keys_str = self.keymode.to_string();
        let lane_str = lane.to_string();
        
        let saturate = |pattern: &StringPattern| -> String {
            pattern.raw()
                .replace("{keys}", &keys_str)
                .replace("{lane}", &lane_str)
        };

        LaneFallback {
            receptor: saturate(&dynamic_assets::Receptors::UP),
            receptor_down: saturate(&dynamic_assets::Receptors::DOWN),
            normal_note: saturate(&dynamic_assets::Notes::HIT_OBJECT),
            long_note_head: saturate(&dynamic_assets::Notes::HOLD_HIT_OBJECT),
            long_note_body: saturate(&dynamic_assets::Notes::HOLD_BODY),
            long_note_tail: saturate(&dynamic_assets::Notes::HOLD_END),
            normal_mine: saturate(&dynamic_assets::Mines::MINE),
        }
    }
}
