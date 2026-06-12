use std::sync::{Arc, RwLock};

use image::{DynamicImage, GenericImageView};

use crate::common::skin::AssetAttribute;
use crate::common::traits::LaneFallback;
use crate::quaver::config::keymode::HealthBarType;
use crate::quaver::{dynamic_assets, static_assets};
use crate::texture::Texture;
use crate::utils::quaver::{QuaDimensions, TextureResolver};
use crate::{Binary, BinaryArcExt, BinaryArcExtOption, ConstTypeEnum, Resources, StringPattern, quaver};
use crate::common::alignment::{Alignment, Anchor, Origin};
use crate::common::color::Rgba;
use crate::common::vector::Vector3;
use crate::extensions::{TextureArcExt, VecExtensions};
use crate::generic::elements::{
    BaseHoldHead, BaseNormalMine, BaseNormalNote, ColumnLighting, Cursor, Healthbar, HitLightingHold, HitLightingNormal, Judgement, JudgementLine, LongNoteBody, LongNoteHead, LongNoteHeadsSnapColored, LongNoteTail, NormalMine, NormalMinesSnapColored, NormalNote, NormalNotesSnapColored, ReceptorDown, ReceptorUp, SkinElement, Stage
};
use crate::generic::layout::{HUDLayout, KeymodeLayout};
use crate::generic::sound::{GenericGameplaySounds, ManiaGameplaySounds, Sounds, UISounds};
use crate::generic::{Gameplay, UI};
use crate::image_proc::proc::{concat_into_sheet, dist_from_bottom, rotate_90_deg_ccw, trim_image_vertical};
use crate::io::texture::TextureProcessor;
use crate::io::Store;
use crate::skin::generic::{GenericManiaSkin, Keymode, Metadata};
use crate::skin::quaver::skin::QuaSkin;
use crate::skin::quaver::QuaSkinIni;
use crate::traits::{KeymodeInvariant, ManiaSkin};
use crate::utils::skin::{StoreRelocator, cleanup_stores};

