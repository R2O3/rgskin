use std::collections::HashSet;
use crate::utils::serde::{
    add_key_value, add_key_value_if_not_default, parse_bool, parse_key_value, parse_u32_list, serialize_bool, serialize_bool_vec_if_not_empty, serialize_u32_slice, serialize_vec_if_not_empty
};
use crate::utils::io::{path_to_unix, path_to_win};
use crate::common::color::Rgba;

#[derive(Clone, Debug)]
pub struct Keymode {
    pub keymode: u8,

    pub keys_under_notes: bool,
    pub judgement_line: bool,
    pub upside_down: bool,
    pub special_style: u8,
    pub combo_burst_style: u8,
    pub split_stages: Option<bool>,
    pub stage_separation: f32,
    pub separate_score: bool,

    pub hit_position: u32,
    pub light_position: u32,
    pub score_position: Option<u32>,
    pub combo_position: Option<u32>,

    pub column_start: u32,
    pub column_right: u32,
    pub column_line_width: Vec<u32>,
    pub column_width: Vec<u32>,
    pub column_spacing: Vec<u32>,

    pub barline_height: f32,
    pub lighting_n_width: Vec<u32>,
    pub lighting_l_width: Vec<u32>,
    pub width_for_note_height_scale: Option<u32>,

    pub light_frame_per_second: u32,

    pub key_flip_when_upside_down: bool,
    pub key_flip_when_upside_down_columns: Vec<bool>,
    pub key_flip_when_upside_down_down_columns: Vec<bool>,
    pub note_flip_when_upside_down: bool,
    pub note_flip_when_upside_down_columns: Vec<bool>,
    pub note_flip_when_upside_down_h_columns: Vec<bool>,
    pub note_flip_when_upside_down_l_columns: Vec<bool>,
    pub note_flip_when_upside_down_t_columns: Vec<bool>,

    pub note_body_style: u8,
    pub note_body_style_columns: Vec<u8>,

    pub colours: Vec<Rgba>,
    pub colour_lights: Vec<Rgba>,
    pub colour_column_line: Rgba,
    pub colour_barline: Rgba,
    pub colour_judgement_line: Rgba,
    pub colour_key_warning: Rgba,
    pub colour_hold: Rgba,
    pub colour_break: Rgba,

    pub receptor_images: Vec<String>,
    pub receptor_images_down: Vec<String>,
    pub normal_note_images: Vec<String>,
    pub long_note_head_images: Vec<String>,
    pub long_note_body_images: Vec<String>,
    pub long_note_tail_images: Vec<String>,

    pub stage_left: String,
    pub stage_right: String,
    pub stage_bottom: String,
    pub stage_hint: String,
    pub stage_light: String,
    pub lighting_n: String,
    pub lighting_l: String,
    pub warning_arrow: String,

    pub hit0: String,
    pub hit50: String,
    pub hit100: String,
    pub hit200: String,
    pub hit300: String,
    pub hit300g: String,
}

impl Default for Keymode {
    fn default() -> Self {
        Self {
            keymode: 0,
            keys_under_notes: false,
            judgement_line: false,
            upside_down: false,
            special_style: 0,
            combo_burst_style: 1,
            split_stages: None,
            stage_separation: 40.0,
            separate_score: true,
            hit_position: 458,
            light_position: 413,
            score_position: None,
            combo_position: None,
            column_start: 136,
            column_right: 19,
            column_line_width: vec![2; 5],
            column_width: vec![30; 4],
            column_spacing: vec![0; 4],
            barline_height: 1.2,
            lighting_n_width: Vec::new(),
            lighting_l_width: Vec::new(),
            width_for_note_height_scale: None,
            light_frame_per_second: 24,
            key_flip_when_upside_down: false,
            key_flip_when_upside_down_columns: Vec::new(),
            key_flip_when_upside_down_down_columns: Vec::new(),
            note_flip_when_upside_down: false,
            note_flip_when_upside_down_columns: Vec::new(),
            note_flip_when_upside_down_h_columns: Vec::new(),
            note_flip_when_upside_down_l_columns: Vec::new(),
            note_flip_when_upside_down_t_columns: Vec::new(),
            note_body_style: 1,
            note_body_style_columns: Vec::new(),
            colours: vec![Rgba { red: 0, green: 0, blue: 0, alpha: 255 }; 18],
            colour_lights: vec![Rgba { red: 55, green: 255, blue: 255, alpha: 255 }; 18],
            colour_column_line: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            colour_barline: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            colour_judgement_line: Rgba { red: 255, green: 255, blue: 255, alpha: 255 },
            colour_key_warning: Rgba { red: 0, green: 0, blue: 0, alpha: 255 },
            colour_hold: Rgba { red: 255, green: 191, blue: 51, alpha: 255 },
            colour_break: Rgba { red: 255, green: 0, blue: 0, alpha: 255 },
            receptor_images: vec![String::new(); 18],
            receptor_images_down: vec![String::new(); 18],
            normal_note_images: vec![String::new(); 18],
            long_note_head_images: vec![String::new(); 18],
            long_note_body_images: vec![String::new(); 18],
            long_note_tail_images: vec![String::new(); 18],
            stage_left: String::new(),
            stage_right: String::new(),
            stage_bottom: String::new(),
            stage_hint: String::new(),
            stage_light: String::new(),
            lighting_n: String::new(),
            lighting_l: String::new(),
            warning_arrow: String::new(),
            hit0: String::new(),
            hit50: String::new(),
            hit100: String::new(),
            hit200: String::new(),
            hit300: String::new(),
            hit300g: String::new(),
        }
    }
}

