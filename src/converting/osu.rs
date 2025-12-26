use std::sync::{Arc, RwLock};
use crate::common::alignment::*;
use crate::common::color::Rgba;
use crate::common::vector::*;
use crate::extensions::TextureArcExt;
use crate::generic::sound::*;
use crate::osu::static_assets;
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
use crate::utils::skin::cleanup_stores;
use crate::BinaryArcExt;

pub fn to_generic_mania(skin: &OsuSkin) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures.clone();
    let samples = skin.samples.clone();
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
                        ReceptorUp::new(Some(texture))
                    } else {
                        ReceptorUp::new(Some(Arc::clone(&blank_texture)))
                    }
                } else {
                    ReceptorUp::new(Some(Arc::clone(&blank_texture)))
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
                        ReceptorDown::new(Some(texture))
                    } else {
                        ReceptorDown::new(Some(Arc::clone(&blank_texture)))
                    }
                } else {
                    ReceptorDown::new(Some(Arc::clone(&blank_texture)))
                }
            })
            .collect();

        let normal_note_elements: Vec<NormalNote> = keymode.normal_note_images
            .iter()
            .map(|path| {
                if !path.is_empty() && textures.contains(path) {
                    NormalNote::new(textures.get_shared(path))
                } else {
                    NormalNote::new(Some(Arc::clone(&blank_texture)))
                }
            })
            .collect();

        let long_note_head_elements: Vec<LongNoteHead> = keymode.long_note_head_images
            .iter()
            .map(|path| {
                if !path.is_empty() && textures.contains(path) {
                    LongNoteHead::new(textures.get_shared(path))
                } else {
                    LongNoteHead::new(Some(Arc::clone(&blank_texture)))
                }
            })
            .collect();

        let long_note_body_elements: Vec<LongNoteBody> = keymode.long_note_body_images
            .iter()
            .map(|path| {
                if !path.is_empty() && textures.contains(path) {
                    LongNoteBody::new(textures.get_shared(path))
                } else {
                    LongNoteBody::new(Some(Arc::clone(&blank_texture)))
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
                        LongNoteTail::new(Some(texture))
                    } else {
                        LongNoteTail::new(Some(Arc::clone(&blank_texture)))
                    }
                } else {
                    LongNoteTail::new(Some(Arc::clone(&blank_texture)))
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
                normal: Some(texture_or_blank(&keymode.lighting_n)),
                hold: Some(texture_or_blank(&keymode.lighting_l)) 
            },
            column_lighting: ColumnLighting { 
                texture: Some(texture_or_blank(&keymode.stage_light)) 
            },
            judgement_line: JudgementLine {
                texture: Some(texture_or_blank("")),
                color: Rgba::default(),
            }
        });
    }

    let default_keymode = skin.skin_ini.keymodes[0].clone();
    let layout_keymode = skin.skin_ini.get_keymode(4).unwrap_or(&default_keymode);

    let health_bar_fg = textures.get_shared(static_assets::Interface::SCOREBAR_COLOUR).unwrap();
    let health_bar_bg = textures.get_shared(static_assets::Interface::SCOREBAR_BG).unwrap();
    rotate_90_deg_ccw(&health_bar_fg)?;
    rotate_90_deg_ccw(&health_bar_bg)?;

    let gameplay = Gameplay {
        health_bar: Healthbar::new(Some(health_bar_fg), Some(health_bar_bg)),
        stage: Stage::new(
            Some(blank_texture.clone()), // TODO: properly implement stage background for osu
            textures.get_shared(&default_keymode.stage_right)
                .or(textures.get_shared(static_assets::Mania::STAGE_RIGHT)),
            textures.get_shared(&default_keymode.stage_left)
                .or(textures.get_shared(static_assets::Mania::STAGE_LEFT)),
        ),
        judgement: Judgement::new(
            textures.get_shared(&default_keymode.hit300g)
                .or(textures.get_shared(static_assets::Mania::HIT300G)),
            textures.get_shared(&default_keymode.hit300)
                .or(textures.get_shared(static_assets::Mania::HIT300)),
            textures.get_shared(&default_keymode.hit200)
                .or(textures.get_shared(static_assets::Mania::HIT200)),
            textures.get_shared(&default_keymode.hit100)
                .or(textures.get_shared(static_assets::Mania::HIT100)),
            textures.get_shared(&default_keymode.hit50)
                .or(textures.get_shared(static_assets::Mania::HIT50)),
            textures.get_shared(&default_keymode.hit0)
                .or(textures.get_shared(static_assets::Mania::HIT0)),
        ),
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

    let sounds = Sounds {
        ui: UISounds {
            menu_back_click: samples.get_shared(static_assets::Samples::MENU_BACK_CLICK).as_ref().map(|a| a.get_path()),
            ui_click: samples.get_shared(static_assets::Samples::CLICK_SHORT_CONFIRM).as_ref().map(|a| a.get_path()),
            ui_select: samples.get_shared(static_assets::Samples::MENU_FREEPLAY_CLICK).as_ref().map(|a| a.get_path()),
            ui_hover: samples.get_shared(static_assets::Samples::CLICK_SHORT).as_ref().map(|a| a.get_path())
        },
        gameplay: GenericGameplaySounds {
            miss: samples.get_shared(static_assets::Samples::COMBOBREAK).as_ref().map(|a| a.get_path()),
            fail: samples.get_shared(static_assets::Samples::FAILSOUND).as_ref().map(|a| a.get_path()),
            restart: samples.get_shared(static_assets::Samples::PAUSE_RETRY_CLICK).as_ref().map(|a| a.get_path())
        },
        mania: ManiaGameplaySounds {
            hit: samples.get_shared(static_assets::Samples::DRUM_HITNORMAL).as_ref().map(|a| a.get_path())
        },
    };
    
    Ok(GenericManiaSkin {
        resolution: skin.resolution,
        sounds,
        metadata,
        gameplay,
        keymodes,
        textures,
        samples
    })
}

