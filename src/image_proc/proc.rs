use std::sync::{Arc, RwLock};
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage, imageops::{self, FilterType}};
use crate::{
    common, io::texture::Texture, process_texture, process_texture_mut
}; 

pub fn dist_from_top(img: &RgbaImage, alpha_tolerance: f32) -> u32 {
    let (width, height) = img.dimensions();
    let tolerance = (255.0 * alpha_tolerance).clamp(0.0, 255.0) as u8;
    let raw = img.as_raw();

    for y in 0..height {
        let row_start = (y * width) as usize * 4;
        let row_end = row_start + (width as usize) * 4;
        
        for pixel in raw[row_start..row_end].chunks_exact(4) {
            if pixel[3] > tolerance {
                return y;
            }
        }
    }

    height
}

pub fn dist_from_bottom(img: &RgbaImage, alpha_tolerance: f32) -> u32 {
    let (width, height) = img.dimensions();
    let tolerance = (255.0 * alpha_tolerance).clamp(0.0, 255.0) as u8;
    let raw = img.as_raw();

    for y in (0..height).rev() {
        let row_start = (y * width) as usize * 4;
        let row_end = row_start + (width as usize) * 4;
        
        for pixel in raw[row_start..row_end].chunks_exact(4) {
            if pixel[3] > tolerance {
                return height - 1 - y;
            }
        }
    }

    height
}

pub fn dist_from_left(img: &RgbaImage, alpha_tolerance: f32) -> u32 {
    let (width, height) = img.dimensions();
    let tolerance = (255.0 * alpha_tolerance).clamp(0.0, 255.0) as u8;
    let raw = img.as_raw();

    let mut min_left = width;

    for y in 0..height {
        let row_start = (y * width) as usize * 4;
        
        for x in 0..(min_left as usize) {
            if raw[row_start + x * 4 + 3] > tolerance {
                min_left = x as u32;
                break;
            }
        }
        if min_left == 0 { break; }
    }

    min_left
}

pub fn dist_from_right(img: &RgbaImage, alpha_tolerance: f32) -> u32 {
    let (width, height) = img.dimensions();
    let tolerance = (255.0 * alpha_tolerance).clamp(0.0, 255.0) as u8;
    let raw = img.as_raw();

    let mut max_right = 0;

    for y in 0..height {
        let row_start = (y * width) as usize * 4;
        
        for x in (max_right..width).rev() {
            if raw[row_start + (x as usize) * 4 + 3] > tolerance {
                max_right = x + 1;
                break;
            }
        }
        if max_right == width { break; }
    }
    
    width - max_right
}

