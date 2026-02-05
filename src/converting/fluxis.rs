use std::sync::{Arc, RwLock};
use image::GenericImageView;

use crate::common::alignment::*;
use crate::common::color::Rgba;
use crate::common::vector::*;
use crate::extensions::TextureArcExt;
use crate::fluxis::static_assets;
use crate::generic::{sound::*, Gameplay, Keymode, Metadata, UI};
use crate::generic::layout::{HUDLayout, KeymodeLayout};
use crate::generic::elements::{*, self};
use crate::image_proc::generate_fluxis_preview;
use crate::image_proc::proc::dist_from_bottom;
use crate::io::Store;
use crate::io::texture::{Texture, TextureProcessor};
use crate::skin::fluxis::layout_json::component::*;
use crate::skin::fluxis::layout_json::gameplay::*;
use crate::skin::fluxis::skin_json::colors::{JudgementColors, SnapColors};
use crate::skin::fluxis::skin_json::info::Info;
use crate::skin::fluxis::skin_json;
use crate::skin::fluxis::skin_json::overrides::Overrides;
use crate::skin::fluxis::{
    skin::FluXisSkin,
    FluXisLayout,
    SkinJson
};
use crate::traits::KeymodeInvariant;
use crate::utils::fluxis::FluXisDimensions;
use crate::utils::math::Resizer;
use crate::utils::skin::cleanup_stores;
use crate::{Binary, BinaryArcExt, BinaryArcExtOption, GenericManiaSkin, Resources};