pub fn from_generic_mania(skin: &GenericManiaSkin) -> Result<OsuSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures.clone();
    let mut samples = skin.samples.clone();
    let mut osu_keymodes: Vec<osu::Keymode> = Vec::new();

    let resize = Resizer::new(
        skin.resolution,
        Some(Vector2::new(OsuDimensions::X.as_u32(), OsuDimensions::Y.as_u32()))
    );

    let blank_texture: Arc<RwLock<Texture>> = textures.get_shared("blank")
        .unwrap_or(Arc::new(RwLock::new(Texture::from_blank("blank".to_string()))));

    let general = General {
        name: skin.metadata.name.clone(),
        author: skin.metadata.creator.clone(),
        version: skin.metadata.version.clone(),
        cursor_centre: skin.metadata.center_cursor,
        ..Default::default()
    };

    let mut receptor_processor = TextureProcessor::<()>::new();
    let mut tail_processor = TextureProcessor::<()>::new();

    for keymode in &skin.keymodes {
        let average_column_width = keymode.layout.average_column_width();
        let receptor_offset = keymode.layout.receptor_offset;

        let receptor_images: Vec<String> = keymode.receptor_up
            .iter()
            .map(|receptor| {
                if let Some(texture_arc) = &receptor.texture {
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
                }
                receptor.get_path().unwrap_or_default()
            })
            .collect();

        let receptor_images_down: Vec<String> = keymode.receptor_down
            .iter()
            .map(|receptor| {
                if let Some(texture_arc) = &receptor.texture {
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
                }
                receptor.get_path().unwrap_or_default()
            })
            .collect();

        let normal_note_images: Vec<String> = keymode.normal_note
            .iter()
            .map(|note| note.get_path().unwrap_or_default())
            .collect();

        let long_note_head_images: Vec<String> = keymode.long_note_head
            .iter()
            .map(|note| note.get_path().unwrap_or_default())
            .collect();

        let long_note_body_images: Vec<String> = keymode.long_note_body
            .iter()
            .map(|note| note.get_path().unwrap_or_default())
            .collect();

        let long_note_tail_images: Vec<String> = keymode.long_note_tail
            .iter()
            .map(|note| {
                if let Some(texture_arc) = &note.texture {
                    if !Arc::ptr_eq(texture_arc, &blank_texture) {
                        tail_processor.process_once_void(texture_arc, |arc_texture| {
                            flip_vertical(arc_texture);
                        });
                    }
                }
                note.get_path().unwrap_or_default()
            })
            .collect();

        if let Some(bg_arc) = &skin.gameplay.health_bar.background {
            if let Some(health_bar_bg) = bg_arc.get_image() {
                let texture = Arc::new(RwLock::new(
                    Texture::with_data(static_assets::Interface::SCOREBAR_BG.to_string(),
                    health_bar_bg
                )));
                rotate_90_deg_cw(&texture)?;
                textures.insert(texture.take_texture());
            }
        }

        if let Some(fill_arc) = &skin.gameplay.health_bar.fill {
            if let Some(health_bar_colour) = fill_arc.get_image() {
                let texture = Arc::new(RwLock::new(
                    Texture::with_data(static_assets::Interface::SCOREBAR_COLOUR.to_string(),
                    health_bar_colour
                )));
                rotate_90_deg_cw(&texture)?;
                textures.insert(texture.take_texture());
            }
        }

        // these wouldn't be present in other skins
        if !textures.contains(static_assets::Interface::STAR) {
            textures.copy("blank", static_assets::Interface::STAR);
        }

        if !textures.contains(static_assets::Interface::STAR2) {
            textures.copy("blank", static_assets::Interface::STAR2);
        }

        if !textures.contains(static_assets::Interface::SCOREBAR_MARKER) {
            textures.copy("blank", static_assets::Interface::SCOREBAR_MARKER);
        }

        if !textures.contains(static_assets::Mania::STAGE_HINT) {
            textures.copy("blank", static_assets::Mania::STAGE_HINT);
        }

        if !textures.contains(static_assets::Mania::WARNINGARROW) {
            textures.copy("blank", static_assets::Mania::WARNINGARROW);
        }

        // Samples

        if let Some(s) = &skin.sounds.ui.menu_back_click {
            samples.copy(s, static_assets::Samples::MENU_BACK_CLICK);
        }

        if let Some(s) = &skin.sounds.ui.ui_click {
            samples.copy(s, static_assets::Samples::CLICK_SHORT_CONFIRM);
        }

        if let Some(s) = &skin.sounds.ui.ui_select {
            samples.copy(s, static_assets::Samples::MENU_FREEPLAY_CLICK);
        }

        if let Some(s) = &skin.sounds.ui.ui_hover {
            samples.copy(s, static_assets::Samples::CLICK_SHORT);
        }

        if let Some(s) = &skin.sounds.gameplay.miss {
            samples.copy(s, static_assets::Samples::COMBOBREAK);
        }

        if let Some(s) = &skin.sounds.gameplay.fail {
            samples.copy(s, static_assets::Samples::FAILSOUND);
        }

        if let Some(s) = &skin.sounds.gameplay.restart {
            samples.copy(s, static_assets::Samples::PAUSE_RETRY_CLICK);
        }

        if let Some(s) = &skin.sounds.mania.hit {
            samples.copy(s, static_assets::Samples::DRUM_HITNORMAL);
        }

        let source_aspect_ratio = resize.source.y as f32 / resize.source.x as f32;
        let playfield_pos = (OsuDimensions::Y.as_f32() / source_aspect_ratio) * keymode.layout.x_offset;
        let column_width = resize.to_target_x::<f32>(keymode.layout.average_column_width());
        let playfield_width = column_width * keymode.keymode as f32;

        let osu_keymode = osu::Keymode {
            keymode: keymode.keymode,
            keys_under_notes: !keymode.layout.receptor_above_notes,
            hit_position: ((1.0 - keymode.layout.hit_position) * resize.target.y as f32) as u32,
            column_start: (playfield_pos - playfield_width / 2.0) as u32,
            column_width: keymode.layout.column_widths
                .iter()
                .map(|cw| (resize.to_target_x::<u32>(*cw)))
                .collect(),
            column_spacing: keymode.layout.column_spacing.clone(),
            column_line_width: vec![0; keymode.keymode as usize + 1], // osu skins are the only skins that support line widths so no need to implement in generic skin
            receptor_images,
            receptor_images_down,
            normal_note_images,
            long_note_head_images,
            long_note_body_images,
            long_note_tail_images,
            lighting_n: keymode.hit_lighting.normal.as_ref().map(|a| a.get_path()).unwrap_or_default(),
            lighting_l: keymode.hit_lighting.hold.as_ref().map(|a| a.get_path()).unwrap_or_default(),
            stage_light: keymode.column_lighting.texture.as_ref().map(|a| a.get_path()).unwrap_or_default(),
            judgement_line: keymode.layout.show_judgement_line,
            hit0: skin.gameplay.judgement.miss.as_ref().map(|a| a.get_path()).unwrap_or_default(),
            hit50: skin.gameplay.judgement.bad.as_ref().map(|a| a.get_path()).unwrap_or_default(),
            hit100: skin.gameplay.judgement.good.as_ref().map(|a| a.get_path()).unwrap_or_default(),
            hit200: skin.gameplay.judgement.great.as_ref().map(|a| a.get_path()).unwrap_or_default(),
            hit300: skin.gameplay.judgement.perfect.as_ref().map(|a| a.get_path()).unwrap_or_default(),
            hit300g: skin.gameplay.judgement.flawless.as_ref().map(|a| a.get_path()).unwrap_or_default(),
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

    cleanup_stores(&skin_ini, Some(&mut textures), Some(&mut samples));
    
    Ok(OsuSkin::new(skin_ini, Some(textures), Some(samples)))
}