pub fn trim_image_vertical(img: DynamicImage, alpha_tolerance: f32) -> DynamicImage {
    let (width, height) = img.dimensions();

    let rgba_img = img.to_rgba8();
    
    let top_trim = dist_from_top(&rgba_img, alpha_tolerance);
    let bottom_dist = dist_from_bottom(&rgba_img, alpha_tolerance);
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

pub fn trim_image_horizontal(img: DynamicImage, alpha_tolerance: f32) -> DynamicImage {
    let (width, height) = img.dimensions();

    let rgba_img = img.to_rgba8();
    
    let left_trim = dist_from_left(&rgba_img, alpha_tolerance);
    let right_dist = dist_from_right(&rgba_img, alpha_tolerance);
    let right_trim = width - right_dist;
    
    if left_trim >= right_trim {
        return img;
    }
    
    let new_width = right_trim - left_trim;
    
    let rgba_img = img.to_rgba8();
    let mut trimmed_img = image::RgbaImage::new(new_width, height);
    
    for y in 0..height {
        for x in 0..new_width {
            let source_x = x + left_trim;
            let pixel = rgba_img.get_pixel(source_x, y);
            trimmed_img.put_pixel(x, y, *pixel);
        }
    }
    
    DynamicImage::ImageRgba8(trimmed_img)
}

pub fn get_trim_bounds(img: &DynamicImage, alpha_tolerance: f32) -> (u32, u32, u32, u32) {
    let rgba = img.to_rgba8();
    (
        dist_from_top(&rgba, alpha_tolerance),
        dist_from_bottom(&rgba, alpha_tolerance),
        dist_from_left(&rgba, alpha_tolerance),
        dist_from_right(&rgba, alpha_tolerance),
    )
}

pub fn trim_image(img: DynamicImage, alpha_tolerance: f32) -> DynamicImage {
    let (width, height) = img.dimensions();

    let (top, bottom, left, right) = get_trim_bounds(&img, alpha_tolerance);
    
    let top_trim = top;
    let bottom_trim = height - bottom;
    let left_trim = left;
    let right_trim = width - right;
    
    if top_trim >= bottom_trim || left_trim >= right_trim {
        return img;
    }
    
    let new_width = right_trim - left_trim;
    let new_height = bottom_trim - top_trim;
    
    img.crop_imm(left_trim, top_trim, new_width, new_height)
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

// stupid I know but the simplest and works
pub fn get_dominant_color(img: &DynamicImage, filter: FilterType) -> Rgba<u8> {
    let resized = img.resize_exact(1, 1, filter);
    resized.get_pixel(0, 0)
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

fn to_linear(c: u8) -> f32 {
    (c as f32 / 255.0).powf(2.2)
}

fn to_srgb(c: f32) -> u8 {
    (c.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8
}

pub fn blend_channel(bg: u8, fg: u8, alpha: f32) -> u8 {
    ((fg as f32 * alpha) + (bg as f32 * (1.0 - alpha))) as u8
}

pub fn blend_alpha(bg: u8, fg: u8, _alpha: f32) -> u8 {
    let bg_alpha = bg as f32 / 255.0;
    let fg_alpha = fg as f32 / 255.0;
    ((fg_alpha + bg_alpha * (1.0 - fg_alpha)) * 255.0) as u8
}

pub fn extract_grayscale_base(
    images: &[&DynamicImage],
    tints: Option<&[common::color::Rgba]>,
    filter: FilterType,
) -> DynamicImage {
    assert!(!images.is_empty());
    if let Some(t) = tints {
        assert_eq!(t.len(), images.len());
    }

    let (w, h) = images[0].dimensions();

    let mut num   = vec![0.0f32; (w * h) as usize];
    let mut denom = 0.0f32;
    let mut alpha = vec![0.0f32; (w * h) as usize];

    for (i, img) in images.iter().enumerate() {
        debug_assert_eq!(img.dimensions(), (w, h));

        let Rgba([tr, tg, tb, _]) = tints
            .map(|t| t[i].to_image_rs())
            .unwrap_or_else(|| get_dominant_color(img, filter));

        let tint = [to_linear(tr), to_linear(tg), to_linear(tb)];
        let t2: f32 = tint.iter().map(|&t| t * t).sum();

        if t2 < 1e-6 {
            continue;
        }

        denom += t2;

        let rgba = img.to_rgba8();
        for (x, y, pixel) in rgba.enumerate_pixels() {
            let Rgba([r, g, b, a]) = *pixel;
            let idx = (y * w + x) as usize;
            num[idx] += to_linear(r) * tint[0]
                      + to_linear(g) * tint[1]
                      + to_linear(b) * tint[2];
            alpha[idx] += a as f32 / 255.0;
        }
    }

    let count = images.len() as f32;

    let buf: RgbaImage = ImageBuffer::from_fn(w, h, |x, y| {
        let idx = (y * w + x) as usize;
        let v = to_srgb(num[idx] / denom);
        let a = ((alpha[idx] / count) * 255.0) as u8;
        Rgba([v, v, v, a])
    });

    DynamicImage::ImageRgba8(buf)
}

pub fn extract_from_sheet(sheet: &DynamicImage, rows: u32, columns: u32) -> Vec<DynamicImage>
{
    let (width, height) = sheet.dimensions();

    let sprite_w = width / columns;
    let sprite_h = height / rows;

    let mut result = Vec::with_capacity((rows * columns) as usize);

    for j in 0..rows {
        for i in 0..columns {
            let x = i * sprite_w;
            let y = j * sprite_h;

            let sprite = sheet.crop_imm(x, y, sprite_w, sprite_h);

            result.push(sprite);
        }
    }

    result
}

pub fn extract_from_sheet_trimmed(sheet: &DynamicImage, rows: u32, columns: u32) -> Vec<DynamicImage>
{
    let (width, height) = sheet.dimensions();

    let sprite_w = width / columns;
    let sprite_h = height / rows;

    let mut result = Vec::with_capacity((rows * columns) as usize);

    for j in 0..rows {
        for i in 0..columns {
            let x = i * sprite_w;
            let y = j * sprite_h;

            let sprite = sheet.crop_imm(x, y, sprite_w, sprite_h);

            result.push(trim_image(sprite, 0.1));
        }
    }

    result
}

pub fn concat_into_sheet(sprites: Vec<&DynamicImage>, rows: u32, columns: u32) -> Option<DynamicImage> {
    if sprites.is_empty() {
        return None
    }

    let (sprite_w, sprite_h) = sprites[0].dimensions();
    let width = sprite_w * columns;
    let height = sprite_h * rows;

    let mut sheet = image::RgbaImage::new(width, height);

    for (index, sprite) in sprites.into_iter().enumerate() {
        let i = (index as u32) % columns;
        let j = (index as u32) / columns;

        let x = i * sprite_w;
        let y = j * sprite_h;

        sheet.copy_from(sprite, x, y).unwrap();
    }

    Some(DynamicImage::ImageRgba8(sheet))
}
