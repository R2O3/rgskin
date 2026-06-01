use std::sync::{Arc, RwLock};
use image::DynamicImage;
use crate::{
    common::color::Rgba, image_proc::proc::{fill_rect, pad_image_vertical, trim_image_vertical}, io::texture::Texture, process_texture, utils::osu::OsuDimensions
}; 

/// Replicates how the key image is drawn in osu!mania
pub fn to_osu_column_draw(texture: &Arc<RwLock<Texture>>, column_width: u32) -> Result<(), Box<dyn std::error::Error>> {
    process_texture!(texture, |img: DynamicImage| {
        let ns = OsuDimensions::ReceptorScale.as_f32();
        let hds = OsuDimensions::ReceptorScale2x.as_f32();

        let trimmed_img = trim_image_vertical(img, 0.01);

        let is_2x = trimmed_img.height() > OsuDimensions::ReceptorHeight.as_u32();
        
        let multiplier = if is_2x { hds } else { ns};
        let new_width = (column_width as f32 * multiplier) as u32;

        let resized_img = trimmed_img.resize_exact(new_width, trimmed_img.height(), image::imageops::FilterType::Triangle);
        resized_img
    })
}

/// Converts the key image to be displayed having the correct ratios inside osu!mania
/// osu!mania stretches the image so we have to counter-stretch it for it to display as the original
pub fn to_osu_column(texture: &Arc<RwLock<Texture>>, column_width: u32, receptor_offset: u32) -> Result<(), Box<dyn std::error::Error>> {
    process_texture!(texture, |img: DynamicImage| {
        let ns = OsuDimensions::ReceptorScale.as_f32();
        
        let scale_sd = ns;
        let scale_hd = ns / 2.0;

        let trimmed_orig = trim_image_vertical(img, 0.01);
        
        let target_drawn_height = column_width as f32 * (trimmed_orig.height() as f32 / trimmed_orig.width() as f32);
        
        let expected_content_height_sd = (target_drawn_height / scale_sd).round() as u32;
        let expected_content_height_hd = (target_drawn_height / scale_hd).round() as u32;
        
        let expected_final_height_hd = expected_content_height_hd + receptor_offset;
        let is_2x = expected_final_height_hd > OsuDimensions::ReceptorHeight.as_u32();
        
        let (new_height, actual_scale_factor) = if is_2x {
            (expected_content_height_hd, scale_hd)
        } else {
            (expected_content_height_sd, scale_sd)
        };

        let new_width = (column_width as f32 / actual_scale_factor).round() as u32;

        let resized_img = trimmed_orig.resize_exact(new_width, new_height, image::imageops::FilterType::Triangle);
        
        pad_image_vertical(resized_img, 0, receptor_offset)
    })
}

pub fn generate_stage_background(colours: Vec<Rgba>, column_width: u32) -> DynamicImage {
    let width = colours.len() as u32 * column_width;
    let height = OsuDimensions::Y.as_u32();
    let mut canvas = image::RgbaImage::new(width, height);

    for (i, colour) in colours.iter().enumerate() {
        fill_rect(&mut canvas, &colour.to_image_rs(), i as u32 * column_width, 0, column_width, height);
    }

    DynamicImage::ImageRgba8(canvas)
}