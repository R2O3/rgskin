use std::sync::{Arc, RwLock};

use crate::common::alignment::*;
use crate::common::color::Rgba;
use crate::common::vector::*;
use crate::extensions::TextureArcExt;
use crate::generic::Gameplay;
use crate::image_proc::proc::{dist_from_bottom, flip_vertical, rotate_90_deg_ccw, rotate_90_deg_cw, to_osu_column, to_osu_column_draw};
use crate::io::Store;
use crate::io::texture::{Texture, TextureProcessor};
use crate::osu::{self, General, OsuSkin, SkinIni};
use crate::skin::generic::layout::{HUDLayout, KeymodeLayout};
use crate::skin::generic::{elements::*, Keymode, Metadata, GenericManiaSkin};
use crate::traits::ManiaSkinConfig;
use crate::utils::math::Resizer;
use crate::utils::osu::OsuDimensions;

pub fn to_generic_mania(skin: OsuSkin) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures;
    let mut keymodes: Vec<Keymode> = Vec::new();

    textures.insert(Texture::from_blank("blank".to_string()));
    let blank_texture = textures.get_shared("blank").unwrap();

    let metadata = Metadata {
        name: skin.skin_ini.general.name.clone(),
        creator: skin.skin_ini.general.author.clone(),
        version: skin.skin_ini.general.version.clone(),
        center_cursor: skin.skin_ini.general.cursor_centre.clone()
    };

    let mut receptor_processor = TextureProcessor::<i32>::new();
    let mut tail_processor = TextureProcessor::<()>::new();

    for keymode in &skin.skin_ini.keymodes {
        let key_count = keymode.keymode as usize;
        let average_column_width = keymode.column_width.iter().sum::<u32>() / keymode.column_width.len() as u32;
        let mut max_receptor_offset = 0;

        let receptor_up_elements: Vec<ReceptorUp> = keymode.receptor_images
            .iter()
            .map(|path| {
                if !path.is_empty() {
                    if let Some(texture) = textures.get_shared(path) {
                        let offset = receptor_processor.process_once(&texture, |arc_texture| {
                            let offset = arc_texture.with_image(|img| dist_from_bottom(img, 0.1));
                            if let Err(e) = to_osu_column_draw(arc_texture, average_column_width) {
                                eprintln!("Failed to process receptor texture: {}", e);
                            }
                            offset.try_into().unwrap() // TODO: potential panic check later
                        });
                        max_receptor_offset = max_receptor_offset.max(offset);
                        ReceptorUp::new(texture)
                    } else {
                        ReceptorUp::new(Arc::clone(&blank_texture))
                    }
                } else {
                    ReceptorUp::new(Arc::clone(&blank_texture))
                }
            })
            .collect();

        let receptor_down_elements: Vec<ReceptorDown> = keymode.receptor_images_down
            .iter()
            .map(|path| {
                if !path.is_empty() {
                    if let Some(texture) = textures.get_shared(path) {
                        let offset = receptor_processor.process_once(&texture, |arc_texture| {
                            let offset = arc_texture.with_image(|img| dist_from_bottom(img, 0.1));
                            if let Err(e) = to_osu_column_draw(arc_texture, average_column_width) {
                                eprintln!("Failed to process receptor texture: {}", e);
                            }
                            offset.try_into().unwrap()
                        });
                        max_receptor_offset = max_receptor_offset.max(offset);
                        ReceptorDown::new(texture)
                    } else {
                        ReceptorDown::new(Arc::clone(&blank_texture))
                    }
                } else {
                    ReceptorDown::new(Arc::clone(&blank_texture))
                }
            })
            .collect();

        let normal_note_elements: Vec<NormalNote> = keymode.normal_note_images
            .iter()
            .map(|path| {
                if !path.is_empty() && textures.contains(path) {
                    NormalNote::new(textures.get_shared(path).unwrap())
                } else {
                    NormalNote::new(Arc::clone(&blank_texture))
                }
            })
            .collect();

        let long_note_head_elements: Vec<LongNoteHead> = keymode.long_note_head_images
            .iter()
            .map(|path| {
                if !path.is_empty() && textures.contains(path) {
                    LongNoteHead::new(textures.get_shared(path).unwrap())
                } else {
                    LongNoteHead::new(Arc::clone(&blank_texture))
                }
            })
            .collect();

        let long_note_body_elements: Vec<LongNoteBody> = keymode.long_note_body_images
            .iter()
            .map(|path| {
                if !path.is_empty() && textures.contains(path) {
                    LongNoteBody::new(textures.get_shared(path).unwrap())
                } else {
                    LongNoteBody::new(Arc::clone(&blank_texture))
                }
            })
            .collect();

        let long_note_tail_elements: Vec<LongNoteTail> = keymode.long_note_tail_images
            .iter()
            .map(|path| {
                if !path.is_empty() {
                    if let Some(texture) = textures.get_shared(path) {
                        tail_processor.process_once_void(&texture, |arc_texture| {
                            flip_vertical(arc_texture);
                        });
                        LongNoteTail::new(texture)
                    } else {
                        LongNoteTail::new(Arc::clone(&blank_texture))
                    }
                } else {
                    LongNoteTail::new(Arc::clone(&blank_texture))
                }
            })
            .collect();

        let show_judgement_line = keymode.judgement_line;

        let layout = KeymodeLayout {
            keymode: key_count as u8,
            receptor_above_notes: !keymode.keys_under_notes,
            show_judgement_line: show_judgement_line,
            x_offset: keymode.column_start as f32 / OsuDimensions::X.as_f32(),
            hit_position: (1.0 - (keymode.hit_position as f32 / OsuDimensions::Y.as_f32())).abs(),
            receptor_offset: max_receptor_offset as i32,
            column_widths: keymode.column_width.iter().map(|cw| *cw as f32 / OsuDimensions::X.as_f32()).collect(),
            column_spacing: keymode.column_spacing.clone(),
        };

        let texture_or_blank = |path: &str| textures.get_shared(path).unwrap_or(blank_texture.clone());

        keymodes.push(Keymode { 
            keymode: key_count as u8,
            layout,
            receptor_up: receptor_up_elements,
            receptor_down: receptor_down_elements,
            normal_note: normal_note_elements,
            long_note_head: long_note_head_elements,
            long_note_body: long_note_body_elements,
            long_note_tail: long_note_tail_elements,
            hit_lighting: HitLighting { 
                normal: texture_or_blank(&keymode.lighting_n),
                hold: texture_or_blank(&keymode.lighting_l) 
            },
            column_lighting: ColumnLighting { 
                texture: texture_or_blank(&keymode.stage_light) 
            },
            judgement_line: JudgementLine {
                texture: texture_or_blank(""),
                color: Rgba::default(),
            }
        });
    }

    let default_keymode = skin.skin_ini.keymodes[0].clone();
    let layout_keymode = skin.skin_ini.get_keymode(4).unwrap_or(&default_keymode);

    let health_bar_fg = textures.get_shared("scorebar-colour").unwrap();
    let health_bar_bg = textures.get_shared("scorebar-bg").unwrap();
    rotate_90_deg_ccw(&health_bar_fg)?;
    rotate_90_deg_ccw(&health_bar_bg)?;

    let health_bar = Healthbar::new(health_bar_fg, health_bar_bg);

    let gameplay = Gameplay {
        health_bar: health_bar,
        layout: HUDLayout {
            combo: (
                Vector3::new(
                    0.5,
                    layout_keymode.combo_position.unwrap_or_default() as f32 / OsuDimensions::Y.as_f32(),
                    1.0
                ),
                Alignment { anchor: Anchor::BottomLeft, origin: Origin::BottomLeft }
            ),
            rating: (
                Vector3::new(0.0, -30.0 / OsuDimensions::Y.as_f32(), 1.0),
                Alignment { anchor: Anchor::Centre, origin: Origin::Centre }
            ),
            accuracy: (
                Vector3::new(-50.0 / OsuDimensions::X.as_f32(), 50.0 / OsuDimensions::X.as_f32(), 1.0),
                Alignment { anchor: Anchor::TopRight,origin: Origin::Centre }
            ),
            score: (
                Vector3::new(
                    -50.0 / OsuDimensions::X.as_f32(),
                    0.0,
                    1.0
                ),
                Alignment { anchor: Anchor::TopRight, origin: Origin::TopRight }
            ),
            judgement: (
                Vector3::new(
                    0.5,
                    layout_keymode.score_position.unwrap_or_default() as f32 / OsuDimensions::Y.as_f32(),
                    1.0
                ),
                Alignment { anchor: Anchor::Centre, origin: Origin::Centre }
            ),
        }
    };
    
    Ok(GenericManiaSkin {
        resolution: skin.resolution,
        metadata,
        gameplay,
        keymodes,
        textures
    })
}