impl Keymode {
    fn parse_indexed_image(key: &str, value: &str, prefix: &str, suffix: &str, target: &mut Vec<String>) {
        if let Some(index_str) = key.strip_prefix(prefix).and_then(|s| s.strip_suffix(suffix)) {
            let numeric_part: String = index_str.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(index) = numeric_part.parse::<usize>() {
                if index < target.len() {
                    target[index] = value.to_string();
                }
            }
        }
    }

    fn parse_indexed_bool(key: &str, value: &str, prefix: &str, suffix: &str, target: &mut Vec<bool>) {
        if let Some(index_str) = key.strip_prefix(prefix).and_then(|s| s.strip_suffix(suffix)) {
            let numeric_part: String = index_str.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(index) = numeric_part.parse::<usize>() {
                while target.len() <= index {
                    target.push(false);
                }
                target[index] = parse_bool(value);
            }
        }
    }

    fn parse_indexed_u8(key: &str, value: &str, prefix: &str, suffix: &str, target: &mut Vec<u8>) {
        if let Some(index_str) = key.strip_prefix(prefix).and_then(|s| s.strip_suffix(suffix)) {
            let numeric_part: String = index_str.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(index) = numeric_part.parse::<usize>() {
                if let Ok(val) = value.parse::<u8>() {
                    while target.len() <= index {
                        target.push(0);
                    }
                    target[index] = val;
                }
            }
        }
    }

    fn parse_indexed_color(key: &str, value: &str, prefix: &str, target: &mut Vec<Rgba>) {
        if let Some(index_str) = key.strip_prefix(prefix) {
            let numeric_part: String = index_str.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(index) = numeric_part.parse::<usize>() {
                if index > 0 {
                    let array_index = index - 1;
                    if let Ok(color) = Rgba::from_str(value) {
                        while target.len() <= array_index {
                            target.push(Rgba { red: 0, green: 0, blue: 0, alpha: 255 });
                        }
                        target[array_index] = color;
                    }
                }
            }
        }
    }

    fn alloc_vecs(&mut self) {
        let key_count = self.keymode as usize;
        self.column_line_width = vec![2; key_count + 1];
        self.column_width = vec![30; key_count];
        self.column_spacing = vec![0; key_count];
        self.receptor_images = vec![String::new(); key_count];
        self.receptor_images_down = vec![String::new(); key_count];
        self.normal_note_images = vec![String::new(); key_count];
        self.long_note_head_images = vec![String::new(); key_count];
        self.long_note_body_images = vec![String::new(); key_count];
        self.long_note_tail_images = vec![String::new(); key_count];
        self.colours = vec![Rgba { red: 0, green: 0, blue: 0, alpha: 255 }; key_count];
        self.colour_lights = vec![Rgba { red: 55, green: 255, blue: 255, alpha: 255 }; key_count];
    }

    pub fn from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut keymode = Self::default();

