use std::sync::Arc;

use crate::utils::quaver::QuaDimensions;
use crate::{BinaryArcExt, quaver};
use crate::common::alignment::{Alignment, Anchor, Origin};
use crate::common::color::Rgba;
use crate::common::vector::Vector3;
use crate::extensions::TextureArcExt;
use crate::generic::elements::{
    ColumnLighting, Cursor, Healthbar, HitLighting, Judgement, JudgementLine, LongNoteBody,
    LongNoteHead, LongNoteTail, NormalNote, ReceptorDown, ReceptorUp, Stage,
};
use crate::generic::layout::{HUDLayout, KeymodeLayout};
use crate::generic::sound::{GenericGameplaySounds, ManiaGameplaySounds, Sounds, UISounds};
use crate::generic::{Gameplay, UI};
use crate::image_proc::proc::{dist_from_bottom, trim_image_vertical};
use crate::io::texture::{Texture, TextureProcessor};
use crate::io::Store;
use crate::skin::generic::{GenericManiaSkin, Keymode, Metadata};
use crate::skin::quaver::skin::QuaSkin;
use crate::skin::quaver::QuaSkinIni;
use crate::traits::KeymodeInvariant;
use crate::utils::skin::cleanup_stores;

pub fn to_generic_mania(skin: &QuaSkin) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures.clone();
    let samples = skin.samples.clone();
    let mut keymodes: Vec<Keymode> = Vec::new();

    textures.insert(Texture::from_blank("blank".to_string()));
    let blank_texture = textures.get_shared("blank").unwrap();

    let metadata = Metadata {
        name: skin.skin_ini.general.name.clone(),
        creator: skin.skin_ini.general.author.clone(),
        version: skin.skin_ini.general.version.clone(),
    };

    let mut receptor_processor = TextureProcessor::<i32>::new();

    for keymode in &skin.skin_ini.keymodes {
        let key_count = keymode.keymode as usize;
        let mut max_receptor_offset = 0;

        let fallbacks: Vec<_> = (1..=key_count)
            .map(|lane| keymode.primary_fallback(lane))
            .collect();

        let receptors = keymode.get_receptors();
        let receptors_down = keymode.get_receptors_down();
        let normal_notes = keymode.get_normal_notes();
        let long_note_heads = keymode.get_long_note_heads();
        let long_note_bodies = keymode.get_long_note_bodies();
        let long_note_tails = keymode.get_long_note_tails();

        let receptor_up_elements: Vec<ReceptorUp> = receptors
            .iter()
            .zip(fallbacks.iter().map(|f| &f.receptor))
            .map(|(path, fallback_path)| {
                let tex_path = if textures.contains(path) { path } else { fallback_path };
                if let Some(texture) = textures.get_shared(tex_path) {
                    let offset = receptor_processor.process_once(&texture, |arc| {
                        arc.with_image(|img| dist_from_bottom(img, 0.1)).try_into().unwrap_or(0)
                    });
                    receptor_processor.process_once_void(&texture, |arc| {
                        arc.data_mut(|img| {
                            *img = trim_image_vertical(img.clone(), 0.2);
                        });
                    });
                    max_receptor_offset = max_receptor_offset.max(offset);
                    ReceptorUp::new(Some(texture))
                } else {
                    ReceptorUp::new(Some(Arc::clone(&blank_texture)))
                }
            })
            .collect();

        let receptor_down_elements: Vec<ReceptorDown> = receptors_down
            .iter()
            .zip(fallbacks.iter().map(|f| &f.receptor_down))
            .map(|(path, fallback_path)| {
                let tex_path = if textures.contains(path) { path } else { fallback_path };
                if let Some(texture) = textures.get_shared(tex_path) {
                    let offset = receptor_processor.process_once(&texture, |arc| {
                        arc.with_image(|img| dist_from_bottom(img, 0.1)).try_into().unwrap_or(0)
                    });
                    receptor_processor.process_once_void(&texture, |arc| {
                        arc.data_mut(|img| {
                            *img = trim_image_vertical(img.clone(), 0.2);
                        });
                    });
                    max_receptor_offset = max_receptor_offset.max(offset);
                    ReceptorDown::new(Some(texture))
                } else {
                    ReceptorDown::new(Some(Arc::clone(&blank_texture)))
                }
            })
            .collect();

        let normal_note_elements: Vec<NormalNote> = normal_notes
            .iter()
            .zip(fallbacks.iter().map(|f| &f.normal_note))
            .map(|(path, fallback_path)| {
                let tex_path = if textures.contains(path) { path } else { fallback_path };
                if let Some(texture) = textures.get_shared(tex_path) {
                    NormalNote::new(Some(texture))
                } else {
                    NormalNote::new(Some(Arc::clone(&blank_texture)))
                }
            })
            .collect();

        let long_note_head_elements: Vec<LongNoteHead> = long_note_heads
            .iter()
            .zip(fallbacks.iter().map(|f| &f.long_note_head))
            .map(|(path, fallback_path)| {
                let tex_path = if textures.contains(path) { path } else { fallback_path };
                if let Some(texture) = textures.get_shared(tex_path) {
                    LongNoteHead::new(Some(texture))
                } else {
                    LongNoteHead::new(Some(Arc::clone(&blank_texture)))
                }
            })
            .collect();

        let long_note_body_elements: Vec<LongNoteBody> = long_note_bodies
            .iter()
            .zip(fallbacks.iter().map(|f| &f.long_note_body))
            .map(|(path, fallback_path)| {
                let tex_path = if textures.contains(path) { path } else { fallback_path };
                if let Some(texture) = textures.get_shared(tex_path) {
                    LongNoteBody::new(Some(texture))
                } else {
                    LongNoteBody::new(Some(Arc::clone(&blank_texture)))
                }
            })
            .collect();

        let long_note_tail_elements: Vec<LongNoteTail> = long_note_tails
            .iter()
            .zip(fallbacks.iter().map(|f| &f.long_note_tail))
            .map(|(path, fallback_path)| {
                let tex_path = if textures.contains(path) { path } else { fallback_path };
                if let Some(texture) = textures.get_shared(tex_path) {
                    LongNoteTail::new(Some(texture))
                } else {
                    LongNoteTail::new(Some(Arc::clone(&blank_texture)))
                }
            })
            .collect();

        let layout = KeymodeLayout {
            keymode: key_count as u8,
            receptor_above_notes: keymode.receptors_over_hit_objects,
            show_judgement_line: false,
            x_offset: 0.5,
            hit_position: 1.0 - (keymode.hit_pos_offset_y as f32 / 768.0),
            receptor_offset: (keymode.receptor_pos_offset_y + max_receptor_offset as i32),
            column_widths: vec![keymode.column_size as f32 / 1366.0; key_count],
            column_spacing: vec![0.0; key_count],
        };

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
                normal: Some(Arc::clone(&blank_texture)),
                hold: Some(Arc::clone(&blank_texture)),
            },
            column_lighting: ColumnLighting { texture: Some(Arc::clone(&blank_texture)) },
            judgement_line: JudgementLine { texture: Some(Arc::clone(&blank_texture)), color: Rgba::default() },
            stage: Stage::new(None, None, None),
            fallbacks,
        });
    }

    let ui = UI {
        cursor: Cursor {
            texture: textures.get_shared("cursor").or_else(|| Some(blank_texture.clone())),
            centered: skin.skin_ini.general.center_cursor,
        }
    };


    // TODO: use static_assets for alignments
    let gameplay = Gameplay {
        health_bar: Healthbar::new(None, None),
        judgement: Judgement::new(None, None, None, None, None, None),
        layout: HUDLayout {
            combo: (Vector3::new(0.0, 0.0, 1.0), Alignment { anchor: Anchor::TopLeft, origin: Origin::TopLeft }),
            rating: (Vector3::new(0.0, 0.0, 1.0), Alignment { anchor: Anchor::TopLeft, origin: Origin::TopLeft }),
            accuracy: (Vector3::new(0.0, 0.0, 1.0), Alignment { anchor: Anchor::TopLeft, origin: Origin::TopLeft }),
            score: (Vector3::new(0.0, 0.0, 1.0), Alignment { anchor: Anchor::TopLeft, origin: Origin::TopLeft }),
            judgement: (Vector3::new(0.0, 0.0, 1.0), Alignment { anchor: Anchor::TopLeft, origin: Origin::TopLeft }),
        },
    };

    let sounds = Sounds {
        ui: UISounds { menu_back_click: None, ui_click: None, ui_select: None, ui_hover: None },
        gameplay: GenericGameplaySounds { miss: None, fail: None, restart: None },
        mania: ManiaGameplaySounds { hit: None },
    };

    Ok(GenericManiaSkin {
        resolution: skin.resolution,
        sounds,
        metadata,
        ui,
        gameplay,
        keymodes,
        textures,
        samples,
    })
}