pub fn from_generic_mania(skin: GenericManiaSkin) -> Result<OsuSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures;
    let mut osu_keymodes: Vec<osu::Keymode> = Vec::new();

    let resize = Resizer::new(
        skin.resolution,
        Some(Vector2::new(OsuDimensions::X.as_u32(), OsuDimensions::Y.as_u32()))
    );

    let blank_texture: Arc<RwLock<Texture>> = textures.get_shared("blank")
        .unwrap_or(Arc::new(RwLock::new(Texture::from_blank("blank".to_string()))));

    let general = General {
        name: skin.metadata.name,
        author: skin.metadata.creator,
        version: skin.metadata.version,
        cursor_centre: skin.metadata.center_cursor,
        ..Default::default()
    };

    let mut receptor_processor = TextureProcessor::<()>::new();
    let mut tail_processor = TextureProcessor::<()>::new();

    for keymode in skin.keymodes {
        let average_column_width = keymode.layout.average_column_width();
        let receptor_offset = keymode.layout.receptor_offset;

        let receptor_images: Vec<String> = keymode.receptor_up
            .iter()
            .map(|receptor| {
                let texture_arc = &receptor.texture;
                
                if !Arc::ptr_eq(texture_arc, &blank_texture) {
                    receptor_processor.process_once_void(texture_arc, |arc_texture| {
                        if let Err(e) = to_osu_column(
                            arc_texture,
                            resize.from_target_x::<u32>(average_column_width),
                            receptor_offset.clamp(0, OsuDimensions::X.as_i32()) as u32
                        ) {
                            eprintln!("Failed to process receptor up texture: {}", e);
                        }
                    });
                }
                receptor.get_path()
            })
            .collect();

        let receptor_images_down: Vec<String> = keymode.receptor_down
            .iter()
            .map(|receptor| {
                let texture_arc = &receptor.texture;
                
                if !Arc::ptr_eq(texture_arc, &blank_texture) {
                    receptor_processor.process_once_void(texture_arc, |arc_texture| {
                        if let Err(e) = to_osu_column(
                            arc_texture,
                            resize.from_target_x::<u32>(average_column_width),
                            receptor_offset.clamp(0, OsuDimensions::X.as_i32()) as u32
                        ) {
                            eprintln!("Failed to process receptor down texture: {}", e);
                        }
                    });
                }
                receptor.get_path()
            })
            .collect();

        let normal_note_images: Vec<String> = keymode.normal_note
            .iter()
            .map(|note| note.get_path())
            .collect();

        let long_note_head_images: Vec<String> = keymode.long_note_head
            .iter()
            .map(|note| note.get_path())
            .collect();

        let long_note_body_images: Vec<String> = keymode.long_note_body
            .iter()
            .map(|note| note.get_path())
            .collect();

        let long_note_tail_images: Vec<String> = keymode.long_note_tail
            .iter()
            .map(|note| {
                let texture_arc = &note.texture;
                
                if !Arc::ptr_eq(texture_arc, &blank_texture) {
                    tail_processor.process_once_void(texture_arc, |arc_texture| {
                        flip_vertical(arc_texture);
                    });
                }
                note.get_path()
            })
            .collect();

        if let Some(health_bar_bg) = skin.gameplay.health_bar.background.get_image() {
            let texture = Arc::new(RwLock::new(Texture::with_data("scorebar-bg".to_string(), health_bar_bg)));
            rotate_90_deg_cw(&texture)?;
            textures.insert(texture.take_texture());
        }

        if let Some(health_bar_colour) = skin.gameplay.health_bar.fill.get_image() {
            let texture = Arc::new(RwLock::new(Texture::with_data("scorebar-colour".to_string(), health_bar_colour)));
            rotate_90_deg_cw(&texture)?;
            textures.insert(texture.take_texture());
        }

        // these wouldn't be present in other skins
        if !textures.contains("star") {
            textures.copy("blank", "star");
        }

        if !textures.contains("star2") {
            textures.copy("blank", "star2");
        }

        if !textures.contains("mania-stage-hint") {
            textures.copy("blank", "mania-stage-hint");
        }

        if !textures.contains("mania-warningarrow") {
            textures.copy("blank", "mania-warningarrow");
        }

        let osu_keymode = osu::Keymode {
            keymode: keymode.keymode,
            keys_under_notes: !keymode.layout.receptor_above_notes,
            hit_position: ((1.0 - keymode.layout.hit_position) * resize.target.y as f32) as u32,
            column_start: resize.to_target_x::<u32>(keymode.layout.x_offset),
            column_width: keymode.layout.column_widths
                .iter()
                .map(|cw| (resize.to_target_x::<u32>(*cw)))
                .collect(),
            column_spacing: keymode.layout.column_spacing,
            column_line_width: vec![0; keymode.keymode as usize + 1], // osu skins are the only skins that support line widths so no need to implement in generic skin
            receptor_images,
            receptor_images_down,
            normal_note_images,
            long_note_head_images,
            long_note_body_images,
            long_note_tail_images,
            lighting_n: keymode.hit_lighting.normal.get_path(),
            lighting_l: keymode.hit_lighting.hold.get_path(),
            stage_light: keymode.column_lighting.get_path(),
            judgement_line: false,
            ..Default::default()
        };

        osu_keymodes.push(osu_keymode);
    }
    
    let mut skin_ini = SkinIni {
        general,
        keymodes: osu_keymodes,
    };

    let osu_dimensions = Vector2::new(OsuDimensions::X.into(), OsuDimensions::Y.into());

    for keymode in &mut skin_ini.keymodes {

        let score_size = Vector2::new(100.0, 50.0);
        let combo_size = Vector2::new(150.0, 100.0);
        
        let score_pos = Alignment::calculate_pos(
            osu_dimensions,
            score_size,
            &skin.gameplay.layout.judgement.1
        );
        
        let combo_pos = Alignment::calculate_pos(
            osu_dimensions,
            combo_size,
            &skin.gameplay.layout.combo.1
        );
        
        keymode.score_position = Some(score_pos.y as u32);
        keymode.combo_position = Some(combo_pos.y as u32);
    }
    
    Ok(OsuSkin::new(skin_ini, Some(textures)))
}