pub fn to_generic_mania(skin: &FluXisSkin, layout: Option<&FluXisLayout>) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures.clone();
    let samples = skin.samples.clone();
    let layout_d = FluXisLayout::default();
    let layout = layout.unwrap_or(&layout_d);
    let mut keymodes: Vec<Keymode> = Vec::new();

    textures.insert(Texture::from_blank("blank".to_string()));
    let blank_texture = textures.get_shared("blank").unwrap();

    let metadata = Metadata {
        name: skin.skin_json.info.name.clone(),
        creator: skin.skin_json.info.creator.clone(),
        ..Default::default()
    };

    let mut receptor_processor = TextureProcessor::<i32>::new();

    for keymode in &skin.skin_json.keymodes {
        let key_count = keymode.keymode as usize;
        let mut max_additional_offset = 0;

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
                        let offset = receptor_processor.process_once(&texture, |arc| {
                            arc.with_image(|img| dist_from_bottom(img, 0.1)) as i32
                        });
                        max_additional_offset = max_additional_offset.max(offset);
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
                        let offset = receptor_processor.process_once(&texture, |tex| {
                            tex.with_image(|img| dist_from_bottom(img, 0.1)) as i32
                        });
                        max_additional_offset = max_additional_offset.max(offset);
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

        let fallback_to_normal = keymode.long_note_head_images.iter().all(|path| path.is_empty());

        let long_note_head_elements: Vec<LongNoteHead> = if fallback_to_normal {
            keymode.normal_note_images
                .iter()
                .zip(fallbacks.iter().map(|f| &f.normal_note))
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
                .collect()
        } else {
            keymode.long_note_head_images
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
                .collect()
        };

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
                        LongNoteTail::new(Some(texture))
                    } else {
                        LongNoteTail::new(Some(Arc::clone(&blank_texture)))
                    }
                } else {
                    if let Some(fallback) = textures.get_shared(fallback_path) {
                        LongNoteTail::new(Some(fallback))
                    } else {
                        LongNoteTail::new(Some(Arc::clone(&blank_texture)))
                    }
                }
            })
            .collect();

        let show_judgement_line = !skin.skin_json.overrides.stage.hitline.trim().is_empty();

        let new_layout = KeymodeLayout {
            keymode: key_count as u8,
            receptor_above_notes: !keymode.receptors_first,
            show_judgement_line: show_judgement_line,
            x_offset: 0.5,
            hit_position: (keymode.hit_position) as f32 / FluXisDimensions::Y.as_f32(),
            receptor_offset: keymode.receptor_offset + max_additional_offset,
            column_widths: vec![keymode.column_width as f32 / FluXisDimensions::X.as_f32(); key_count],
            column_spacing: vec![0.0; key_count],
        };

        let column_lighting_path = &skin.skin_json.overrides.lighting.column_lighting;
        let texture_or_blank = |path: &str| textures.get_shared(path).unwrap_or(blank_texture.clone());
        keymodes.push(Keymode { 
            keymode: key_count as u8,
            layout: new_layout,
            receptor_up: receptor_up_elements,
            receptor_down: receptor_down_elements,
            normal_note: normal_note_elements,
            long_note_head: long_note_head_elements,
            long_note_body: long_note_body_elements,
            long_note_tail: long_note_tail_elements,
            hit_lighting: HitLighting { normal: Some(blank_texture.clone()),
                hold: Some(blank_texture.clone()) },
            column_lighting: ColumnLighting { texture: Some(texture_or_blank(column_lighting_path)) },
            judgement_line: JudgementLine {
                texture: if !show_judgement_line {
                    Some(blank_texture.clone())
                } else {
                    Some(texture_or_blank(&skin.skin_json.overrides.stage.hitline))
                },
                color: Rgba::default(),
            },
            fallbacks
        });
    }

    let fluxis_x = FluXisDimensions::X.as_f32();
    let fluxis_y = FluXisDimensions::Y.as_f32();

    let combo_hud = layout.gameplay.components.get("Combo").unwrap();
    let rating_hud = layout.gameplay.components.get("PerformanceRating").unwrap();
    let accuracy_hud = layout.gameplay.components.get("Accuracy").unwrap();
    let judgement_hud = layout.gameplay.components.get("Judgement").unwrap();

    let gameplay = Gameplay {
        health_bar: Healthbar::new(
            textures.get_shared(&skin.skin_json.overrides.health.foreground)
                .or(textures.get_shared(static_assets::Health::FOREGROUND)),
            textures.get_shared(&skin.skin_json.overrides.health.background)
                .or(textures.get_shared(static_assets::Health::BACKGROUND))
        ),
        stage: Stage::new(
            textures.get_shared(&skin.skin_json.overrides.stage.background)
                .or(textures.get_shared(static_assets::Stage::BACKGROUND)),
            textures.get_shared(&skin.skin_json.overrides.stage.border_right)
                .or(textures.get_shared(static_assets::Stage::BORDER_RIGHT)),
            textures.get_shared(&skin.skin_json.overrides.stage.border_left)
                .or(textures.get_shared(static_assets::Stage::BORDER_LEFT)),
        ),
        judgement: elements::Judgement::new(
            textures.get_shared(&skin.skin_json.overrides.judgement.flawless)
                .or(textures.get_shared(static_assets::Judgement::FLAWLESS)),
            textures.get_shared(&skin.skin_json.overrides.judgement.perfect)
                .or(textures.get_shared(static_assets::Judgement::PERFECT)),
            textures.get_shared(&skin.skin_json.overrides.judgement.great)
                .or(textures.get_shared(static_assets::Judgement::GREAT)),
            textures.get_shared(&skin.skin_json.overrides.judgement.alright)
                .or(textures.get_shared(static_assets::Judgement::ALRIGHT)),
            textures.get_shared(&skin.skin_json.overrides.judgement.okay)
                .or(textures.get_shared(static_assets::Judgement::OKAY)),
            textures.get_shared(&skin.skin_json.overrides.judgement.miss)
                .or(textures.get_shared(static_assets::Judgement::MISS)),
        ),
        layout: HUDLayout {
            combo: (
                Vector3::new(
                    combo_hud.position.x / fluxis_x,
                    combo_hud.position.y / fluxis_y,
                    combo_hud.scale
                ),
                Alignment { 
                    anchor: Anchor::from_u8(combo_hud.anchor).unwrap_or_default(), 
                    origin: Origin::from_u8(combo_hud.origin).unwrap_or_default() 
                }
            ),
            rating: (
                Vector3::new(
                    rating_hud.position.x / fluxis_x,
                    rating_hud.position.y / fluxis_y,
                    rating_hud.scale
                ),
                Alignment { 
                    anchor: Anchor::from_u8(rating_hud.anchor).unwrap_or_default(), 
                    origin: Origin::from_u8(rating_hud.origin).unwrap_or_default() 
                }
            ),
            accuracy: (
                Vector3::new(
                    accuracy_hud.position.x / fluxis_x,
                    accuracy_hud.position.y / fluxis_y,
                    accuracy_hud.scale
                ),
                Alignment { 
                    anchor: Anchor::from_u8(accuracy_hud.anchor).unwrap_or_default(), 
                    origin: Origin::from_u8(accuracy_hud.origin).unwrap_or_default() 
                }
            ),
            score: (
                Vector3::new(
                    -187.5 / fluxis_x,
                    0.0,
                    1.0
                ),
                Alignment { anchor: Anchor::TopRight, origin: Origin::TopRight }
            ),
            judgement: (
                Vector3::new(
                    judgement_hud.position.x / fluxis_x,
                    judgement_hud.position.y / fluxis_y,
                    judgement_hud.scale
                ),
                Alignment { 
                    anchor: Anchor::from_u8(judgement_hud.anchor).unwrap_or_default(), 
                    origin: Origin::from_u8(judgement_hud.origin).unwrap_or_default() 
                }
            ),
        }
    };

    let ui = UI {
        cursor: Cursor {
            texture: Some(Arc::new(RwLock::new(
                Texture::from_bytes(
                    "Cursor/fluxis_cursor".to_string(),
                    &Resources::cursor("fluxis_cursor.png")
                        .expect("Cursor resource not found")
                ).expect("Failed to load cursor texture")
            ))),
            centered: false,
        }
    };

    let sounds = Sounds {
        ui: UISounds {
            menu_back_click: samples.get_shared(static_assets::Samples::UI_BACK).get_path(),
            ui_click: samples.get_shared(static_assets::Samples::UI_CLICK).get_path(),
            ui_select: samples.get_shared(static_assets::Samples::UI_SELECT).get_path(),
            ui_hover: samples.get_shared(static_assets::Samples::UI_HOVER).get_path()
        },
        gameplay: GenericGameplaySounds {
            miss: samples.get_shared(static_assets::Samples::GAMEPLAY_MISS).get_path(),
            fail: samples.get_shared(static_assets::Samples::GAMEPLAY_FAIL).get_path(),
            restart: samples.get_shared(static_assets::Samples::GAMEPLAY_RESTART).get_path()
        },
        mania: ManiaGameplaySounds {
            hit: samples.get_shared(static_assets::Samples::GAMEPLAY_HIT).get_path()
        },
    };

    Ok(GenericManiaSkin {
        resolution: skin.resolution,
        sounds,
        metadata,
        ui,
        gameplay,
        keymodes,
        textures,
        samples
    })
}

