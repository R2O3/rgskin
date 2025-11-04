use std::sync::{Arc, RwLock};
use image::{DynamicImage, GenericImageView, Rgba};
use crate::{io::Texture, process_texture, utils::osu::OsuDimensions}; 

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

pub fn to_osu_column_draw(texture: &Arc<RwLock<Texture>>, column_width: u32) -> Result<(), Box<dyn std::error::Error>> {
    process_texture!(texture, |img: DynamicImage| {
        let scale_factor = column_width as f32 / OsuDimensions::ReceptorWidth.as_f32();
        let current_width = img.width();
        let new_width = (current_width as f32 * scale_factor) as u32;

        let resized_img = img.resize_exact(new_width, img.height(), image::imageops::FilterType::Triangle);

        trim_image_vertical(resized_img, 0.01)
    })
}

pub fn to_osu_column(texture: &Arc<RwLock<Texture>>, column_width: u32, receptor_offset: u32) -> Result<(), Box<dyn std::error::Error>> {
    process_texture!(texture, |img: DynamicImage| {
        let scale_factor = OsuDimensions::ReceptorWidth.as_f32() / column_width as f32;
        let current_width = img.width();
        let new_width = (current_width as f32 * scale_factor) as u32;
        let new_height = (img.height() as f32 * scale_factor) as u32;

        let resized_img = img.resize_exact(new_width, new_height, image::imageops::FilterType::Triangle);
        let trimmed_img = trim_image_vertical(resized_img, 0.01);
        
        pad_image_vertical(trimmed_img, 0, receptor_offset)
    })
}