pub fn to_generic_mania(skin: &QuaSkin) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
    let mut textures = skin.textures.clone();
    let samples = skin.samples.clone();
    let mut keymodes: Vec<Keymode> = Vec::new();

    // we want to avoid making changes to the original skin_ini
    let mut skin_ini  = skin.skin_ini.clone();
    skin_ini.sync_from_shared();

    textures.insert(Texture::from_blank("blank".to_string()));
    let blank_texture = textures.get_shared("blank").unwrap();

    let metadata = Metadata {
        name: skin_ini.general.name.clone(),
        creator: skin_ini.general.author.clone(),
        version: skin_ini.general.version.clone(),
    };

    let mut receptor_processor = TextureProcessor::<i32>::new();

    let culled = textures.dedupe_all();

    let remap = |paths: Vec<String>| -> Vec<String> {
        paths.into_iter()
            .map(|p| culled.get(&p).cloned().unwrap_or(p))
            .collect()
    };

    for keymode in &skin_ini.keymodes {
        let key_count = keymode.keymode as usize;
        let mut max_receptor_offset = 0;

        let build_fallbacks = |fallback_vec: &[u8], asset: StringPattern| -> Vec<Option<String>> {
            fallback_vec.iter()
                .map(|lane| {
                    if *lane == 0 {
                        None
                    } else {
                        Some(keymode.get_shared(asset.clone(), *lane as usize).to_string())
                    }
                })
                .collect()
        };

        let mut resolver = TextureResolver::new(&mut textures, keymode, Arc::clone(&blank_texture));

        let (mut normal_notes_snap_colored, base_normal_note, norm_note_snap_cols) = resolver.resolve_snap_colored(
            dynamic_assets::Notes::HIT_OBJECT_SHEET,
            "base_snap_n",
            true,
            true,
            |rows, cols, _len| !(rows != 0 && cols != 1),
            |frames, rows, cols, colors| NormalNotesSnapColored::new(frames, None, Some(cols), Some(rows), colors),
            |base_arc| BaseNormalNote::new(base_arc),
        );

        if let Some(ns) = &mut normal_notes_snap_colored {
            ns.colors = norm_note_snap_cols.clone();
        }

        let (mut long_notes_snap_colored, base_long_note, long_head_snap_cols) = resolver.resolve_snap_colored(
            dynamic_assets::Notes::HOLD_OBJECT_SHEET,
            "base_snap_h",
            true,
            true,
            |_rows, _cols, len| len == 9,
            |frames, rows, cols, colors| LongNoteHeadsSnapColored::new(frames, None, Some(cols), Some(rows), colors),
            |base_arc| BaseHoldHead::new(base_arc),
        );

        if let Some(ls) = &mut long_notes_snap_colored {
            ls.colors = long_head_snap_cols.clone();
        }

        let (mut normal_mines_snap_colored, base_normal_mine, norm_mine_snap_cols) = resolver.resolve_snap_colored(
            dynamic_assets::Mines::MINE_SHEET,
            "base_snap_m",
            false,
            false,
            |rows, cols, _len| !(rows != 0 && cols != 1),
            |frames, rows, cols, colors| NormalMinesSnapColored::new(frames, None, Some(cols), Some(rows), colors),
            |base_arc| BaseNormalMine::new(base_arc),
        );

        if let Some(ms) = &mut normal_mines_snap_colored {
            ms.colors = norm_mine_snap_cols.clone();
        }

        let receptor_up_fallbacks = build_fallbacks(&keymode.receptor_fallbacks, dynamic_assets::Receptors::UP);
        let receptor_down_fallbacks = build_fallbacks(&keymode.receptor_fallbacks, dynamic_assets::Receptors::DOWN);
        let normal_notes_fallbacks = build_fallbacks(&keymode.hitobject_fallbacks, dynamic_assets::Notes::HIT_OBJECT);
        let long_note_heads_fallbacks = build_fallbacks(&keymode.holdbody_fallbacks, dynamic_assets::Notes::HOLD_HIT_OBJECT);
        let long_note_bodies_fallbacks = build_fallbacks(&keymode.holdbody_fallbacks, dynamic_assets::Notes::HOLD_BODY);
        let long_note_tails_fallbacks = build_fallbacks(&keymode.holdend_fallbacks, dynamic_assets::Notes::HOLD_END);
        let normal_mines_fallbacks = build_fallbacks(&keymode.holdend_fallbacks, dynamic_assets::Mines::MINE);

        let receptors = remap(keymode.get_receptors());
        let receptors_down = remap(keymode.get_receptors_down());
        let normal_notes = remap(keymode.get_normal_notes());
        let long_note_heads = remap(keymode.get_long_note_heads());
        let long_note_bodies = remap(keymode.get_long_note_bodies());
        let long_note_tails = remap(keymode.get_long_note_tails());
        let normal_mines = remap(keymode.get_normal_mines());

        // TODO: In the future we'd probably want to trim based on "receptor down" since we only check for transparency with a tolererance
        // and if the down texture has a glow for example it will throw off the trimming and we'd end up with unmatching receptors
        let receptor_up_elements: Vec<ReceptorUp> = receptors
            .iter()
            .enumerate()
            .map(|(i, path)| {
                let fallback_path = receptor_up_fallbacks.get(i).and_then(|f| f.as_deref());
                if let Some(texture) = resolver.get_texture_opt(path, fallback_path) {
                    let offset = receptor_processor.process_once(&texture, |arc| {
                        let off = arc.with_image(|img| {
                            dist_from_bottom(&img.to_rgba8(), 0.1)
                        }).try_into().unwrap_or(0);

                        arc.data_mut(|img| {
                            *img = trim_image_vertical(img.clone(), 0.2);
                        });

                        off
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
            .enumerate()
            .map(|(i, path)| {
                let fallback_path = receptor_down_fallbacks.get(i).and_then(|f| f.as_deref());
                if let Some(texture) = resolver.get_texture_opt(path, fallback_path) {
                    let offset = receptor_processor.process_once(&texture, |arc| {
                        let off = arc.with_image(|img| {
                            dist_from_bottom(&img.to_rgba8(), 0.1)
                        }).try_into().unwrap_or(0);

                        arc.data_mut(|img| {
                            *img = trim_image_vertical(img.clone(), 0.2);
                        });

                        off
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
            .enumerate()
            .map(|(i, path)| {
                let fallback_path = normal_notes_fallbacks.get(i).and_then(|f| f.as_deref());
                NormalNote::new(Some(resolver.get_texture(path, fallback_path)))
            })
            .collect();

        let long_note_head_elements: Vec<LongNoteHead> = long_note_heads
            .iter()
            .enumerate()
            .map(|(i, path)| {
                let fallback_path = long_note_heads_fallbacks.get(i).and_then(|f| f.as_deref());
                
                if let Some(texture) = resolver.get_texture_opt(path, fallback_path) {
                    LongNoteHead::new(Some(texture))
                } else {
                    let nn_fallback_path = normal_notes_fallbacks.get(i).and_then(|f| f.as_deref());
                    LongNoteHead::new(Some(resolver.get_texture(&normal_notes[i], nn_fallback_path)))
                }
            })
            .collect();

        let long_note_body_elements: Vec<LongNoteBody> = long_note_bodies
            .iter()
            .enumerate()
            .map(|(i, path)| {
                let fallback_path = long_note_bodies_fallbacks.get(i).and_then(|f| f.as_deref());
                LongNoteBody::new(Some(resolver.get_texture(path, fallback_path)))
            })
            .collect();

        let long_note_tail_elements: Vec<LongNoteTail> = long_note_tails
            .iter()
            .enumerate()
            .map(|(i, path)| {
                let fallback_path = long_note_tails_fallbacks.get(i).and_then(|f| f.as_deref());
                LongNoteTail::new(Some(resolver.get_texture(path, fallback_path)))
            })
            .collect();

        let normal_mine_elements: Vec<NormalMine> = normal_notes
            .iter()
            .enumerate()
            .map(|(i, path)| {
                let fallback_path = normal_mines_fallbacks.get(i).and_then(|f| f.as_deref());
                NormalMine::new(Some(resolver.get_texture(path, fallback_path)))
            })
            .collect();

        let layout = KeymodeLayout {
            keymode: key_count as u8,
            receptor_above_notes: keymode.receptors_over_hit_objects,
            show_judgement_line: false,
            x_offset: 0.5,
            hit_position: -((keymode.column_size as f32 - keymode.hit_pos_offset_y as f32).abs() - max_receptor_offset as f32) / QuaDimensions::Y.as_f32(),
            receptor_offset: (keymode.receptor_pos_offset_y + max_receptor_offset as i32),
            column_widths: vec![keymode.column_size as f32 / QuaDimensions::X.as_f32(); key_count],
            column_spacing: vec![0.0; key_count],
        };

        let fallbacks: Vec<LaneFallback> = (0..key_count)
            .map(|i| {
                let ln_head_path = &long_note_heads[i];
                let ln_head_fallback = long_note_heads_fallbacks.get(i).and_then(|v| v.as_deref());
                let nn_path = &normal_notes[i];
                let nn_fallback = normal_notes_fallbacks.get(i).and_then(|v| v.as_deref());
                let nm_path = &normal_mines[i];
                let nm_fallback = normal_mines_fallbacks.get(i).and_then(|v| v.as_deref());

                LaneFallback {
                    receptor: resolver.resolve_path(&receptors[i], receptor_up_fallbacks.get(i).and_then(|v| v.as_deref())),
                    receptor_down: resolver.resolve_path(&receptors_down[i], receptor_down_fallbacks.get(i).and_then(|v| v.as_deref())),
                    normal_note: resolver.resolve_path(nn_path, nn_fallback),
                    long_note_head: if resolver.get_texture_opt(ln_head_path, ln_head_fallback).is_some() {
                        resolver.resolve_path(ln_head_path, ln_head_fallback)
                    } else {
                        resolver.resolve_path(nn_path, nn_fallback)
                    },
                    long_note_body: resolver.resolve_path(&long_note_bodies[i], long_note_bodies_fallbacks.get(i).and_then(|v| v.as_deref())),
                    long_note_tail: resolver.resolve_path(&long_note_tails[i], long_note_tails_fallbacks.get(i).and_then(|v| v.as_deref())),
                    normal_mine: resolver.resolve_path(&nm_path, nm_fallback),
                }
            })
            .collect();

        let hln = resolver.get_frames(keymode.get_generic(dynamic_assets::Lighting::HIT_LIGHTING, 0), false);
        let hlh = resolver.get_frames(keymode.get_generic(dynamic_assets::Lighting::HOLD_LIGHTING, 0), false);

        keymodes.push(Keymode {
            keymode: key_count as u8,
            layout,
            use_snap_color: keymode.use_hit_object_sheet,
            snap_colors: norm_mine_snap_cols,
            receptor_up: receptor_up_elements,
            receptor_down: receptor_down_elements,
            base_normal_note,
            base_long_note,
            base_normal_mine,
            normal_notes: normal_note_elements,
            long_note_heads: long_note_head_elements,
            long_note_bodies: long_note_body_elements,
            long_note_tails: long_note_tail_elements,
            normal_mines: normal_mine_elements,
            normal_notes_snap_colored,
            long_note_heads_snap_colored: long_notes_snap_colored,
            normal_mines_snap_colored,
            hit_lighting_normal: HitLightingNormal::new(hln.0, Some(keymode.hit_lighting_fps as f32), Some(hln.2), Some(hln.1)),
            hit_lighting_hold: HitLightingHold::new(hlh.0, Some(keymode.hold_lighting_fps as f32), Some(hlh.2), Some(hlh.1)),
            column_lighting: ColumnLighting { texture: Some(Arc::clone(&blank_texture)) },
            judgement_line: JudgementLine { texture: Some(Arc::clone(&blank_texture)), color: Rgba::default() },
            stage: Stage::new(
                 textures.get_shared(&keymode.get_generic(dynamic_assets::Stage::BG_MASK, 0)),
                 textures.get_shared(&keymode.get_generic(dynamic_assets::Stage::RIGHT_BORDER, 0)),
                 textures.get_shared(&keymode.get_generic(dynamic_assets::Stage::LEFT_BORDER, 0))
                ),
            fallbacks,
        });
    }

    let default_keymode = skin.get_keymode(4).unwrap_or(skin.skin_ini.keymodes.first().unwrap());

    let ui = UI {
        cursor: Cursor {
            texture: textures.get_shared(&static_assets::Cursor::MAIN_CURSOR)
            .or_else(|| {
                let bytes = Resources::cursor("qua_cursor.png") // osu!stable cursor sucks
                    .expect("Failed to load cursor texture");
                let tex = Texture::from_bytes(
                    static_assets::Cursor::MAIN_CURSOR.to_string(),
                    &bytes
                ).ok()?;
                Some(Arc::new(RwLock::new(tex)))
            }),
            centered: skin_ini.general.center_cursor,
            rotate: false
        }
    };

    fn get_anchor<T: ConstTypeEnum<Attribute = AssetAttribute>>(pattern: StringPattern) -> Anchor {
        T::find_attribute(&pattern, |a| a.as_anchor().is_some())
            .and_then(AssetAttribute::as_anchor)
            .unwrap_or(Anchor::TopLeft)
    }

    let health_bar_fg = textures.get_shared(&static_assets::HealthBar::FOREGROUND).unwrap_or(blank_texture.clone());
    let health_bar_bg = textures.get_shared(&&static_assets::HealthBar::BACKGROUND).unwrap_or(blank_texture.clone());

    if default_keymode.health_bar_type == HealthBarType::Horizontal
    {
        rotate_90_deg_ccw(&health_bar_fg)?;
        rotate_90_deg_ccw(&health_bar_bg)?;
    }

    let gameplay = Gameplay {
        health_bar: Healthbar::new(
            Some(health_bar_fg),
            Some(health_bar_bg)
        ),
        judgement: Judgement::new(
            textures.get_shared(&static_assets::Judgements::MARV),
            textures.get_shared(&static_assets::Judgements::PERF),
            textures.get_shared(&static_assets::Judgements::GREAT),
            textures.get_shared(&static_assets::Judgements::GOOD),
            textures.get_shared(&static_assets::Judgements::OKAY),
            textures.get_shared(&static_assets::Judgements::MISS),
        ),
        layout: HUDLayout {
            combo: (Vector3::new(0.0, 0.0, 1.0), Alignment { anchor: get_anchor::<static_assets::Numbers>(static_assets::Numbers::COMBO), origin: Origin::TopLeft }),
            rating: (Vector3::new(0.0, 0.0, 1.0), Alignment { anchor: Anchor::TopLeft, origin: Origin::TopLeft }),
            accuracy: (Vector3::new(0.0, 0.0, 1.0), Alignment { anchor: Anchor::TopRight, origin: Origin::TopLeft }),
            score: (Vector3::new(0.0, 0.0, 1.0), Alignment { anchor: Anchor::TopLeft, origin: Origin::TopLeft }),
            judgement: (Vector3::new(0.0, 0.0, 1.0), Alignment { anchor: get_anchor::<static_assets::Judgements>(static_assets::Judgements::MARV), origin: Origin::TopLeft }),
        },
    };

    let sounds = Sounds {
        ui: UISounds {
            menu_back_click: samples.get_shared(&static_assets::Sfx::BACK).get_path(),
            ui_click: samples.get_shared(&static_assets::Sfx::CLICK).get_path(),
            ui_select: samples.get_shared(&static_assets::Sfx::SELECT).get_path(),
            ui_hover: samples.get_shared(&static_assets::Sfx::HOVER).get_path(),
        },
        gameplay: GenericGameplaySounds {
            miss: samples.get_shared(&static_assets::Sfx::COMBO_BREAK).get_path(),
            fail: samples.get_shared(&static_assets::Sfx::FAILURE).get_path(),
            restart: samples.get_shared(&static_assets::Sfx::RETRY).get_path(),
        },
        mania: ManiaGameplaySounds {
            hit: samples.get_shared(&static_assets::Sfx::HIT).get_path(),
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
        samples,
    };

    generic_skin.ensure_textures();

    Ok(generic_skin)
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
        
        qua_km.keymode = keymode.keymode;
        qua_km.receptors_over_hit_objects = keymode.layout.receptor_above_notes;
        qua_km.column_size = (keymode.layout.column_widths.average().unwrap_or(0.0) * QuaDimensions::X.as_f32()).round() as i32;
        qua_km.receptor_pos_offset_y = keymode.layout.receptor_offset;
        qua_km.hit_pos_offset_y = (qua_km.column_size as f32 - (keymode.layout.hit_position * QuaDimensions::Y.as_f32())).abs() as i32;
        qua_km.health_bar_type = HealthBarType::Vertical;

        let q_receptors = qua_km.get_receptors();
        let q_receptors_down = qua_km.get_receptors_down();
        let q_normal_notes = qua_km.get_normal_notes();
        let q_ln_heads = qua_km.get_long_note_heads();
        let q_ln_bodies = qua_km.get_long_note_bodies();
        let q_ln_tails = qua_km.get_long_note_tails();

        let mut body_processor = TextureProcessor::<()>::new();

        for i in 0..(keymode.keymode as usize) {
            {

                let mut tr = StoreRelocator::new(&mut textures);

                if let Some(r) = keymode.receptor_up.get(i) {
                    tr.reloc_arc_lock(&r.texture, StringPattern::from(&q_receptors[i]));
                }
                if let Some(r) = keymode.receptor_down.get(i) {
                    tr.reloc_arc_lock(&r.texture, StringPattern::from(&q_receptors_down[i]));
                }
                if let Some(n) = keymode.normal_notes.get(i) {
                    tr.reloc_arc_lock(&n.texture, StringPattern::from(&q_normal_notes[i]));
                }
                if let Some(n) = keymode.long_note_heads.get(i) {
                    tr.reloc_arc_lock(&n.texture, StringPattern::from(&q_ln_heads[i]));
                }
                if let Some(n) = keymode.long_note_bodies.get(i) {
                    if let Some(texture_arc) = &n.texture
                    {
                            body_processor.process_once_void(texture_arc, |arc_texture| {
                                arc_texture.data_mut(|img| {
                                    let (width, height) = img.dimensions();
                                    let max_res = QuaDimensions::MaxResolution.as_u32();
                                    
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
                    tr.reloc_arc_lock(&n.texture, StringPattern::from(&q_ln_bodies[i]));
                }
                if let Some(n) = keymode.long_note_tails.get(i) {
                    tr.reloc_arc_lock(&n.texture, StringPattern::from(&q_ln_tails[i]));
                }
            }

            fn get_sheet<T: SkinElement>(element: &T, pattern: StringPattern) -> Option<Texture> {
                let sprite_locks = element.as_texture_frames();
                let sprites_vec: Vec<_> = sprite_locks.iter().collect();

                let mut sprites: Vec<&DynamicImage> = sprites_vec
                    .iter()
                    .map(|g| g.state().as_loaded().expect("Sprite not loaded"))
                    .collect();

                if sprites.len() == 1 {
                    let key = pattern.to_string();
                    let sheet = sprites.remove(0).clone();
                    Some(Texture::with_data(key, sheet))
                } else {
                    let rows = element.get_rows().unwrap_or(1);
                    let cols = element.get_columns().unwrap_or(1);
                    let sheet = concat_into_sheet(sprites, rows, cols);

                    let key = format!("{}@{}x{}", pattern.to_string(), rows, cols);

                    sheet.map(|tex| Texture::with_data(key, tex))
                }
            }

            if let Some(tex) = get_sheet(&keymode.hit_lighting_normal, keymode.get_generic(dynamic_assets::Lighting::HIT_LIGHTING, 0)) {
                textures.insert(tex);
            }
        }

        let mut tr = StoreRelocator::new(&mut textures);

        tr.reloc_arc_lock(&keymode.stage.background, qua_km.get_generic(dynamic_assets::Stage::BG_MASK, 0));
        tr.reloc_arc_lock(&keymode.stage.border_right, qua_km.get_generic(dynamic_assets::Stage::RIGHT_BORDER, 0));
        tr.reloc_arc_lock(&keymode.stage.border_left, qua_km.get_generic(dynamic_assets::Stage::LEFT_BORDER, 0));

        qua_keymodes.push(qua_km);
    }

    let mut tr = StoreRelocator::new(&mut textures);
    let mut sr = StoreRelocator::new(&mut samples);

    tr.reloc_arc_lock(&skin.gameplay.health_bar.background, static_assets::HealthBar::BACKGROUND);
    tr.reloc_arc_lock(&skin.gameplay.health_bar.fill, static_assets::HealthBar::FOREGROUND);

    tr.reloc_arc_lock(&skin.gameplay.judgement.flawless, static_assets::Judgements::MARV);
    tr.reloc_arc_lock(&skin.gameplay.judgement.perfect, static_assets::Judgements::PERF);
    tr.reloc_arc_lock(&skin.gameplay.judgement.great, static_assets::Judgements::GREAT);
    tr.reloc_arc_lock(&skin.gameplay.judgement.good, static_assets::Judgements::GOOD);
    tr.reloc_arc_lock(&skin.gameplay.judgement.bad, static_assets::Judgements::OKAY);
    tr.reloc_arc_lock(&skin.gameplay.judgement.miss, static_assets::Judgements::MISS);

    sr.reloc_str(&skin.sounds.ui.menu_back_click, static_assets::Sfx::BACK);
    sr.reloc_str(&skin.sounds.ui.ui_click, static_assets::Sfx::CLICK);
    sr.reloc_str(&skin.sounds.ui.ui_select, static_assets::Sfx::SELECT);
    sr.reloc_str(&skin.sounds.ui.ui_hover, static_assets::Sfx::HOVER);
    sr.reloc_str(&skin.sounds.gameplay.miss, static_assets::Sfx::COMBO_BREAK);
    sr.reloc_str(&skin.sounds.gameplay.fail, static_assets::Sfx::FAILURE);
    sr.reloc_str(&skin.sounds.gameplay.restart, static_assets::Sfx::RETRY);
    sr.reloc_str(&skin.sounds.mania.hit, static_assets::Sfx::HIT);

    skin_ini.keymodes = qua_keymodes;

    cleanup_stores(&skin_ini, Some(&mut textures), Some(&mut samples));

    Ok(QuaSkin::new(
        skin_ini,
        Some(textures),
        Some(samples),
    ))
}
