use image::{DynamicImage, Rgba};
use crate::{
    fluxis::{skin_json::Keymode, SkinJson},
    image_proc::proc::overlay_image,
    prelude::TextureStore,
    traits::ManiaSkinConfig,
    BinaryArcExt,
    Store
}; 

pub fn generate_fluxis_preview(
    skin_json: &SkinJson,
    textures: &TextureStore,
    width: u32,
    height: u32
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
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
    
    Ok(DynamicImage::ImageRgba8(canvas))
}

fn draw_background(
    canvas: &mut image::RgbaImage,
    textures: &TextureStore,
    bg_key: &str,
    width: u32,
    height: u32
) {
    let mut use_black_bg = true;
    
    if let Some(bg_texture) = textures.get_shared(bg_key) {
        if let Some(bg_img_dynamic) = bg_texture.get_data() {
            let bg_img = bg_img_dynamic.to_rgba8();
            
            if is_valid_background(&bg_img) {
                use_black_bg = false;
                let scaled_bg = image::imageops::resize(
                    &bg_img, width, height, image::imageops::FilterType::Lanczos3
                );
                
                for y in 0..height {
                    for x in 0..width {
                        canvas.put_pixel(x, y, *scaled_bg.get_pixel(x, y));
                    }
                }
            }
        }
    }
    
    if use_black_bg {
        for pixel in canvas.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 255]);
        }
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
            let receptor_img = img.to_rgba8();
            let aspect_ratio = receptor_img.height() as f32 / receptor_img.width() as f32;
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
            if let Some(receptor_img_dynamic) = receptor_texture.get_data() {
                let receptor_img = receptor_img_dynamic.to_rgba8();
                let aspect_ratio = receptor_img.height() as f32 / receptor_img.width() as f32;
                let target_height = (scaled_column_width as f32 * aspect_ratio) as u32;
                
                let scaled_receptor = image::imageops::resize(
                    &receptor_img, scaled_column_width, target_height,
                    image::imageops::FilterType::Lanczos3
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
    
    let head_texture = textures.get_shared(ln_head_key)
        .and_then(|t| t.get_data())
        .ok_or("Long note head texture not found")?;
    let body_texture = textures.get_shared(ln_body_key)
        .and_then(|t| t.get_data())
        .ok_or("Long note body texture not found")?;
    let tail_texture = textures.get_shared(ln_tail_key)
        .and_then(|t| t.get_data())
        .ok_or("Long note tail texture not found")?;
    
    let head_img = head_texture.to_rgba8();
    let body_img = body_texture.to_rgba8();
    let tail_img = tail_texture.to_rgba8();
    
    let head_aspect = head_img.height() as f32 / head_img.width() as f32;
    let head_height = (scaled_column_width as f32 * head_aspect) as u32;
    let scaled_head = image::imageops::resize(
        &head_img, scaled_column_width, head_height, image::imageops::FilterType::Lanczos3
    );
    
    let tail_aspect = tail_img.height() as f32 / tail_img.width() as f32;
    let tail_height = (scaled_column_width as f32 * tail_aspect) as u32;
    let scaled_tail = image::imageops::resize(
        &tail_img, scaled_column_width, tail_height, image::imageops::FilterType::Lanczos3
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
                let scaled_body = image::imageops::resize(
                    &body_img, scaled_column_width, body_length,
                    image::imageops::FilterType::Lanczos3
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
        if let Some(note_img_dynamic) = note_texture.get_data() {
            let note_img = note_img_dynamic.to_rgba8();
            let aspect_ratio = note_img.height() as f32 / note_img.width() as f32;
            let target_height = (scaled_column_width as f32 * aspect_ratio) as u32;
            
            let scaled_note = image::imageops::resize(
                &note_img, scaled_column_width, target_height,
                image::imageops::FilterType::Lanczos3
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
