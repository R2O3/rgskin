use std::sync::Arc;
use crate::common::{Alignment, Anchor, Origin, Vector3};
use crate::extensions::TextureArcExt;
use crate::generic::{Gameplay, Keymode, Metadata,};
use crate::generic::layout::{HUDLayout, KeymodeLayout};
use crate::generic::elements::*;
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
use crate::{GenericManiaSkin, Store, Texture};

pub fn to_generic_mania(skin: FluXisSkin, layout: Option<FluXisLayout>) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures;
    let layout = layout.unwrap_or(FluXisLayout::default());
    let mut keymodes: Vec<Keymode> = Vec::new();

    textures.insert(Texture::empty("blank".to_string()));
    let blank_texture = textures.get_shared("blank").unwrap();

    let metadata = Metadata {
        name: skin.skin_json.info.name.clone(),
        creator: skin.skin_json.info.creator.clone(),
        center_cursor: false,
        ..Default::default()
    };

    // let mut processed_textures = HashSet::new();

    for keymode in &skin.skin_json.keymodes {
        let key_count = keymode.keymode as usize;
        let receptor_up_elements: Vec<ReceptorUp> = keymode.receptor_images
        .iter()
            .map(|path| {
                if !path.is_empty() {
                    if let Some(texture) = textures.get_shared(path) {
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

        let long_note_head_elements: Vec<LongNoteHead> = keymode.normal_note_images
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
                if !path.is_empty() && textures.contains(path) {
                    LongNoteTail::new(textures.get_shared(path).unwrap())
                } else {
                    LongNoteTail::new(Arc::clone(&blank_texture))
                }
            })
            .collect();

        let new_layout = KeymodeLayout {
            keymode: key_count as u8,
            receptor_above_notes: !keymode.receptors_first,
            x_offset: 0.5,
            hit_position: keymode.hit_position,
            receptor_offset: keymode.receptor_offset,
            column_widths: vec![keymode.column_width; key_count],
            column_spacing: vec![0; key_count],
        };

        keymodes.push(Keymode { 
            keymode: key_count as u8,
            layout: new_layout,
            receptor_up: receptor_up_elements,
            receptor_down: receptor_down_elements,
            normal_note: normal_note_elements,
            long_note_head: long_note_head_elements,
            long_note_body: long_note_body_elements,
            long_note_tail: long_note_tail_elements
        });
    }

    let combo_hud = layout.gameplay.components.get("Combo").unwrap();
    let rating_hud = layout.gameplay.components.get("PerformanceRating").unwrap();
    let accuracy_hud = layout.gameplay.components.get("Accuracy").unwrap();

    let gameplay = Gameplay {
        health_bar: Healthbar::new(
            textures.get_shared(&skin.skin_json.overrides.stage.health_foreground).unwrap(),
            textures.get_shared(&skin.skin_json.overrides.stage.health_background).unwrap()
        ),
        layout: HUDLayout {
            combo: (
                Vector3::new(
                    combo_hud.position.x / 1920.0,
                    combo_hud.position.y / 1080.0,
                    combo_hud.scale
                ),
                Alignment { 
                    anchor: Anchor::from_u8(combo_hud.anchor).unwrap_or_default(), 
                    origin: Origin::from_u8(combo_hud.origin).unwrap_or_default() 
                }
            ),
            rating: (
                Vector3::new(
                    rating_hud.position.x / 1920.0,
                    rating_hud.position.y / 1080.0,
                    rating_hud.scale
                ),
                Alignment { 
                    anchor: Anchor::from_u8(rating_hud.anchor).unwrap_or_default(), 
                    origin: Origin::from_u8(rating_hud.origin).unwrap_or_default() 
                }
            ),
            accuracy: (
                Vector3::new(
                    accuracy_hud.position.x / 1920.0,
                    accuracy_hud.position.y / 1080.0,
                    accuracy_hud.scale
                ),
                Alignment { 
                    anchor: Anchor::from_u8(accuracy_hud.anchor).unwrap_or_default(), 
                    origin: Origin::from_u8(accuracy_hud.origin).unwrap_or_default() 
                }
            ),
            score: (
                Vector3::new(
                    -187.5 / 1920.0,
                    0.0,
                    1.0
                ),
                Alignment { anchor: Anchor::TopRight, origin: Origin::TopRight }
            ),
        }
    };

    Ok(GenericManiaSkin { 
        metadata, 
        gameplay, 
        keymodes, 
        textures 
    })
}

pub fn from_generic_mania(skin: GenericManiaSkin) -> Result<(FluXisSkin, FluXisLayout), Box<dyn std::error::Error>> {
    let mut fluxis_keymodes: Vec<skin_json::Keymode> = Vec::new();
    
    for keymode in &skin.keymodes {
        let key_count = keymode.keymode as u8;
        
        let receptor_images: Vec<String> = keymode.receptor_up
            .iter()
            .map(|receptor| receptor.path())
            .collect();
        
        let receptor_images_down: Vec<String> = keymode.receptor_down
            .iter()
            .map(|receptor| receptor.path())
            .collect();
        
        let normal_note_images: Vec<String> = keymode.normal_note
            .iter()
            .map(|note| note.path())
            .collect();
        
        let long_note_body_images: Vec<String> = keymode.long_note_body
            .iter()
            .map(|note| note.path())
            .collect();
        
        let long_note_tail_images: Vec<String> = keymode.long_note_tail
            .iter()
            .map(|note| note.path())
            .collect();
        
        fluxis_keymodes.push(skin_json::Keymode {
            keymode: key_count,
            receptor_images,
            receptor_images_down,
            normal_note_images,
            long_note_body_images,
            long_note_tail_images,
            receptors_first: !keymode.layout.receptor_above_notes,
            hit_position: keymode.layout.hit_position,
            receptor_offset: keymode.layout.receptor_offset,
            column_width: keymode.layout.column_widths.get(0).copied().unwrap_or(0),
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
    skin_json.sync_overrides_from_stage();
    skin_json.sync_overrides_from_keymodes();
    
    let fluxis_skin = FluXisSkin {
        skin_json,
        textures: skin.textures,
    };

    let mut layout = FluXisLayout::new(skin.metadata.name.clone(), skin.metadata.creator.clone());

    let default_combo_comp = Combo::default().component;
    let (combo_pos, combo_align) = &skin.gameplay.layout.combo;
    layout.add_component_to_gameplay("Combo".to_string(), Component {
        position: Position {
            x: combo_pos.x * 1920.0,
            y: combo_pos.y * 1080.0,
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
            x: rating_pos.x * 1920.0,
            y: rating_pos.y * 1080.0,
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
            x: kps_pos.x * 1920.0,
            y: kps_pos.y * 1080.0,
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
            x: accuracy_pos.x * 1920.0,
            y: accuracy_pos.y * 1080.0,
        },
        scale: accuracy_pos.z,
        anchor: accuracy_align.anchor as u8,
        origin: accuracy_align.origin as u8,
        anchor_to_playfield: default_accuracy_comp.anchor_to_playfield,
        settings: default_accuracy_comp.settings
    });
    
    Ok((fluxis_skin, layout))
}