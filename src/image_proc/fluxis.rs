use fast_image_resize::FilterType;
use image::{Rgba, RgbaImage};
use rayon::prelude::*;
use crate::{
    BinaryArcExt, Store, fluxis::{SkinJson, skin_json::Keymode}, image_proc::proc::{fill_rect, overlay_image, resize_img}, prelude::TextureStore, traits::ManiaSkinConfig
}; 

pub fn generate_fluxis_preview(
    skin_json: &SkinJson,
    textures: &TextureStore,
    width: u32,
    height: u32
) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let mut canvas = image::RgbaImage::new(width, height);
    
    draw_background(&mut canvas, textures, &skin_json.overrides.stage.background, width, height);
    
    let keymode_4k = skin_json.get_keymode(4)
        .ok_or("4k not found in skin json")?;
    
    let scaled_column_width = calculate_scaled_column_width(keymode_4k.column_width, width);
    let stage_x_offset = calculate_stage_offset(scaled_column_width, width);
    let max_receptor_height = calculate_max_receptor_height(keymode_4k, textures, scaled_column_width);
    let hit_y = calculate_hit_position(height, max_receptor_height);
    
    draw_receptors(&mut canvas, keymode_4k, textures, stage_x_offset, scaled_column_width, hit_y)?;
    
    let note_spacing = (height as f32 * 0.3) as u32;
    let note_gap = 20u32;
    
    draw_notes(&mut canvas, keymode_4k, textures, stage_x_offset, scaled_column_width, hit_y + (max_receptor_height as f32 / 1.5) as u32, 
                note_spacing, note_gap, height)?;
    
    Ok(canvas)
}

fn draw_background(
    canvas: &mut image::RgbaImage,
    textures: &TextureStore,
    bg_key: &str,
    width: u32,
    height: u32,
) {
    let mut use_black_bg = true;

    if let Some(bg_texture) = textures.get_shared(bg_key) {
        if let Some(bg_img) = bg_texture.get_data() {
            if is_valid_background(&bg_img) {
                use_black_bg = false;
                let scaled_bg = resize_img(
                    &bg_img, width, height, FilterType::Hamming,
                );
                canvas
                    .par_chunks_mut((width * 4) as usize)
                    .enumerate()
                    .for_each(|(y, row)| {
                        let src_row_start = (y as u32 * width) as usize * 4;
                        let src_row = &scaled_bg.as_raw()[src_row_start..src_row_start + row.len()];
                        row.copy_from_slice(src_row);
                    });
            }
        }
    }

    if use_black_bg {
        fill_rect(canvas, &Rgba([0, 0, 0, 255]), 0, 0, width, height);
    }
}

fn is_valid_background(img: &image::RgbaImage) -> bool {
    if img.width() < 2 || img.height() < 2 {
        return false;
    }
    
    img.pixels().any(|p| p[3] > 0)
}

fn calculate_scaled_column_width(column_width: u32, canvas_width: u32) -> u32 {
    let total_stage_width = column_width * 4;
    let target_stage_width = (canvas_width as f32 * 0.85) as u32;
    let scale_factor = target_stage_width as f32 / total_stage_width as f32;
    (column_width as f32 * scale_factor) as u32
}

fn calculate_stage_offset(scaled_column_width: u32, canvas_width: u32) -> u32 {
    let scaled_total_width = scaled_column_width * 4;
    (canvas_width.saturating_sub(scaled_total_width)) / 2
}

fn calculate_max_receptor_height(
    keymode: &Keymode,
    textures: &TextureStore,
    scaled_column_width: u32
) -> u32 {
    (0..4)
        .filter_map(|col| keymode.receptor_images.get(col))
        .filter_map(|key| textures.get_shared(key))
        .filter_map(|texture| texture.get_data())
        .map(|img| {
            let aspect_ratio = img.height() as f32 / img.width() as f32;
            (scaled_column_width as f32 * aspect_ratio) as u32
        })
        .max()
        .unwrap_or(0)
}

fn calculate_hit_position(canvas_height: u32, max_receptor_height: u32) -> u32 {
    let bottom_margin = (canvas_height as f32 * 0.05) as u32;
    canvas_height.saturating_sub(max_receptor_height + bottom_margin)
}

fn draw_receptors(
    canvas: &mut image::RgbaImage,
    keymode: &Keymode,
    textures: &TextureStore,
    stage_x_offset: u32,
    scaled_column_width: u32,
    hit_y: u32
) -> Result<(), Box<dyn std::error::Error>> {
    for col in 0..4 {
        let x = stage_x_offset + (col * scaled_column_width);
        let receptor_key = keymode.receptor_images.get(col as usize)
            .ok_or(format!("Receptor image not found for column {}", col))?;
        
        if let Some(receptor_texture) = textures.get_shared(receptor_key) {
            if let Some(receptor_img) = receptor_texture.get_data() {
                let aspect_ratio = receptor_img.height() as f32 / receptor_img.width() as f32;
                let target_height = (scaled_column_width as f32 * aspect_ratio) as u32;
                
                let scaled_receptor = resize_img(
                    &receptor_img, scaled_column_width, target_height,
                    FilterType::Hamming
                );
                
                overlay_image(canvas, &scaled_receptor, x, hit_y);
            }
        }
    }
    
    Ok(())
}