pub fn from_generic_mania(skin: &GenericManiaSkin) -> Result<(FluXisSkin, FluXisLayout), Box<dyn std::error::Error>> {
    let mut textures = skin.textures.clone();
    let mut samples = skin.samples.clone();
    let mut fluxis_keymodes: Vec<skin_json::Keymode> = Vec::new();

    let blank_texture: Arc<RwLock<Texture>> = textures.get_shared("blank")
        .unwrap_or(Arc::new(RwLock::new(Texture::from_blank("blank".to_string()))));

    let mut body_processor = TextureProcessor::<()>::new();
    let mut tail_processor = TextureProcessor::<()>::new();

    let resize = Resizer::new(
        skin.resolution,
        Some(Vector2::new(FluXisDimensions::X.as_u32(), FluXisDimensions::Y.as_u32()))
    );
    
    for keymode in &skin.keymodes {
        let key_count = keymode.keymode as u8;
        
        let receptor_images: Vec<String> = keymode.receptor_up
            .iter()
            .map(|receptor| receptor.get_path().unwrap_or_default())
            .collect();
        
        let receptor_images_down: Vec<String> = keymode.receptor_down
            .iter()
            .map(|receptor| receptor.get_path().unwrap_or_default())
            .collect();
        
        let normal_note_images: Vec<String> = keymode.normal_note
            .iter()
            .map(|note| note.get_path().unwrap_or_default())
            .collect();
        
        let long_note_head_images: Vec<String> = keymode.long_note_head
            .iter()
            .map(|note| note.get_path().unwrap_or_default())
            .collect();

        
        // you can't do percy in fluXis (at least not above 4096px)
        let long_note_body_images: Vec<String> = keymode.long_note_body
            .iter()
            .map(|note| {
                if let Some(texture_arc) = &note.texture {
                    if !Arc::ptr_eq(texture_arc, &blank_texture) {
                        body_processor.process_once_void(texture_arc, |arc_texture| {
                            arc_texture.data_mut(|img| {
                                let (width, height) = img.dimensions();
                                let max_res = FluXisDimensions::MaxResolution.as_u32();
                                
                                if width > max_res || height > max_res {
                                    *img = img.resize_exact(
                                        width.min(max_res),
                                        height.min(max_res),
                                        image::imageops::FilterType::Lanczos3
                                    );
                                }
                            });
                        });
                    }
                }
                note.get_path().unwrap_or_default()
            })
            .collect();

        let long_note_tail_images: Vec<String> = keymode.long_note_tail
            .iter()
            .map(|note| {
                if let Some(texture_arc) = &note.texture {
                    if !Arc::ptr_eq(texture_arc, &blank_texture) {
                        tail_processor.process_once_void(texture_arc, |arc_texture| {
                            arc_texture.data_mut(|img| {
                                let (width, height) = img.dimensions();
                                let max_res = FluXisDimensions::MaxResolution.as_u32();
                                
                                if width > max_res || height > max_res {
                                    *img = img.resize_exact(
                                        width.min(max_res),
                                        height.min(max_res),
                                        image::imageops::FilterType::Lanczos3
                                    );
                                }
                            });
                        });
                    }
                }
                note.get_path().unwrap_or_default()
            })
            .collect();

        let hit_pos = (keymode.layout.hit_position * FluXisDimensions::Y.as_f32()) + (keymode.layout.receptor_offset as f32 - (keymode.layout.hit_position * resize.source.y as f32));
        
        fluxis_keymodes.push(skin_json::Keymode {
            keymode: key_count,
            receptor_images,
            receptor_images_down,
            normal_note_images,
            long_note_head_images,
            long_note_body_images,
            long_note_tail_images,
            receptors_first: !keymode.layout.receptor_above_notes,
            hit_position: ((hit_pos + (keymode.layout.receptor_offset as f32 - hit_pos)) as i32)
                .clamp(0, FluXisDimensions::Y.as_i32()),
            receptor_offset: keymode.layout.receptor_offset,
            column_width: resize.to_target_x::<u32>(keymode.layout.column_widths
                .get(0).copied()
                .unwrap_or(0.0)),
            tint_notes: false,
            tint_lns: false,
            tint_receptors: false,
            ..Default::default()
        });
    }
    
    let health_foreground = skin.gameplay.health_bar.fill.get_path().unwrap_or_default();
    let health_background = skin.gameplay.health_bar.background.get_path().unwrap_or_default();
    
    let mut skin_json = SkinJson {
        info: Info {
            name: skin.metadata.name.clone(),
            creator: skin.metadata.creator.clone(),
            ..Default::default()
        },
        keymodes: fluxis_keymodes,
        overrides: Overrides::default(),
        judgements: JudgementColors::default(),
        snap_colors: SnapColors::default(),
    };

    if let Some(flawless_arc) = &skin.gameplay.judgement.flawless {
        textures.copy(&flawless_arc.get_path(), static_assets::Judgement::FLAWLESS);
    }

    if let Some(perfect_arc) = &skin.gameplay.judgement.perfect {
        textures.copy(&perfect_arc.get_path(), static_assets::Judgement::PERFECT);
    }

    if let Some(great_arc) = &skin.gameplay.judgement.great {
        textures.copy(&great_arc.get_path(), static_assets::Judgement::GREAT);
    }

    if let Some(good_arc) = &skin.gameplay.judgement.good {
        textures.copy(&good_arc.get_path(), static_assets::Judgement::ALRIGHT);
    }

    if let Some(bad_arc) = &skin.gameplay.judgement.bad {
        textures.copy(&bad_arc.get_path(), static_assets::Judgement::OKAY);
    }

    if let Some(miss_arc) = &skin.gameplay.judgement.miss {
        textures.copy(&miss_arc.get_path(), static_assets::Judgement::MISS);
    }

    skin_json.overrides.health.foreground = health_foreground;
    skin_json.overrides.health.background = health_background;
    skin_json.overrides.lighting.column_lighting = skin.keymodes.first().unwrap().column_lighting.get_path().unwrap_or_default();
    skin_json.overrides.stage.hitline = if skin.keymodes.first().unwrap().layout.show_judgement_line {
        skin.keymodes.first().unwrap().judgement_line.get_path().unwrap_or_else(|| "blank".to_string())
    } else {
        "blank".to_string()
    };
    textures.copy(&skin.gameplay.stage.get_path().unwrap_or_default(), "Stage/background");
    skin_json.overrides.stage.border_right = skin.gameplay.stage.border_right.get_path().unwrap_or_default();
    skin_json.overrides.stage.border_left = skin.gameplay.stage.border_left.get_path().unwrap_or_default();
    skin_json.sync_overrides_from_keymodes();

    if let Some(s) = &skin.sounds.ui.menu_back_click {
        samples.copy(s, static_assets::Samples::UI_BACK);
    }
    
    if let Some(s) = &skin.sounds.ui.ui_click {
        samples.copy(s, static_assets::Samples::UI_CLICK);
    }
    
    if let Some(s) = &skin.sounds.ui.ui_select {
        samples.copy(s, static_assets::Samples::UI_SELECT);
    }
    
    if let Some(s) = &skin.sounds.ui.ui_hover {
        samples.copy(s, static_assets::Samples::UI_HOVER);
    }
    
    if let Some(s) = &skin.sounds.gameplay.miss {
        samples.copy(s, static_assets::Samples::GAMEPLAY_MISS);
    }
    
    if let Some(s) = &skin.sounds.gameplay.fail {
        samples.copy(s, static_assets::Samples::GAMEPLAY_FAIL);
    }
    
    if let Some(s) = &skin.sounds.gameplay.restart {
        samples.copy(s, static_assets::Samples::GAMEPLAY_RESTART);
    }
    
    if let Some(s) = &skin.sounds.mania.hit {
        samples.copy(s, static_assets::Samples::GAMEPLAY_HIT);
    }

    if let Some(preview) = generate_fluxis_preview(&skin_json, &textures, 512, 512).ok() {
        textures.insert(Texture::with_data("icon".to_string(), preview));
    }

    cleanup_stores(&skin_json, Some(&mut textures), Some(&mut samples));

    let fluxis_skin = FluXisSkin::new(skin_json, Some(textures), Some(samples));

    let mut layout = FluXisLayout::new(skin.metadata.name.clone(), skin.metadata.creator.clone());

    let fluxis_x = FluXisDimensions::X.as_f32();
    let fluxis_y = FluXisDimensions::Y.as_f32();

    let default_combo_comp = Combo::default().component;
    let (combo_pos, combo_align) = &skin.gameplay.layout.combo;
    layout.add_component_to_gameplay("Combo".to_string(), Component {
        position: Position {
            x: combo_pos.x * fluxis_x,
            y: combo_pos.y * fluxis_y,
        },
        scale: combo_pos.z,
        anchor: combo_align.anchor as u8,
        origin: combo_align.origin as u8,
        anchor_to_playfield: default_combo_comp.anchor_to_playfield,
        settings: default_combo_comp.settings
    });

    let default_rating_comp = PerformanceRating::default().component;
    let (rating_pos, rating_align) = &skin.gameplay.layout.rating;
    layout.add_component_to_gameplay("PerformanceRating".to_string(), Component {
        position: Position {
            x: rating_pos.x * fluxis_x,
            y: rating_pos.y * fluxis_y,
        },
        scale: rating_pos.z,
        anchor: rating_align.anchor as u8,
        origin: rating_align.origin as u8,
        anchor_to_playfield: default_rating_comp.anchor_to_playfield,
        settings: default_rating_comp.settings
    });

    let default_kps_comp = KeysPerSecond::default().component;
    let (kps_pos, kps_align) = &skin.gameplay.layout.rating;
    layout.add_component_to_gameplay("KeysPerSecond".to_string(), Component {
        position: Position {
            x: kps_pos.x * fluxis_x,
            y: kps_pos.y * fluxis_y,
        },
        scale: kps_pos.z,
        anchor: kps_align.anchor as u8,
        origin: kps_align.origin as u8,
        anchor_to_playfield: default_kps_comp.anchor_to_playfield,
        settings: default_kps_comp.settings
    });

    let default_accuracy_comp = Accuracy::default().component;
    let (accuracy_pos, accuracy_align) = &skin.gameplay.layout.accuracy;
    layout.add_component_to_gameplay("Accuracy".to_string(), Component {
        position: Position {
            x: accuracy_pos.x * fluxis_x,
            y: accuracy_pos.y * fluxis_y,
        },
        scale: accuracy_pos.z,
        anchor: accuracy_align.anchor as u8,
        origin: accuracy_align.origin as u8,
        anchor_to_playfield: default_accuracy_comp.anchor_to_playfield,
        settings: default_accuracy_comp.settings
    });
    
    Ok((fluxis_skin, layout))
}