pub fn from_generic_mania(skin: &GenericManiaSkin) -> Result<QuaSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures.clone();
    let mut samples = skin.samples.clone();
    
    let mut skin_ini = QuaSkinIni::default();
    skin_ini.general.name = skin.metadata.name.clone();
    skin_ini.general.author = skin.metadata.creator.clone();
    skin_ini.general.version = skin.metadata.version.clone();
    skin_ini.general.center_cursor = skin.ui.cursor.centered;
    skin_ini.general.use_skin_backgrounds = false;

    let mut qua_keymodes = Vec::new();

    for keymode in &skin.keymodes {
        let mut qua_km = quaver::Keymode::default();

        let min_receptor_height = keymode.receptor_up.iter()
            .filter_map(|r| r.texture.as_ref())
            .chain(keymode.receptor_down.iter().filter_map(|r| r.texture.as_ref()))
            .filter_map(|tex| textures.get_shared(&tex.get_path()))
            .map(|arc| arc.with_image(|img| img.height()))
            .map(|h| h as i32)
            .min()
            .unwrap_or(0) as f32;
        
        qua_km.keymode = keymode.keymode;
        qua_km.receptors_over_hit_objects = keymode.layout.receptor_above_notes;
        qua_km.column_size = (keymode.layout.column_widths.get(0).copied().unwrap_or(0.0) * QuaDimensions::X.as_f32()) as i32;
        qua_km.receptor_pos_offset_y = keymode.layout.receptor_offset;
        qua_km.hit_pos_offset_y = (min_receptor_height - (keymode.layout.hit_position * QuaDimensions::Y.as_f32()) * 2.0).abs() as i32;

        let q_receptors = qua_km.get_receptors();
        let q_receptors_down = qua_km.get_receptors_down();
        let q_normal_notes = qua_km.get_normal_notes();
        let q_ln_heads = qua_km.get_long_note_heads();
        let q_ln_bodies = qua_km.get_long_note_bodies();
        let q_ln_tails = qua_km.get_long_note_tails();

        for i in 0..(keymode.keymode as usize) {
            if let Some(r) = keymode.receptor_up.get(i) {
                if let Some(tex) = &r.texture { textures.copy(&tex.get_path(), &q_receptors[i]); }
            }
            if let Some(r) = keymode.receptor_down.get(i) {
                if let Some(tex) = &r.texture { textures.copy(&tex.get_path(), &q_receptors_down[i]); }
            }
            if let Some(n) = keymode.normal_note.get(i) {
                if let Some(tex) = &n.texture { textures.copy(&tex.get_path(), &q_normal_notes[i]); }
            }
            if let Some(n) = keymode.long_note_head.get(i) {
                if let Some(tex) = &n.texture { textures.copy(&tex.get_path(), &q_ln_heads[i]); }
            }
            if let Some(n) = keymode.long_note_body.get(i) {
                if let Some(tex) = &n.texture { textures.copy(&tex.get_path(), &q_ln_bodies[i]); }
            }
            if let Some(n) = keymode.long_note_tail.get(i) {
                if let Some(tex) = &n.texture { textures.copy(&tex.get_path(), &q_ln_tails[i]); }
            }
        }
        
        qua_keymodes.push(qua_km);
    }

    skin_ini.keymodes = qua_keymodes;

    cleanup_stores(&skin_ini, Some(&mut textures), Some(&mut samples));

    Ok(QuaSkin::new(
        skin_ini,
        Some(textures),
        Some(samples),
    ))
}