fn draw_notes(
    canvas: &mut image::RgbaImage,
    keymode: &Keymode,
    textures: &TextureStore,
    stage_x_offset: u32,
    scaled_column_width: u32,
    hit_y: u32,
    note_spacing: u32,
    note_gap: u32,
    canvas_height: u32
) -> Result<(), Box<dyn std::error::Error>> {
    for col in 0..4 {
        let x = stage_x_offset + (col * scaled_column_width);
        
        if col == 2 {
            draw_long_note(canvas, keymode, textures, x, col, scaled_column_width, 
                          hit_y, note_spacing, note_gap)?;
        } else {
            draw_normal_notes(canvas, keymode, textures, x, col, scaled_column_width,
                            hit_y, note_spacing, note_gap, canvas_height)?;
        }
    }
    
    Ok(())
}

fn draw_long_note(
    canvas: &mut image::RgbaImage,
    keymode: &Keymode,
    textures: &TextureStore,
    x: u32,
    col: u32,
    scaled_column_width: u32,
    hit_y: u32,
    note_spacing: u32,
    note_gap: u32
) -> Result<(), Box<dyn std::error::Error>> {    
    let ln_head_key = keymode.long_note_head_images.get(col as usize)
        .ok_or(format!("Long note head image not found for column {}", col))?;
    let ln_body_key = keymode.long_note_body_images.get(col as usize)
        .ok_or(format!("Long note body image not found for column {}", col))?;
    let ln_tail_key = keymode.long_note_tail_images.get(col as usize)
        .ok_or(format!("Long note tail image not found for column {}", col))?;
    
    let head_img = textures.get_shared(ln_head_key)
        .and_then(|t| t.get_data())
        .ok_or("Long note head texture not found")?;
    let body_img = textures.get_shared(ln_body_key)
        .and_then(|t| t.get_data())
        .ok_or("Long note body texture not found")?;
    let tail_img = textures.get_shared(ln_tail_key)
        .and_then(|t| t.get_data())
        .ok_or("Long note tail texture not found")?;
    
    let head_aspect = head_img.height() as f32 / head_img.width() as f32;
    let head_height = (scaled_column_width as f32 * head_aspect) as u32;
    let scaled_head = resize_img(
        &head_img, scaled_column_width, head_height, FilterType::Hamming
    );
    
    let tail_aspect = tail_img.height() as f32 / tail_img.width() as f32;
    let tail_height = (scaled_column_width as f32 * tail_aspect) as u32;
    let scaled_tail = resize_img(
        &tail_img, scaled_column_width, tail_height, FilterType::Hamming
    );
    
    let staircase_offset = col * (note_spacing / 8);
    let ln_total_length = note_spacing + (canvas.height() as f32 * 0.1) as u32;
    
    if let Some(head_y) = hit_y.checked_sub(head_height + note_gap + staircase_offset) {
        if let Some(tail_y) = head_y.checked_sub(ln_total_length) {
            overlay_image(canvas, &scaled_tail, x, tail_y);
            
            let body_start_y = head_y + (head_height / 2);
            let body_end_y = tail_y + tail_height;
            
            if body_start_y > body_end_y {
                let body_length = body_start_y - body_end_y;
                let scaled_body = resize_img(
                    &body_img, scaled_column_width, body_length,
                    FilterType::Hamming
                );
                overlay_image(canvas, &scaled_body, x, body_end_y);
            }
            
            overlay_image(canvas, &scaled_head, x, head_y);
        }
    }
    
    Ok(())
}

fn draw_normal_notes(
    canvas: &mut image::RgbaImage,
    keymode: &Keymode,
    textures: &TextureStore,
    x: u32,
    col: u32,
    scaled_column_width: u32,
    hit_y: u32,
    note_spacing: u32,
    note_gap: u32,
    canvas_height: u32
) -> Result<(), Box<dyn std::error::Error>> {
    let note_key = keymode.normal_note_images.get(col as usize)
        .ok_or(format!("Note image not found for column {}", col))?;
    
    if let Some(note_texture) = textures.get_shared(note_key) {
        if let Some(note_img) = note_texture.get_data() {
            let aspect_ratio = note_img.height() as f32 / note_img.width() as f32;
            let target_height = (scaled_column_width as f32 * aspect_ratio) as u32;
            
            let scaled_note = resize_img(
                &note_img, scaled_column_width, target_height,
                FilterType::Hamming
            );
            
            for i in 0..2 {
                let staircase_offset = col * (note_spacing / 8);
                
                if let Some(note_y) = hit_y.checked_sub(
                    target_height + note_gap + i * note_spacing + staircase_offset
                ) {
                    if note_y < canvas_height {
                        overlay_image(canvas, &scaled_note, x, note_y);
                    }
                }
            }
        }
    }
    
    Ok(())
}
