use std::sync::{Arc, RwLock};
use image::{imageops, DynamicImage, GenericImageView, Rgba};
use crate::{
    io::texture::Texture,
    process_texture, process_texture_mut,
}; 

pub fn dist_from_bottom(img: &DynamicImage, alpha_tolerance: f32) -> u32 {
    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = rgba_img.get_pixel(x, y);
            let tolerance_u8 = (255.0 * alpha_tolerance).clamp(0.0, 255.0) as u8;
            if pixel[3] > tolerance_u8 {
                return height - 1 - y;
            }
        }
    }
    
    height
}

pub fn dist_from_top(img: &DynamicImage, alpha_tolerance: f32) -> u32 {
    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    
    for y in 0..height {
        for x in 0..width {
            let pixel = rgba_img.get_pixel(x, y);
            let tolerance_u8 = (255.0 * alpha_tolerance).clamp(0.0, 255.0) as u8;
            if pixel[3] > tolerance_u8 {
                return y;
            }
        }
    }
    
    height
}

pub fn trim_image_vertical(img: DynamicImage, alpha_tolerance: f32) -> DynamicImage {
    let (width, height) = img.dimensions();
    
    let top_trim = dist_from_top(&img, alpha_tolerance);
    let bottom_dist = dist_from_bottom(&img, alpha_tolerance);
    let bottom_trim = height - bottom_dist;
    
    if top_trim >= bottom_trim {
        return img;
    }
    
    let new_height = bottom_trim - top_trim;
    
    let rgba_img = img.to_rgba8();
    let mut trimmed_img = image::RgbaImage::new(width, new_height);
    
    for y in 0..new_height {
        for x in 0..width {
            let source_y = y + top_trim;
            let pixel = rgba_img.get_pixel(x, source_y);
            trimmed_img.put_pixel(x, y, *pixel);
        }
    }
    
    DynamicImage::ImageRgba8(trimmed_img)
}

pub fn pad_image_vertical(img: DynamicImage, top_pad: u32, bottom_pad: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let new_height = height + top_pad + bottom_pad;
    
    let rgba_img = img.to_rgba8();
    let mut padded_img = image::RgbaImage::new(width, new_height);
    
    for y in 0..new_height {
        for x in 0..width {
            padded_img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }
    
    for y in 0..height {
        for x in 0..width {
            let pixel = rgba_img.get_pixel(x, y);
            padded_img.put_pixel(x, y + top_pad, *pixel);
        }
    }
    
    DynamicImage::ImageRgba8(padded_img)
}

pub fn flip_vertical(texture: &Arc<RwLock<Texture>>) {
    process_texture_mut!(texture, |img: &mut DynamicImage| {
        imageops::flip_vertical_in_place(img);
    })
}

pub fn rotate_90_deg_cw(texture: &Arc<RwLock<Texture>>) -> Result<(), Box<dyn std::error::Error>> {
    process_texture!(texture, |img: DynamicImage| {
        let rotated_buffer = imageops::rotate90(&img);
        DynamicImage::ImageRgba8(rotated_buffer)
    })
}

pub fn rotate_90_deg_ccw(texture: &Arc<RwLock<Texture>>) -> Result<(), Box<dyn std::error::Error>> {
    process_texture!(texture, |img: DynamicImage| {
        let rotated_buffer = imageops::rotate270(&img);
        DynamicImage::ImageRgba8(rotated_buffer)
    })
}

pub fn resize_width(
    texture: &Arc<RwLock<Texture>>, 
    new_width: u32,
    filter: image::imageops::FilterType
) -> Result<(), Box<dyn std::error::Error>> {
    process_texture!(texture, |img: DynamicImage| {
        let aspect_ratio = img.height() as f32 / img.width() as f32;
        let new_height = (new_width as f32 * aspect_ratio) as u32;
        img.resize_exact(new_width, new_height, filter)
    })
}

pub fn resize_height(
    texture: &Arc<RwLock<Texture>>, 
    new_height: u32,
    filter: image::imageops::FilterType
) -> Result<(), Box<dyn std::error::Error>> {
    process_texture!(texture, |img: DynamicImage| {
        let aspect_ratio = img.width() as f32 / img.height() as f32;
        let new_width = (new_height as f32 * aspect_ratio) as u32;
        img.resize_exact(new_width, new_height, filter)
    })
}

pub fn fill_rect(
    base: &mut image::RgbaImage,
    color: &image::Rgba<u8>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) {
    let img_width = base.width();
    let img_height = base.height();
    
    let x_end = (x + width).min(img_width);
    let y_end = (y + height).min(img_height);
    
    for py in y..y_end {
        for px in x..x_end {
            base.put_pixel(px, py, *color);
        }
    }
}

pub fn overlay_image(
    base: &mut image::RgbaImage,
    overlay: &image::RgbaImage,
    x: u32,
    y: u32
) {
    for oy in 0..overlay.height() {
        for ox in 0..overlay.width() {
            let px = x + ox;
            let py = y + oy;
            
            if px < base.width() && py < base.height() {
                let src_pixel = overlay.get_pixel(ox, oy);
                let alpha = src_pixel[3] as f32 / 255.0;
                
                if alpha > 0.0 {
                    let dst_pixel = base.get_pixel(px, py);
                    let blended = image::Rgba([
                        blend_channel(dst_pixel[0], src_pixel[0], alpha),
                        blend_channel(dst_pixel[1], src_pixel[1], alpha),
                        blend_channel(dst_pixel[2], src_pixel[2], alpha),
                        blend_alpha(dst_pixel[3], src_pixel[3], alpha),
                    ]);
                    base.put_pixel(px, py, blended);
                }
            }
        }
    }
}

pub fn blend_channel(bg: u8, fg: u8, alpha: f32) -> u8 {
    ((fg as f32 * alpha) + (bg as f32 * (1.0 - alpha))) as u8
}

pub fn blend_alpha(bg: u8, fg: u8, _alpha: f32) -> u8 {
    let bg_alpha = bg as f32 / 255.0;
    let fg_alpha = fg as f32 / 255.0;
    ((fg_alpha + bg_alpha * (1.0 - fg_alpha)) * 255.0) as u8
}