        for line in content.lines() {
            let line = line.trim();
            
            if line.is_empty() || line.starts_with("//") || line.starts_with('[') {
                continue;
            }

            let (key_str, value_str) = parse_key_value(line);

            match key_str {
                "Keys" => {
                    keymode.keymode = value_str.parse()?;
                    keymode.alloc_vecs();
                }
                "KeysUnderNotes" => keymode.keys_under_notes = parse_bool(value_str),
                "JudgementLine" => keymode.judgement_line = parse_bool(value_str),
                "UpsideDown" => keymode.upside_down = parse_bool(value_str),
                "SpecialStyle" => keymode.special_style = value_str.parse().unwrap_or(0),
                "ComboBurstStyle" => {
                    keymode.combo_burst_style = match value_str.to_lowercase().as_str() {
                        "left" => 0,
                        "right" => 1,
                        "both" => 2,
                        _ => value_str.parse().unwrap_or(1),
                    }
                }
                "SplitStages" => keymode.split_stages = Some(parse_bool(value_str)),
                "StageSeparation" => keymode.stage_separation = value_str.parse().unwrap_or(40.0),
                "SeparateScore" => keymode.separate_score = parse_bool(value_str),
                "HitPosition" => keymode.hit_position = value_str.parse()?,
                "LightPosition" => keymode.light_position = value_str.parse().unwrap_or(413),
                "ScorePosition" => keymode.score_position = Some(value_str.parse()?),
                "ComboPosition" => keymode.combo_position = Some(value_str.parse()?),
                "ColumnStart" => keymode.column_start = value_str.parse()?,
                "ColumnRight" => keymode.column_right = value_str.parse()?,
                "ColumnLineWidth" => keymode.column_line_width = parse_u32_list(value_str),
                "ColumnWidth" => keymode.column_width = parse_u32_list(value_str),
                "ColumnSpacing" => keymode.column_spacing = parse_u32_list(value_str),
                "BarlineHeight" => keymode.barline_height = value_str.parse().unwrap_or(1.2),
                "LightingNWidth" => keymode.lighting_n_width = parse_u32_list(value_str),
                "LightingLWidth" => keymode.lighting_l_width = parse_u32_list(value_str),
                "WidthForNoteHeightScale" => keymode.width_for_note_height_scale = Some(value_str.parse()?),
                "LightFramePerSecond" => keymode.light_frame_per_second = value_str.parse().unwrap_or(24),
                "KeyFlipWhenUpsideDown" => keymode.key_flip_when_upside_down = parse_bool(value_str),
                "NoteFlipWhenUpsideDown" => keymode.note_flip_when_upside_down = parse_bool(value_str),
                "NoteBodyStyle" => keymode.note_body_style = value_str.parse().unwrap_or(1),
                "ColourColumnLine" => {
                    if let Ok(color) = Rgba::from_str(value_str) {
                        keymode.colour_column_line = color;
                    }
                }
                "ColourBarline" => {
                    if let Ok(color) = Rgba::from_str(value_str) {
                        keymode.colour_barline = color;
                    }
                }
                "ColourJudgementLine" => {
                    if let Ok(color) = Rgba::from_str(value_str) {
                        keymode.colour_judgement_line = color;
                    }
                }
                "ColourKeyWarning" => {
                    if let Ok(color) = Rgba::from_str(value_str) {
                        keymode.colour_key_warning = color;
                    }
                }
                "ColourHold" => {
                    if let Ok(color) = Rgba::from_str(value_str) {
                        keymode.colour_hold = color;
                    }
                }
                "ColourBreak" => {
                    if let Ok(color) = Rgba::from_str(value_str) {
                        keymode.colour_break = color;
                    }
                }
                "StageLeft" => keymode.stage_left = path_to_unix(&value_str.to_string()),
                "StageRight" => keymode.stage_right = path_to_unix(&value_str.to_string()),
                "StageBottom" => keymode.stage_bottom = path_to_unix(&value_str.to_string()),
                "StageHint" => keymode.stage_hint = path_to_unix(&value_str.to_string()),
                "StageLight" => keymode.stage_light = path_to_unix(&value_str.to_string()),
                "LightingN" => keymode.lighting_n = path_to_unix(&value_str.to_string()),
                "LightingL" => keymode.lighting_l = path_to_unix(&value_str.to_string()),
                "WarningArrow" => keymode.warning_arrow = path_to_unix(&value_str.to_string()),
                "Hit0" => keymode.hit0 = path_to_unix(&value_str.to_string()),
                "Hit50" => keymode.hit50 = path_to_unix(&value_str.to_string()),
                "Hit100" => keymode.hit100 = path_to_unix(&value_str.to_string()),
                "Hit200" => keymode.hit200 = path_to_unix(&value_str.to_string()),
                "Hit300" => keymode.hit300 = path_to_unix(&value_str.to_string()),
                "Hit300g" => keymode.hit300g = path_to_unix(&value_str.to_string()),
                _ => {
                    if key_str.starts_with("KeyImage") {
                        if key_str.ends_with("D") {
                            Self::parse_indexed_image(key_str, &path_to_unix(value_str), "KeyImage", "D", &mut keymode.receptor_images_down);
                        } else {
                            Self::parse_indexed_image(key_str, &path_to_unix(value_str), "KeyImage", "", &mut keymode.receptor_images);
                        }
                    } else if key_str.starts_with("NoteImage") {
                        if key_str.contains("H") {
                            Self::parse_indexed_image(key_str, &path_to_unix(value_str), "NoteImage", "H", &mut keymode.long_note_head_images);
                        } else if key_str.contains("L") {
                            Self::parse_indexed_image(key_str, &path_to_unix(value_str), "NoteImage", "L", &mut keymode.long_note_body_images);
                        } else if key_str.contains("T") {
                            Self::parse_indexed_image(key_str, &path_to_unix(value_str), "NoteImage", "T", &mut keymode.long_note_tail_images);
                        } else {
                            Self::parse_indexed_image(key_str, &path_to_unix(value_str), "NoteImage", "", &mut keymode.normal_note_images);
                        }
                    } else if key_str.starts_with("Colour") && !key_str.contains("Light") {
                        Self::parse_indexed_color(key_str, value_str, "Colour", &mut keymode.colours);
                    } else if key_str.starts_with("ColourLight") {
                        Self::parse_indexed_color(key_str, value_str, "ColourLight", &mut keymode.colour_lights);
                    } else if key_str.starts_with("KeyFlipWhenUpsideDown") {
                        if key_str.ends_with("D") {
                            Self::parse_indexed_bool(key_str, value_str, "KeyFlipWhenUpsideDown", "D", &mut keymode.key_flip_when_upside_down_down_columns);
                        } else {
                            Self::parse_indexed_bool(key_str, value_str, "KeyFlipWhenUpsideDown", "", &mut keymode.key_flip_when_upside_down_columns);
                        }
                    } else if key_str.starts_with("NoteFlipWhenUpsideDown") {
                        if key_str.contains("H") {
                            Self::parse_indexed_bool(key_str, value_str, "NoteFlipWhenUpsideDown", "H", &mut keymode.note_flip_when_upside_down_h_columns);
                        } else if key_str.contains("L") {
                            Self::parse_indexed_bool(key_str, value_str, "NoteFlipWhenUpsideDown", "L", &mut keymode.note_flip_when_upside_down_l_columns);
                        } else if key_str.contains("T") {
                            Self::parse_indexed_bool(key_str, value_str, "NoteFlipWhenUpsideDown", "T", &mut keymode.note_flip_when_upside_down_t_columns);
                        } else {
                            Self::parse_indexed_bool(key_str, value_str, "NoteFlipWhenUpsideDown", "", &mut keymode.note_flip_when_upside_down_columns);
                        }
                    } else if key_str.starts_with("NoteBodyStyle") {
                        Self::parse_indexed_u8(key_str, value_str, "NoteBodyStyle", "", &mut keymode.note_body_style_columns);
                    }
                }
            }
        }

