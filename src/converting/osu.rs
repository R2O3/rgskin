use std::sync::{Arc, RwLock};
use crate::common::alignment::*;
use crate::common::color::Rgba;
use crate::common::vector::*;
use crate::extensions::{TextureArcExt, VecExtensions};
use crate::generic::{sound::*, UI};
use crate::image_proc::{generate_stage_background, to_osu_column, to_osu_column_draw};
use crate::osu::static_assets;
use crate::generic::Gameplay;
use crate::image_proc::proc::{dist_from_bottom, flip_vertical, resize_width, rotate_90_deg_ccw, rotate_90_deg_cw};
use crate::io::Store;
use crate::io::texture::{Texture, TextureProcessor};
use crate::osu::{self, General, OsuSkin, OsuSkinIni};
use crate::skin::generic::layout::{HUDLayout, KeymodeLayout};
use crate::skin::generic::{elements::*, Keymode, Metadata, GenericManiaSkin};
use crate::traits::{KeymodeInvariant, ManiaSkinConfig};
use crate::utils::osu::OsuDimensions;
use crate::utils::skin::{cleanup_stores, StoreRelocator};
use crate::{Binary, BinaryArcExt, BinaryArcExtOption, BinaryState, Resources, StringPattern};

pub fn to_generic_mania(skin: &OsuSkin) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures.clone();
    let samples = skin.samples.clone();
    let mut keymodes: Vec<Keymode> = Vec::new();

    textures.insert(Texture::from_blank("blank".to_string()));
    let blank_texture = textures.get_shared("blank").unwrap();

    let metadata = Metadata {
        name: skin.skin_ini.general.name.clone(),
        creator: skin.skin_ini.general.author.clone(),
        version: "latest".to_string(),
    };

    let mut receptor_processor = TextureProcessor::<i32>::new();
    let mut tail_processor = TextureProcessor::<()>::new();

    for keymode in &skin.skin_ini.keymodes {
        let key_count = keymode.keymode as usize;
        let average_column_width = keymode.column_width.average().unwrap_or(0.0);
        let mut max_receptor_offset = 0;

        let fallbacks = keymode.get_fallbacks();

        debug_assert!(
            fallbacks.len() == keymode.receptor_images.len()
            && fallbacks.len() == keymode.receptor_images_down.len()
            && fallbacks.len() == keymode.normal_note_images.len()
            && fallbacks.len() == keymode.long_note_head_images.len()
            && fallbacks.len() == keymode.long_note_body_images.len()
            && fallbacks.len() == keymode.long_note_tail_images.len()
            , "Length of fallbacks doesn't match actual keymode"
        );

        let receptor_up_elements: Vec<ReceptorUp> = keymode.receptor_images
            .iter()
            .zip(fallbacks.iter().map(|f| &f.receptor))
            .map(|(path, fallback_path)| {
                if !path.is_empty() {
                    if let Some(texture) = textures.get_shared(path) {
                        let offset = receptor_processor.process_once(&texture, |arc_texture| {
                            let offset = arc_texture.with_image(|img| dist_from_bottom(&img.to_rgba8(), 0.1));
                            if let Err(e) = to_osu_column_draw(arc_texture, average_column_width as u32) {
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
                    if let Some(fallback) = textures.get_shared(fallback_path) {
                        ReceptorUp::new(Some(fallback))
                    } else { 
                        ReceptorUp::new(Some(Arc::clone(&blank_texture)))
                    }
                }
            })
            .collect();

        let receptor_down_elements: Vec<ReceptorDown> = keymode.receptor_images_down
            .iter()
            .zip(fallbacks.iter().map(|f| &f.receptor_down))
            .map(|(path, fallback_path)| {
                if !path.is_empty() {
                    if let Some(texture) = textures.get_shared(path) {
                        let offset = receptor_processor.process_once(&texture, |arc_texture| {
                            let offset = arc_texture.with_image(|img| dist_from_bottom(&img.to_rgba8(), 0.1));
                            if let Err(e) = to_osu_column_draw(arc_texture, average_column_width as u32) {
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
                    if let Some(fallback) = textures.get_shared(fallback_path) {
                        ReceptorDown::new(Some(fallback))
                    } else {
                        ReceptorDown::new(Some(Arc::clone(&blank_texture)))
                    }
                }
            })
            .collect();

        let normal_note_elements: Vec<NormalNote> = keymode.normal_note_images
            .iter()
            .zip(fallbacks.iter().map(|f| &f.normal_note))
            .map(|(path, fallback_path)| {
                if !path.is_empty() {
                    if let Some(texture) = textures.get_shared(path) {
                        NormalNote::new(Some(texture))
                    } else {
                        NormalNote::new(Some(Arc::clone(&blank_texture)))
                    }
                } else {
                    if let Some(fallback) = textures.get_shared(fallback_path) {
                        NormalNote::new(Some(fallback))
                    } else {
                        NormalNote::new(Some(Arc::clone(&blank_texture)))
                    }
                }
            })
            .collect();

        let long_note_head_elements: Vec<LongNoteHead> = keymode.long_note_head_images
            .iter()
            .zip(fallbacks.iter().map(|f| &f.long_note_head))
            .map(|(path, fallback_path)| {
                if !path.is_empty() {
                    if let Some(texture) = textures.get_shared(path) {
                        LongNoteHead::new(Some(texture))
                    } else {
                        LongNoteHead::new(Some(Arc::clone(&blank_texture)))
                    }
                } else {
                    if let Some(fallback) = textures.get_shared(fallback_path) {
                        LongNoteHead::new(Some(fallback))
                    } else {
                        LongNoteHead::new(Some(Arc::clone(&blank_texture)))
                    }
                }
            })
            .collect();

        let long_note_body_elements: Vec<LongNoteBody> = keymode.long_note_body_images
            .iter()
            .zip(fallbacks.iter().map(|f| &f.long_note_body))
            .map(|(path, fallback_path)| {
                if !path.is_empty() {
                    if let Some(texture) = textures.get_shared(path) {
                        LongNoteBody::new(Some(texture))
                    } else {
                        LongNoteBody::new(Some(Arc::clone(&blank_texture)))
                    }
                } else {
                    if let Some(fallback) = textures.get_shared(fallback_path) {
                        LongNoteBody::new(Some(fallback))
                    } else {
                        LongNoteBody::new(Some(Arc::clone(&blank_texture)))
                    }
                }
            })
            .collect();

        let long_note_tail_elements: Vec<LongNoteTail> = keymode.long_note_tail_images
            .iter()
            .zip(fallbacks.iter().map(|f| &f.long_note_tail))
            .map(|(path, fallback_path)| {
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
                    if let Some(fallback) = textures.get_shared(fallback_path) {
                        tail_processor.process_once_void(&fallback, |arc_texture| {
                            flip_vertical(arc_texture);
                        });
                        LongNoteTail::new(Some(fallback))
                    } else {
                        LongNoteTail::new(Some(Arc::clone(&blank_texture)))
                    }
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

        let stage_texture = Texture::with_data("stage_bg".to_string(),
            generate_stage_background(keymode.colours.clone(), (layout.column_widths.average().unwrap_or(0.0) as f32 * OsuDimensions::X.as_f32()) as u32)
        );
        let stage_background = textures.insert(stage_texture);

        let texture_or_blank = |path: &str| textures.get_shared(path).unwrap_or(blank_texture.clone());

        let get_frames = |fallback_key: &str, prefix: StringPattern| -> Vec<Arc<RwLock<Texture>>> {
            if let Some(tex) = textures.get_shared(fallback_key) {
                vec![tex]
            } else {
                let prefix_string = prefix.to_string();
                let (_, mut all_textures): (Vec<String>, Vec<Arc<RwLock<Texture>>>) = textures
                    .get_shared_all(|t| t.get_path().starts_with(&prefix_string))
                    .into_iter()
                    .unzip();

                all_textures.sort_by_key(|arc| {
                    let path = arc.get_path();

                    path.split('-')
                        .last()
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(0)
                });

                all_textures
            }
        };

        keymodes.push(Keymode { 
            keymode: key_count as u8,
            layout,
            use_snap_color: false,
            snap_colors: Vec::new(),
            receptor_up: receptor_up_elements,
            receptor_down: receptor_down_elements,
            base_normal_note: None,
            base_long_note: None,
            base_normal_mine: None,
            normal_notes: normal_note_elements,
            long_note_heads: long_note_head_elements,
            long_note_bodies: long_note_body_elements,
            long_note_tails: long_note_tail_elements,
            normal_mines: Vec::new(),
            normal_notes_snap_colored: None,
            long_note_heads_snap_colored: None,
            normal_mines_snap_colored: None,
            hit_lighting_normal: HitLightingNormal::new(get_frames(&keymode.lighting_n, static_assets::Mania::LIGHTINGN), Some(keymode.light_frame_per_second as f32), None, None),
            hit_lighting_hold: HitLightingHold::new(get_frames(&keymode.lighting_l, static_assets::Mania::LIGHTINGL), Some(keymode.light_frame_per_second as f32), None, None),
            column_lighting: ColumnLighting { 
                texture: Some(texture_or_blank(&keymode.stage_light)) 
            },
            judgement_line: JudgementLine {
                texture: Some(texture_or_blank("")),
                color: Rgba::default(),
            },
            stage: Stage::new(
                Some(stage_background),
                textures.get_shared(&keymode.stage_right)
                    .or(textures.get_shared(&static_assets::Mania::STAGE_RIGHT)),
                textures.get_shared(&keymode.stage_left)
                    .or(textures.get_shared(&static_assets::Mania::STAGE_LEFT)),
            ),
            fallbacks,
        });
    }

    let default_keymode_fallback = osu::Keymode::default();
    let default_keymode = skin.skin_ini.get_keymode(4).unwrap_or(skin.skin_ini.keymodes.first().unwrap_or(&default_keymode_fallback));

    let health_bar_fg = textures.get_shared(&static_assets::Interface::SCOREBAR_COLOUR).unwrap_or(blank_texture.clone());
    let health_bar_bg = textures.get_shared(&static_assets::Interface::SCOREBAR_BG).unwrap_or(blank_texture.clone());
    rotate_90_deg_ccw(&health_bar_fg)?;
    rotate_90_deg_ccw(&health_bar_bg)?;

    let gameplay = Gameplay {
        health_bar: Healthbar::new(Some(health_bar_fg), Some(health_bar_bg)),
        judgement: Judgement::new(
            textures.get_shared(&default_keymode.hit300g)
                .or(textures.get_shared(&static_assets::Mania::HIT300G)),
            textures.get_shared(&default_keymode.hit300)
                .or(textures.get_shared(&static_assets::Mania::HIT300)),
            textures.get_shared(&default_keymode.hit200)
                .or(textures.get_shared(&static_assets::Mania::HIT200)),
            textures.get_shared(&default_keymode.hit100)
                .or(textures.get_shared(&static_assets::Mania::HIT100)),
            textures.get_shared(&default_keymode.hit50)
                .or(textures.get_shared(&static_assets::Mania::HIT50)),
            textures.get_shared(&default_keymode.hit0)
                .or(textures.get_shared(&static_assets::Mania::HIT0)),
        ),
        layout: HUDLayout {
            combo: (
                Vector3::new(
                    0.5,
                    default_keymode.combo_position.unwrap_or_default() as f32 / OsuDimensions::Y.as_f32(),
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
                    default_keymode.score_position.unwrap_or_default() as f32 / OsuDimensions::Y.as_f32(),
                    1.0
                ),
                Alignment { anchor: Anchor::Centre, origin: Origin::Centre }
            ),
        }
    };
    
    let ui = UI {
        cursor: Cursor {
            texture: textures.get_shared(&static_assets::Interface::CURSOR)
            .or_else(|| {
                let bytes = Resources::cursor("lazer_cursor.png") // osu!stable cursor sucks
                    .expect("Failed to load cursor texture");
                let tex = Texture::from_bytes(
                    static_assets::Interface::CURSOR.to_string(),
                    &bytes
                ).ok()?;
                Some(Arc::new(RwLock::new(tex)))
            }),
            centered: skin.skin_ini.general.cursor_centre.clone(),
            rotate: skin.skin_ini.general.cursor_rotate.clone()
        }
    };

    let sounds = Sounds {
        ui: UISounds {
            menu_back_click: samples.get_shared(&static_assets::Samples::MENU_BACK_CLICK).get_path(),
            ui_click: samples.get_shared(&static_assets::Samples::CLICK_SHORT_CONFIRM).get_path(),
            ui_select: samples.get_shared(&static_assets::Samples::MENU_FREEPLAY_CLICK).get_path(),
            ui_hover: samples.get_shared(&static_assets::Samples::CLICK_SHORT).get_path()
        },
        gameplay: GenericGameplaySounds {
            miss: samples.get_shared(&static_assets::Samples::COMBOBREAK).get_path(),
            fail: samples.get_shared(&static_assets::Samples::FAILSOUND).get_path(),
            restart: samples.get_shared(&static_assets::Samples::PAUSE_RETRY_CLICK).get_path()
        },
        mania: ManiaGameplaySounds {
            hit: samples.get_shared(&static_assets::Samples::DRUM_HITNORMAL).get_path()
        },
    };

    let mut generic_skin = GenericManiaSkin {
        resolution: skin.resolution,
        sounds,
        metadata,
        ui,
        gameplay,
        keymodes,
        textures,
        samples
    };

    generic_skin.ensure_textures();
    
    Ok(generic_skin)
}

pub fn from_generic_mania(skin: &GenericManiaSkin) -> Result<OsuSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures.clone();
    let mut samples = skin.samples.clone();
    let mut osu_keymodes: Vec<osu::Keymode> = Vec::new();

    let blank_texture: Arc<RwLock<Texture>> = textures.get_shared("blank")
        .unwrap_or(Arc::new(RwLock::new(Texture::from_blank("blank".to_string()))));

    let general = General {
        name: skin.metadata.name.clone(),
        author: skin.metadata.creator.clone(),
        version: "latest".to_string(),
        cursor_centre: skin.ui.cursor.centered,
        cursor_rotate: skin.ui.cursor.rotate,
        cursor_expand: true, // TODO: change this when adding quaver later
        ..Default::default()
    };

    let mut receptor_processor = TextureProcessor::<()>::new();
    let mut tail_processor = TextureProcessor::<()>::new();

    for keymode in &skin.keymodes {
        let key_count = keymode.keymode as u8;
        let average_column_width = keymode.layout.column_widths.average().unwrap_or(0.0);
        let receptor_offset = keymode.layout.receptor_offset;
        let use_snap_color = keymode.use_snap_color;

        let base_note_images: Vec<Option<String>> = vec![
            keymode.base_normal_note.as_ref().and_then(|n| n.get_path());
            key_count as usize
        ];
        let base_long_head_images: Vec<Option<String>> = vec![
            keymode.base_long_note.as_ref().and_then(|n| n.get_path());
            key_count as usize
        ];

        let receptor_images: Vec<String> = keymode.receptor_up
            .iter()
            .map(|receptor| {
                if let Some(texture_arc) = &receptor.texture {
                    if !Arc::ptr_eq(texture_arc, &blank_texture) {
                        receptor_processor.process_once_void(texture_arc, |arc_texture| {
                            if let Err(e) = to_osu_column(
                                arc_texture,
                                (average_column_width * OsuDimensions::X.as_f32()) as u32,
                                receptor_offset.clamp(0, OsuDimensions::Y.as_i32()) as u32
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
                                (average_column_width * OsuDimensions::X.as_f32()) as u32,
                                receptor_offset.clamp(0, OsuDimensions::Y.as_i32()) as u32
                            ) {
                                eprintln!("Failed to process receptor down texture: {}", e);
                            }
                        });
                    }
                }
                receptor.get_path().unwrap_or_default()
            })
            .collect();

        let normal_note_images: Vec<String> = {
            let per_key = keymode.normal_notes
                .iter()
                .map(|n| n.get_path().unwrap_or_default())
                .collect();

            if use_snap_color && base_note_images.first().is_some_and(|p| p.is_some()) {
                base_note_images.iter().map(|p| p.clone().unwrap_or_default()).collect()
            } else {
                per_key
            }
        };

        let long_note_head_images: Vec<String> = {
            let per_key: Vec<String> = keymode.long_note_heads
                .iter()
                .enumerate()
                .map(|(i, n)| {
                    let path = n.get_path().unwrap_or_default();
                    
                    if path.is_empty() || path == "blank" {
                        keymode.normal_notes.get(i).and_then(|nn| nn.get_path()).unwrap_or_default()
                    } else {
                        path
                    }
                })
                .collect();

            if use_snap_color && base_long_head_images.first().is_some_and(|p| p.is_some()) {
                base_long_head_images.iter().map(|p| p.clone().unwrap_or_default()).collect()
            
            } else if use_snap_color && base_note_images.first().is_some_and(|p| p.is_some()) {
                base_note_images.iter().map(|p| p.clone().unwrap_or_default()).collect()
            } else {
                per_key
            }
        };

        let long_note_body_images: Vec<String> = keymode.long_note_bodies
            .iter()
            .map(|note| note.get_path().unwrap_or_default())
            .collect();

        let long_note_tail_images: Vec<String> = keymode.long_note_tails
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

        let mut tr = StoreRelocator::new(&mut textures);
        let mut sr = StoreRelocator::new(&mut samples);

        tr.reloc_arc_lock(&skin.gameplay.judgement.flawless, static_assets::Mania::HIT300G);
        tr.reloc_arc_lock(&skin.gameplay.judgement.perfect, static_assets::Mania::HIT300);
        tr.reloc_arc_lock(&skin.gameplay.judgement.great, static_assets::Mania::HIT200);
        tr.reloc_arc_lock(&skin.gameplay.judgement.good, static_assets::Mania::HIT100);
        tr.reloc_arc_lock(&skin.gameplay.judgement.bad, static_assets::Mania::HIT50);
        tr.reloc_arc_lock(&skin.gameplay.judgement.miss, static_assets::Mania::HIT0);

        // these wouldn't be present in other skins
        tr.reloc_str_if_not_exist("blank", static_assets::Interface::STAR);
        tr.reloc_str_if_not_exist("blank", static_assets::Interface::STAR2);
        tr.reloc_str_if_not_exist("blank", static_assets::Interface::SCOREBAR_MARKER);
        tr.reloc_str_if_not_exist("blank", static_assets::Mania::STAGE_HINT);
        tr.reloc_str_if_not_exist("blank", static_assets::Mania::WARNINGARROW);
        tr.reloc_str_if_not_exist("blank", static_assets::Interface::CURSORMIDDLE);
        tr.reloc_str_if_not_exist("blank", static_assets::Interface::CURSORTRAIL);

        if !textures.contains(&static_assets::Interface::CURSOR) {
            if let Some(cursor_arc) = &skin.ui.cursor.texture {
                resize_width(cursor_arc, 24, image::imageops::FilterType::Triangle)?;
            }

            if let Some(cursor_image) = skin.ui.cursor.texture.clone_data() {
                textures.copy_from_data(
                    &static_assets::Interface::CURSOR,
                    BinaryState::Loaded(cursor_image)
                );
            }
        }

        // Samples
        sr.reloc_str(&skin.sounds.ui.menu_back_click, static_assets::Samples::MENU_BACK_CLICK);
        sr.reloc_str(&skin.sounds.ui.ui_click, static_assets::Samples::CLICK_SHORT_CONFIRM);
        sr.reloc_str(&skin.sounds.ui.ui_select, static_assets::Samples::MENU_FREEPLAY_CLICK);
        sr.reloc_str(&skin.sounds.ui.ui_hover, static_assets::Samples::CLICK_SHORT);
        sr.reloc_str(&skin.sounds.gameplay.miss, static_assets::Samples::COMBOBREAK);
        sr.reloc_str(&skin.sounds.gameplay.fail, static_assets::Samples::FAILSOUND);
        sr.reloc_str(&skin.sounds.gameplay.restart, static_assets::Samples::PAUSE_RETRY_CLICK);
        sr.reloc_str(&skin.sounds.mania.hit, static_assets::Samples::DRUM_HITNORMAL);

        // we'll assume that the size of the screen is 16:9 since that's most common
        // osu!mania playfield positions depends on your screen ratio
        let reference_size = Vector2::new(1920f32, 1080f32);
        let aspect_ratio = reference_size.x / reference_size.y;
        let stage_width = (keymode.layout.column_widths.iter().sum::<f32>() + (keymode.layout.column_spacing.iter().sum::<f32>())) * OsuDimensions::X.as_f32() * OsuDimensions::ColumnScaleFromGeneric.as_f32();
        let playfield_pos = (OsuDimensions::Y.as_f32() * aspect_ratio - stage_width.round()) * keymode.layout.x_offset;

        // TODO: add animated assets to osu

        let osu_keymode = osu::Keymode {
            keymode: key_count,
            keys_under_notes: !keymode.layout.receptor_above_notes,
            hit_position: ((1.0 - keymode.layout.hit_position) * OsuDimensions::Y.as_f32()) as u32,
            column_start: playfield_pos,
            column_width: keymode.layout.column_widths
                .iter()
                .map(|cw| *cw * OsuDimensions::X.as_f32())
                .collect(),
            column_spacing: keymode.layout.column_spacing.clone(),
            column_line_width: vec![0.0; key_count as usize + 1], // osu skins are the only skins that support line widths so no need to implement in generic skin
            receptor_images,
            receptor_images_down,
            normal_note_images,
            long_note_head_images,
            long_note_body_images,
            long_note_tail_images,
            lighting_n: keymode.hit_lighting_normal.get_path().unwrap_or_default(),
            lighting_l: keymode.hit_lighting_hold.get_path().unwrap_or_default(),
            stage_light: keymode.column_lighting.texture.get_path().unwrap_or_default(),
            stage_right: keymode.stage.border_right.get_path().unwrap_or_default(),
            stage_left: keymode.stage.border_left.get_path().unwrap_or_default(),
            judgement_line: keymode.layout.show_judgement_line,
            hit0: skin.gameplay.judgement.miss.get_path().unwrap_or_default(),
            hit50: skin.gameplay.judgement.bad.get_path().unwrap_or_default(),
            hit100: skin.gameplay.judgement.good.get_path().unwrap_or_default(),
            hit200: skin.gameplay.judgement.great.get_path().unwrap_or_default(),
            hit300: skin.gameplay.judgement.perfect.get_path().unwrap_or_default(),
            hit300g: skin.gameplay.judgement.flawless.get_path().unwrap_or_default(),
            ..Default::default()
        };

        osu_keymodes.push(osu_keymode);
    }
    
    let mut skin_ini = OsuSkinIni {
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