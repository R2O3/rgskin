use std::sync::Arc;
use crate::common::alignment::*;
use crate::common::color::Rgba;
use crate::common::vector::*;
use crate::extensions::TextureArcExt;
use crate::generic::{Gameplay, Keymode, Metadata,};
use crate::generic::layout::{HUDLayout, KeymodeLayout};
use crate::generic::elements::*;
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
use crate::utils::fluxis::FluXisDimensions;
use crate::utils::math::Resizer;
use crate::GenericManiaSkin;

pub fn to_generic_mania(skin: FluXisSkin, layout: Option<FluXisLayout>) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures;
    let layout = layout.unwrap_or(FluXisLayout::default());
    let mut keymodes: Vec<Keymode> = Vec::new();

    textures.insert(Texture::from_blank("blank".to_string()));
    let blank_texture = textures.get_shared("blank").unwrap();

    let metadata = Metadata {
        name: skin.skin_json.info.name.clone(),
        creator: skin.skin_json.info.creator.clone(),
        center_cursor: false,
        ..Default::default()
    };

    let mut receptor_processor = TextureProcessor::<i32>::new();

    for keymode in skin.skin_json.keymodes {
        let key_count = keymode.keymode as usize;
        let mut max_additional_offset = 0;

        let receptor_up_elements: Vec<ReceptorUp> = keymode.receptor_images
            .iter()
            .map(|path| {
                if !path.is_empty() {
                    if let Some(texture) = textures.get_shared(path) {
                        let offset = receptor_processor.process_once(&texture, |arc| {
                            arc.with_image(|img| dist_from_bottom(img, 0.1)) as i32
                        });
                        max_additional_offset = max_additional_offset.max(offset);
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
                        let offset = receptor_processor.process_once(&texture, |tex| {
                            tex.with_image(|img| dist_from_bottom(img, 0.1)) as i32
                        });
                        max_additional_offset = max_additional_offset.max(offset);
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

        let fallback_to_normal = keymode.long_note_head_images.iter().all(|path| path.is_empty());

        let long_note_head_elements: Vec<LongNoteHead> = if fallback_to_normal {
            keymode.normal_note_images
            .iter()
            .map(|path| {
                if !path.is_empty() && textures.contains(path) {
                    LongNoteHead::new(textures.get_shared(path).unwrap())
                } else {
                    LongNoteHead::new(Arc::clone(&blank_texture))
                }
            })
            .collect()
        } else {
            keymode.long_note_head_images
                .iter()
                .map(|path| {
                    if !path.is_empty() && textures.contains(path) {
                        LongNoteHead::new(textures.get_shared(path).unwrap())
                    } else {
                        LongNoteHead::new(Arc::clone(&blank_texture))
                    }
                })
                .collect()
        };

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
                if !path.is_empty() && textures.contains(path) {
                    LongNoteTail::new(textures.get_shared(path).unwrap())
                } else {
                    LongNoteTail::new(Arc::clone(&blank_texture))
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
            column_spacing: vec![0; key_count],
        };

        let column_lighting_path = &skin.skin_json.overrides.stage.column_lighting;
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
            hit_lighting: HitLighting { normal: blank_texture.clone(),
                hold: blank_texture.clone() },
            column_lighting: ColumnLighting { texture: texture_or_blank(column_lighting_path) },
            judgement_line: JudgementLine {
                texture: if !show_judgement_line {
                    blank_texture.clone()
                } else {
                    texture_or_blank(&skin.skin_json.overrides.stage.hitline)
                },
                color: Rgba::default(),
            },
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
            textures.get_shared(&skin.skin_json.overrides.stage.health_foreground).unwrap(),
            textures.get_shared(&skin.skin_json.overrides.stage.health_background).unwrap()
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

    Ok(GenericManiaSkin {
        resolution: skin.resolution,
        metadata, 
        gameplay, 
        keymodes, 
        textures 
    })
}

pub fn from_generic_mania(skin: GenericManiaSkin) -> Result<(FluXisSkin, FluXisLayout), Box<dyn std::error::Error>> {
    let mut fluxis_keymodes: Vec<skin_json::Keymode> = Vec::new();

    let resize = Resizer::new(
        skin.resolution,
        Some(Vector2::new(FluXisDimensions::X.as_u32(), FluXisDimensions::Y.as_u32()))
    );
    
    for keymode in &skin.keymodes {
        let key_count = keymode.keymode as u8;
        
        let receptor_images: Vec<String> = keymode.receptor_up
            .iter()
            .map(|receptor| receptor.get_path())
            .collect();
        
        let receptor_images_down: Vec<String> = keymode.receptor_down
            .iter()
            .map(|receptor| receptor.get_path())
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
            .map(|note| note.get_path())
            .collect();
        
        fluxis_keymodes.push(skin_json::Keymode {
            keymode: key_count,
            receptor_images,
            receptor_images_down,
            normal_note_images,
            long_note_head_images,
            long_note_body_images,
            long_note_tail_images,
            receptors_first: !keymode.layout.receptor_above_notes,
            hit_position: resize.to_target_y::<i32>(keymode.layout.hit_position)
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
    
    let health_foreground = skin.gameplay.health_bar.fill.get_path();
    let health_background = skin.gameplay.health_bar.background.get_path();
    
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

    skin_json.overrides.stage.health_foreground = health_foreground;
    skin_json.overrides.stage.health_background = health_background;
    skin_json.overrides.stage.column_lighting = skin.keymodes.first().unwrap().column_lighting.get_path();
    skin_json.overrides.stage.hitline = if skin.keymodes.first().unwrap().layout.show_judgement_line {
        skin.keymodes.first().unwrap().judgement_line.get_path()
    } else {
        "blank".to_string()
    };
    skin_json.sync_overrides_from_stage();
    skin_json.sync_overrides_from_keymodes();

    let fluxis_skin = FluXisSkin::new(skin_json, Some(skin.textures));

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