        Ok(keymode)
    }

    pub fn to_str(&self) -> String {
        let mut result = String::new();
        let default = Keymode::default();
        
        add_key_value(&mut result, "Keys", ": ", &self.keymode.to_string(), "\n");
        result.push('\n');
        
        // toggles
        result.push_str(&format!("// --=  Toggles  |  {}k  =--\n", &self.keymode));
        add_key_value(&mut result, "KeysUnderNotes", ": ", &serialize_bool(self.keys_under_notes).to_string(), "\n");
        add_key_value_if_not_default::<bool>(&mut result, "JudgementLine", &self.judgement_line, &default.judgement_line);
        add_key_value_if_not_default::<bool>(&mut result, "UpsideDown", &self.upside_down, &default.upside_down);
        add_key_value_if_not_default::<bool>(&mut result, "SeparateScore", &self.separate_score, &default.separate_score);
        if let Some(split_stages) = self.split_stages {
            add_key_value(&mut result, "SplitStages", ": ", &serialize_bool(split_stages).to_string(), "\n");
        }
        add_key_value_if_not_default::<bool>(&mut result, "KeyFlipWhenUpsideDown", &self.key_flip_when_upside_down, &default.key_flip_when_upside_down);
        add_key_value_if_not_default::<bool>(&mut result, "NoteFlipWhenUpsideDown", &self.note_flip_when_upside_down, &default.note_flip_when_upside_down);
        result.push('\n');

        // position
        result.push_str(&format!("// --=  Position  |  {}k  =--\n", &self.keymode));
        add_key_value(&mut result, "HitPosition", ": ", &self.hit_position.to_string(), "\n");
        add_key_value_if_not_default::<u32>(&mut result, "LightPosition", &self.light_position, &default.light_position);
        
        if let Some(score_position) = self.score_position {
            add_key_value(&mut result, "ScorePosition", ": ", &score_position.to_string(), "\n");
        }

        if let Some(combo_position) = self.combo_position {
            add_key_value(&mut result, "ComboPosition", ": ", &combo_position.to_string(), "\n");
        }
        add_key_value_if_not_default::<f32>(&mut result, "BarlineHeight", &self.barline_height, &default.barline_height);
        result.push('\n');

        // column
        result.push_str(&format!("// --=  Column  |  {}k  =--\n", &self.keymode));
        add_key_value_if_not_default::<u32>(&mut result, "ColumnStart", &self.column_start, &default.column_start);
        add_key_value_if_not_default::<u32>(&mut result, "ColumnRight", &self.column_right, &default.column_right);
        if let Some(width_for_note_height_scale) = self.width_for_note_height_scale {
            add_key_value(&mut result, "WidthForNoteHeightScale", ": ", &width_for_note_height_scale.to_string(), "\n");
        }
        
        if self.column_line_width != default.column_line_width {
            add_key_value(&mut result, "ColumnLineWidth", ": ", &serialize_u32_slice(&self.column_line_width), "\n");
        }
        
        if self.column_width != default.column_width {
            add_key_value(&mut result, "ColumnWidth", ": ", &serialize_u32_slice(&self.column_width), "\n");
        }
        
        if self.column_spacing != default.column_spacing {
            add_key_value(&mut result, "ColumnSpacing", ": ", &serialize_u32_slice(&self.column_spacing), "\n");
        }
        result.push('\n');

        // stage
        result.push_str(&format!("// --=  Stage  |  {}k  =--\n", &self.keymode));
        add_key_value_if_not_default::<f32>(&mut result, "StageSeparation", &self.stage_separation, &default.stage_separation);
        add_key_value_if_not_default::<String>(&mut result, "StageLeft", &self.stage_left, &default.stage_left);
        add_key_value_if_not_default::<String>(&mut result, "StageRight", &self.stage_right, &default.stage_right);
        add_key_value_if_not_default::<String>(&mut result, "StageBottom", &self.stage_bottom, &default.stage_bottom);
        add_key_value_if_not_default::<String>(&mut result, "StageHint", &self.stage_hint, &default.stage_hint);
        add_key_value_if_not_default::<String>(&mut result, "StageLight", &self.stage_light, &default.stage_light);
        result.push('\n');

        // style
        result.push_str(&format!("// --=  Style  |  {}k  =--\n", &self.keymode));
        add_key_value_if_not_default::<u8>(&mut result, "NoteBodyStyle", &self.note_body_style, &default.note_body_style);
        add_key_value_if_not_default::<u8>(&mut result, "SpecialStyle", &self.special_style, &default.special_style);
        add_key_value_if_not_default::<u8>(&mut result, "ComboBurstStyle", &self.combo_burst_style, &default.combo_burst_style);
        serialize_vec_if_not_empty(&mut result, &self.note_body_style_columns, "NoteBodyStyleColumns");
        result.push('\n');

        // lighting
        result.push_str(&format!("// --=  Lighting  |  {}k  =--\n", &self.keymode));
        if !self.lighting_n_width.is_empty() {
            add_key_value(&mut result, "LightingNWidth", ": ", &serialize_u32_slice(&self.lighting_n_width), "\n");
        }
        if !self.lighting_l_width.is_empty() {
            add_key_value(&mut result, "LightingLWidth", ": ", &serialize_u32_slice(&self.lighting_l_width), "\n");
        }
        add_key_value_if_not_default::<String>(&mut result, "LightingN", &self.lighting_n, &default.lighting_n);
        add_key_value_if_not_default::<String>(&mut result, "LightingL", &self.lighting_l, &default.lighting_l);
        add_key_value_if_not_default::<u32>(&mut result, "LightFramePerSecond", &self.light_frame_per_second, &default.light_frame_per_second);
        result.push('\n');
        
        // colors
        result.push_str(&format!("// --=  Colors  |  {}k  =--\n", &self.keymode));
        if self.colours != default.colours {
            for (i, color) in self.colours.iter().enumerate() {
                if i < default.colours.len() && color != &default.colours[i] {
                    add_key_value(&mut result, &format!("Colour{}", i), ": ", &color.to_str(), "\n");
                } else if i >= default.colours.len() {
                    add_key_value(&mut result, &format!("Colour{}", i), ": ", &color.to_str(), "\n");
                }
            }
        }
        
        if self.colour_lights != default.colour_lights {
            for (i, color) in self.colour_lights.iter().enumerate() {
                if i < default.colour_lights.len() && color != &default.colour_lights[i] {
                    add_key_value(&mut result, &format!("ColourLight{}", i), ": ", &color.to_str(), "\n");
                } else if i >= default.colour_lights.len() {
                    add_key_value(&mut result, &format!("ColourLight{}", i), ": ", &color.to_str(), "\n");
                }
            }
        }
        
        if self.colour_column_line != default.colour_column_line {
            add_key_value(&mut result, "ColourColumnLine", ": ", &self.colour_column_line.to_str(), "\n");
        }
        
        if self.colour_barline != default.colour_barline {
            add_key_value(&mut result, "ColourBarline", ": ", &self.colour_barline.to_str(), "\n");
        }
        
        if self.colour_judgement_line != default.colour_judgement_line {
            add_key_value(&mut result, "ColourJudgementLine", ": ", &self.colour_judgement_line.to_str(), "\n");
        }
        
        if self.colour_key_warning != default.colour_key_warning {
            add_key_value(&mut result, "ColourKeyWarning", ": ", &self.colour_key_warning.to_str(), "\n");
        }
        
        if self.colour_hold != default.colour_hold {
            add_key_value(&mut result, "ColourHold", ": ", &self.colour_hold.to_str(), "\n");
        }
        
        if self.colour_break != default.colour_break {
            add_key_value(&mut result, "ColourBreak", ": ", &self.colour_break.to_str(), "\n");
        }
        
        let serialize_string_vec = |result: &mut String, vec: &[String], default_vec: &[String], prefix: &str, suffix: &str| {
            for (i, s) in vec.iter().enumerate() {
                if !s.is_empty() && (i >= default_vec.len() || s != &default_vec[i]) {
                    add_key_value(result, &format!("{}{}{}", prefix, i, suffix), ": ", &path_to_win(s), "\n");
                }
            }
        };
        result.push('\n');

        // receptors
        result.push_str(&format!("// --=  Receptors  |  {}k  =--\n", &self.keymode));
        serialize_string_vec(&mut result, &self.receptor_images, &default.receptor_images, "KeyImage", "");
        serialize_string_vec(&mut result, &self.receptor_images_down, &default.receptor_images_down, "KeyImage", "D");
        result.push('\n');

        // notes
        result.push_str(&format!("// --=  Notes  |  {}k  =--\n", &self.keymode));
        serialize_string_vec(&mut result, &self.normal_note_images, &default.normal_note_images, "NoteImage", "");
        serialize_string_vec(&mut result, &self.long_note_head_images, &default.long_note_head_images, "NoteImage", "H");
        serialize_string_vec(&mut result, &self.long_note_body_images, &default.long_note_body_images, "NoteImage", "L");
        serialize_string_vec(&mut result, &self.long_note_tail_images, &default.long_note_tail_images, "NoteImage", "T");
        result.push('\n');

        // misc
        result.push_str(&format!("// --=  Misc  |  {}k  =--\n", &self.keymode));
        add_key_value_if_not_default::<String>(&mut result, "WarningArrow", &self.warning_arrow, &default.warning_arrow);
        add_key_value_if_not_default::<String>(&mut result, "Hit0", &self.hit0, &default.hit0);
        add_key_value_if_not_default::<String>(&mut result, "Hit50", &self.hit50, &default.hit50);
        add_key_value_if_not_default::<String>(&mut result, "Hit100", &self.hit100, &default.hit100);
        add_key_value_if_not_default::<String>(&mut result, "Hit200", &self.hit200, &default.hit200);
        add_key_value_if_not_default::<String>(&mut result, "Hit300", &self.hit300, &default.hit300);
        add_key_value_if_not_default::<String>(&mut result, "Hit300g", &self.hit300g, &default.hit300g);
        result.push('\n');

        serialize_bool_vec_if_not_empty(&mut result, &self.key_flip_when_upside_down_columns, "KeyFlipWhenUpsideDownColumns");
        serialize_bool_vec_if_not_empty(&mut result, &self.key_flip_when_upside_down_down_columns, "KeyFlipWhenUpsideDownDownColumns");
        serialize_bool_vec_if_not_empty(&mut result, &self.note_flip_when_upside_down_columns, "NoteFlipWhenUpsideDownColumns");
        serialize_bool_vec_if_not_empty(&mut result, &self.note_flip_when_upside_down_h_columns, "NoteFlipWhenUpsideDownHColumns");
        serialize_bool_vec_if_not_empty(&mut result, &self.note_flip_when_upside_down_l_columns, "NoteFlipWhenUpsideDownLColumns");
        serialize_bool_vec_if_not_empty(&mut result, &self.note_flip_when_upside_down_t_columns, "NoteFlipWhenUpsideDownTColumns");
        
        result
    }

    pub fn get_texture_paths(&self) -> HashSet<String> {
        let mut result: HashSet<String> = HashSet::new();

        let insert_vec = |result: &mut HashSet<String>, vec: &[String]| {
            result.extend(vec.iter().filter(|s| !s.is_empty()).cloned());
        };

        let insert_with_fallback = |result: &mut HashSet<String>, custom: &str, fallback: &str| {
            if custom.trim().is_empty() {
                result.insert(fallback.to_string());
            } else {
                result.insert(custom.to_string());
            }
        };

        insert_vec(&mut result, &self.receptor_images);
        insert_vec(&mut result, &self.receptor_images_down);
        insert_vec(&mut result, &self.normal_note_images);
        insert_vec(&mut result, &self.long_note_head_images);
        insert_vec(&mut result, &self.long_note_body_images);
        insert_vec(&mut result, &self.long_note_tail_images);

        insert_with_fallback(&mut result, &self.stage_left, "mania-stage-left");
        insert_with_fallback(&mut result, &self.stage_right, "mania-stage-right");
        insert_with_fallback(&mut result, &self.stage_light, "mania-stage-light");
        insert_with_fallback(&mut result, &self.stage_hint, "mania-stage-hint");
        insert_with_fallback(&mut result, &self.stage_bottom, "mania-stage-bottom");
        
        insert_with_fallback(&mut result, &self.lighting_n, "lightingN");
        insert_with_fallback(&mut result, &self.lighting_l, "lightingL");
        
        insert_with_fallback(&mut result, &self.hit0, "mania-hit0");
        insert_with_fallback(&mut result, &self.hit50, "mania-hit50");
        insert_with_fallback(&mut result, &self.hit100, "mania-hit100");
        insert_with_fallback(&mut result, &self.hit200, "mania-hit200");
        insert_with_fallback(&mut result, &self.hit300, "mania-hit300");
        insert_with_fallback(&mut result, &self.hit300g, "mania-hit300g");
        
        insert_with_fallback(&mut result, &self.warning_arrow, "mania-warningarrow");

        result.insert("lighting".to_string());
        result.insert("lightingA".to_string());
        result.insert("lightingL".to_string());
        result.insert("lightingN".to_string());

        result.insert("comboburst".to_string());
        result.insert("mania-stage-hint".to_string());
        result.insert("star".to_string());
        result.insert("star2".to_string());

        result.insert("scorebar-bg".to_string());
        result.insert("scorebar-colour".to_string());

        result
    }
}