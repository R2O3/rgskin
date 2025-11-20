use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use crate::common::{Alignment, Anchor, Origin, Vector2, Vector3};
use crate::extensions::TextureArcExt;
use crate::generic::Gameplay;
use crate::image_proc::proc::{dist_from_bottom, flip_vertical, to_osu_column, to_osu_column_draw};
use crate::io::{Store, Texture};
use crate::osu::{self, General, OsuSkin, SkinIni};
use crate::skin::generic::layout::{HUDLayout, KeymodeLayout};
use crate::skin::generic::{elements::*, Keymode, Metadata, GenericManiaSkin};
use crate::utils::osu::OsuDimensions;

pub fn to_generic_mania(skin: OsuSkin) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures;
    let mut keymodes: Vec<Keymode> = Vec::new();

    textures.insert(Texture::empty("blank".to_string()));
    let blank_texture = textures.get_shared("blank").unwrap();

    let metadata = Metadata {
        name: skin.skin_ini.general.name.clone(),
        creator: skin.skin_ini.general.author.clone(),
        version: skin.skin_ini.general.version.clone(),
        center_cursor: skin.skin_ini.general.cursor_centre.clone()
    };

    let mut processed_textures = HashSet::new();

    for keymode in &skin.skin_ini.keymodes {
        let key_count = keymode.keymode as usize;
        let average_column_width = keymode.column_width.iter().sum::<u32>() / keymode.column_width.len() as u32;
        let mut receptor_offset = 0;

        let receptor_up_elements: Vec<ReceptorUp> = keymode.receptor_images
            .iter()
            .map(|path| {
                if !path.is_empty() {
                    if let Some(texture) = textures.get_shared(path) {
                        let texture_path = texture.get_path();
                        let mut texture_already_processed = false;

                        if processed_textures.contains(&texture_path) {
                            texture_already_processed = true;
                        } else {
                            processed_textures.insert(texture_path);
                        }

                        if !texture_already_processed {
                            receptor_offset = texture.with_image(|img| dist_from_bottom(img, 0.1));
                            
                            if let Err(e) = to_osu_column_draw(&texture, average_column_width) {
                                eprintln!("Failed to process receptor up texture {}: {}", path, e);
                            }
                        }
                        
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
                        let texture_path = texture.get_path();
                        let mut texture_already_processed = false;

                        if processed_textures.contains(&texture_path) {
                            texture_already_processed = true;
                        } else {
                            processed_textures.insert(texture_path);
                        }

                        if !texture_already_processed {
                            receptor_offset = texture.with_image(|img| dist_from_bottom(img, 0.1));

                            if let Err(e) = to_osu_column_draw(&texture, average_column_width) {
                                eprintln!("Failed to process receptor down texture {}: {}", path, e);
                            }
                        }
                        
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
                        let texture_path = texture.get_path();
                        let mut texture_already_processed = false;

                        if processed_textures.contains(&texture_path) {
                            texture_already_processed = true;
                        } else {
                            processed_textures.insert(texture_path);
                        }

                        if !texture_already_processed {
                            texture.replace_data(
                                texture.with_image(|img| flip_vertical(img.clone())) // too lazy for rust shinanigans so we're just gonna clone
                            );
                        }
                        
                        LongNoteTail::new(texture)
                    } else {
                        LongNoteTail::new(Arc::clone(&blank_texture))
                    }
                } else {
                    LongNoteTail::new(Arc::clone(&blank_texture))
                }
            })
            .collect();

        let layout = KeymodeLayout {
            keymode: key_count as u8,
            receptor_above_notes: !keymode.keys_under_notes,
            x_offset: keymode.column_start as f32 / OsuDimensions::X.as_f32(),
            hit_position: keymode.hit_position as f32 / OsuDimensions::Y.as_f32(),
            receptor_offset: receptor_offset as i32,
            column_widths: keymode.column_width.iter().map(|cw| *cw as f32 / OsuDimensions::X.as_f32()).collect(),
            column_spacing: keymode.column_spacing.clone(),
        };

        keymodes.push(Keymode { 
            keymode: key_count as u8,
            layout: layout,
            receptor_up: receptor_up_elements,
            receptor_down: receptor_down_elements,
            normal_note: normal_note_elements,
            long_note_head: long_note_head_elements,
            long_note_body: long_note_body_elements,
            long_note_tail: long_note_tail_elements
        });
    }

    let default_keymode = skin.skin_ini.keymodes[0].clone();
    let layout_keymode = skin.skin_ini.get_keymode(4).unwrap_or(&default_keymode);

    let gameplay = Gameplay {
        health_bar: Healthbar::new(
            textures.get_shared("scorebar-colour").unwrap(),
            textures.get_shared("scorebar-bg").unwrap()
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
    
    Ok(GenericManiaSkin { 
        metadata, 
        gameplay, 
        keymodes, 
        textures 
    })
}

pub fn from_generic_mania(skin: GenericManiaSkin) -> Result<OsuSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures;
    let mut osu_keymodes: Vec<osu::Keymode> = Vec::new();

    let blank_texture: Arc<RwLock<Texture>> = textures.get_shared("blank")
        .unwrap_or(Arc::new(RwLock::new(Texture::empty("blank".to_string()))));

    let general = General {
        name: skin.metadata.name,
        author: skin.metadata.creator,
        version: skin.metadata.version,
        cursor_centre: skin.metadata.center_cursor,
        ..Default::default()
    };

    let mut processed_textures = HashSet::new();

    for keymode in skin.keymodes {
        let average_column_width = keymode.layout.average_column_width();

        let receptor_images: Vec<String> = keymode.receptor_up
            .iter()
            .map(|receptor| {
                let texture_arc = &receptor.texture;

                let texture_path = texture_arc.get_path();
                let mut texture_already_processed = false;

                if processed_textures.contains(&texture_path) {
                    texture_already_processed = true;
                } else {
                    processed_textures.insert(texture_path);
                }

                if !Arc::ptr_eq(texture_arc, &blank_texture) && !texture_already_processed {
                    if let Err(e) = to_osu_column(texture_arc, (average_column_width * OsuDimensions::X.as_f32()) as u32, keymode.layout.receptor_offset.clamp(0, OsuDimensions::X.as_i32()) as u32) {
                        eprintln!("Failed to process receptor up texture: {}", e);
                    }
                }
                receptor.path()
            })
            .collect();

        let receptor_images_down: Vec<String> = keymode.receptor_down
            .iter()
            .map(|receptor| {
                let texture_arc = &receptor.texture;

                let texture_path = texture_arc.get_path();
                let mut texture_already_processed = false;

                if processed_textures.contains(&texture_path) {
                    texture_already_processed = true;
                } else {
                    processed_textures.insert(texture_path);
                }

                if !Arc::ptr_eq(texture_arc, &blank_texture) && !texture_already_processed {
                    if let Err(e) = to_osu_column(texture_arc, (average_column_width * OsuDimensions::X.as_f32()) as u32, keymode.layout.receptor_offset.clamp(0, OsuDimensions::X.as_i32()) as u32) {
                        eprintln!("Failed to process receptor down texture: {}", e);
                    }
                }
                receptor.path()
            })
            .collect();

        let normal_note_images: Vec<String> = keymode.normal_note
            .iter()
            .map(|note| note.path())
            .collect();

        let long_note_head_images: Vec<String> = keymode.long_note_head
            .iter()
            .map(|note| note.path())
            .collect();

        let long_note_body_images: Vec<String> = keymode.long_note_body
            .iter()
            .map(|note| note.path())
            .collect();

        let long_note_tail_images: Vec<String> = keymode.long_note_tail
            .iter()
            .map(|note| {
                let texture_arc = &note.texture;

                let texture_path = texture_arc.get_path();
                let mut texture_already_processed = false;

                if processed_textures.contains(&texture_path) {
                    texture_already_processed = true;
                } else {
                    processed_textures.insert(texture_path);
                }

                if !Arc::ptr_eq(texture_arc, &blank_texture) && !texture_already_processed {
                    texture_arc.replace_data(
                        texture_arc.with_image(|img| flip_vertical(img.clone()))
                    );
                }
                note.path()
            })
            .collect();

        if let Some(health_bar_bg) = skin.gameplay.health_bar.background.get_data() {
            textures.insert(Texture::with_data("scorebar-bg".to_string(), health_bar_bg));
        }

        if let Some(health_bar_colour) = skin.gameplay.health_bar.fill.get_data() {
            textures.insert(Texture::with_data("scorebar-colour".to_string(), health_bar_colour));
        }

        // these wouldn't be present in other skins
        if !textures.contains("star") {
            textures.insert(Texture::with_data("star".to_string(), blank_texture.clone_data().unwrap()));
        }

        if !textures.contains("star2") {
            textures.insert(Texture::with_data("star2".to_string(), blank_texture.clone_data().unwrap()));
        }

        let osu_keymode = osu::Keymode {
            keymode: keymode.keymode,
            keys_under_notes: !keymode.layout.receptor_above_notes,
            hit_position: (((1.0 - keymode.layout.hit_position).abs() * (OsuDimensions::Y.as_f32()))) as u32,
            column_start: (keymode.layout.x_offset * OsuDimensions::X.as_f32()) as u32,
            column_width: keymode.layout.column_widths.iter().map(|cw| (*cw * OsuDimensions::X.as_f32()) as u32).collect(),
            column_spacing: keymode.layout.column_spacing,
            column_line_width: vec![0; keymode.keymode as usize + 1], // osu skins are the only skins that support line widths so no need to implement in generic skin
            receptor_images,
            receptor_images_down,
            normal_note_images,
            long_note_head_images,
            long_note_body_images,
            long_note_tail_images,
            judgement_line : false,
            ..Default::default()
        };

        osu_keymodes.push(osu_keymode);
    }
    
    let mut skin_ini = SkinIni {
        general,
        keymodes: osu_keymodes,
    };

    let osu_dimensions = Vector2::new(OsuDimensions::X.as_f32(), OsuDimensions::Y.as_f32());

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