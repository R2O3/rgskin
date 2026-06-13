use std::sync::{Arc, RwLock};
use fast_image_resize::{FilterType, ResizeAlg, ResizeOptions, Resizer};
use image::{Rgba, RgbaImage, imageops};
use rayon::prelude::*;
use crate::{
    common, io::texture::Texture, process_texture, process_texture_mut
}; 

pub fn resize_img(
    img: &RgbaImage,
    new_width: u32,
    new_height: u32,
    filter: FilterType,
) -> RgbaImage {
    let mut dst_image = RgbaImage::new(new_width, new_height);

    let mut resizer = Resizer::new();
    let options = ResizeOptions::new().resize_alg(ResizeAlg::Convolution(filter));

    resizer.resize(
        img,
        &mut dst_image,
        Some(&options),
    ).expect("Failed to resize image");

    dst_image
}

pub fn resize_width(
    texture: &Arc<RwLock<Texture>>, 
    new_width: u32,
    filter: FilterType
) -> Result<(), Box<dyn std::error::Error>> {
    process_texture!(texture, |img: RgbaImage| {
        let aspect_ratio = img.height() as f32 / img.width() as f32;
        let new_height = (new_width as f32 * aspect_ratio) as u32;
        resize_img(&img, new_width, new_height, filter)
    })
}

pub fn resize_height(
    texture: &Arc<RwLock<Texture>>, 
    new_height: u32,
    filter: FilterType
) -> Result<(), Box<dyn std::error::Error>> {
    process_texture!(texture, |img: RgbaImage| {
        let aspect_ratio = img.width() as f32 / img.height() as f32;
        let new_width = (new_height as f32 * aspect_ratio) as u32;
        resize_img(&img, new_width, new_height, filter)
    })
}

pub fn dist_from_top(img: &RgbaImage, alpha_tolerance: f32) -> u32 {
    let (width, height) = img.dimensions();
    let tolerance = (255.0 * alpha_tolerance).clamp(0.0, 255.0) as u8;
    let raw = img.as_raw();

    let chunk_size = 32;
    let first_rows: Vec<Option<u32>> = (0..height)
        .collect::<Vec<_>>()
        .par_chunks(chunk_size)
        .map(|chunk| {
            for &y in chunk {
                let row_start = (y * width) as usize * 4;
                let row_end = row_start + (width as usize) * 4;
                for pixel in raw[row_start..row_end].chunks_exact(4) {
                    if pixel[3] > tolerance {
                        return Some(y);
                    }
                }
            }
            None
        })
        .collect();

    first_rows.into_iter().flatten().min().unwrap_or(height)
}

pub fn dist_from_bottom(img: &RgbaImage, alpha_tolerance: f32) -> u32 {
    let (width, height) = img.dimensions();
    let tolerance = (255.0 * alpha_tolerance).clamp(0.0, 255.0) as u8;
    let raw = img.as_raw();

    let chunk_size = 32;
    let last_rows: Vec<Option<u32>> = (0..height)
        .collect::<Vec<_>>()
        .par_chunks(chunk_size)
        .map(|chunk| {
            for &y in chunk.iter().rev() {
                let row_start = (y * width) as usize * 4;
                let row_end = row_start + (width as usize) * 4;
                for pixel in raw[row_start..row_end].chunks_exact(4) {
                    if pixel[3] > tolerance {
                        return Some(y);
                    }
                }
            }
            None
        })
        .collect();

    let max_row = last_rows.into_iter().flatten().max().unwrap_or(0);
    height - 1 - max_row
}

pub fn dist_from_left(img: &RgbaImage, alpha_tolerance: f32) -> u32 {
    let (width, height) = img.dimensions();
    let tolerance = (255.0 * alpha_tolerance).clamp(0.0, 255.0) as u8;
    let raw = img.as_raw();

    let leftmost: u32 = (0..height)
        .into_par_iter()
        .map(|y| {
            let row_start = (y * width) as usize * 4;
            for x in 0..width {
                if raw[row_start + (x as usize) * 4 + 3] > tolerance {
                    return x;
                }
            }
            width
        })
        .min()
        .unwrap_or(width);
    leftmost
}

pub fn dist_from_right(img: &RgbaImage, alpha_tolerance: f32) -> u32 {
    let (width, height) = img.dimensions();
    let tolerance = (255.0 * alpha_tolerance).clamp(0.0, 255.0) as u8;
    let raw = img.as_raw();

    let rightmost: u32 = (0..height)
        .into_par_iter()
        .map(|y| {
            let row_start = (y * width) as usize * 4;
            for x in (0..width).rev() {
                if raw[row_start + (x as usize) * 4 + 3] > tolerance {
                    return x + 1;
                }
            }
            0
        })
        .max()
        .unwrap_or(0);
    width - rightmost
}

pub fn trim_image_vertical(img: &RgbaImage, alpha_tolerance: f32) -> RgbaImage {
    let (width, height) = img.dimensions();
    
    let top_trim = dist_from_top(img, alpha_tolerance);
    let bottom_dist = dist_from_bottom(img, alpha_tolerance);
    let bottom_trim = height - bottom_dist;
    
    if top_trim >= bottom_trim {
        return img.clone();
    }
    
    let new_height = bottom_trim - top_trim;
    let mut trimmed_img = RgbaImage::new(width, new_height);
    
    trimmed_img
        .par_chunks_mut((width * 4) as usize)
        .enumerate()
        .for_each(|(y, row_out)| {
            let source_y = y as u32 + top_trim;
            let source_row = &img.as_raw()[(source_y * width) as usize * 4..][..row_out.len()];
            row_out.copy_from_slice(source_row);
        });

    trimmed_img
}

pub fn trim_image_horizontal(img: &RgbaImage, alpha_tolerance: f32) -> RgbaImage {
    let (width, height) = img.dimensions();
    
    let left_trim = dist_from_left(img, alpha_tolerance);
    let right_dist = dist_from_right(img, alpha_tolerance);
    let right_trim = width - right_dist;
    
    if left_trim >= right_trim {
        return img.clone();
    }
    
    let new_width = right_trim - left_trim;
    let mut trimmed_img = RgbaImage::new(new_width, height);
    
    trimmed_img
        .par_chunks_mut((new_width * 4) as usize)
        .enumerate()
        .for_each(|(y, row_out)| {
            let source_start = ((y as u32 * width) + left_trim) as usize * 4;
            let source_row = &img.as_raw()[source_start..][..row_out.len()];
            row_out.copy_from_slice(source_row);
        });

    trimmed_img
}

pub fn get_trim_bounds(img: &RgbaImage, alpha_tolerance: f32) -> (u32, u32, u32, u32) {
    (
        dist_from_top(img, alpha_tolerance),
        dist_from_bottom(img, alpha_tolerance),
        dist_from_left(img, alpha_tolerance),
        dist_from_right(img, alpha_tolerance),
    )
}

pub fn trim_image(img: &RgbaImage, alpha_tolerance: f32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let (top, bottom, left, right) = get_trim_bounds(img, alpha_tolerance);
    
    let top_trim = top;
    let bottom_trim = height - bottom;
    let left_trim = left;
    let right_trim = width - right;
    
    if top_trim >= bottom_trim || left_trim >= right_trim {
        return img.clone();
    }
    
    let new_width = right_trim - left_trim;
    let new_height = bottom_trim - top_trim;
    
    imageops::crop_imm(img, left_trim, top_trim, new_width, new_height).to_image()
}

pub fn pad_image_vertical(img: &RgbaImage, top_pad: u32, bottom_pad: u32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let new_height = height + top_pad + bottom_pad;

    let mut padded_img = RgbaImage::new(width, new_height);

    padded_img
        .par_chunks_mut(4)
        .for_each(|pixel| pixel.fill(0));

    padded_img
        .par_chunks_mut((width * 4) as usize)
        .enumerate()
        .for_each(|(y, row_out)| {
            let src_y = y as u32 - top_pad;
            if src_y < height {
                let source_row = &img.as_raw()[(src_y * width) as usize * 4..][..row_out.len()];
                row_out.copy_from_slice(source_row);
            }
        });

    padded_img
}

pub fn flip_vertical(texture: &Arc<RwLock<Texture>>) {
    process_texture_mut!(texture, |img: &mut RgbaImage| {
        imageops::flip_vertical_in_place(img);
    })
}

pub fn rotate_90_deg_cw(texture: &Arc<RwLock<Texture>>) -> Result<(), Box<dyn std::error::Error>> {
    process_texture!(texture, |img: RgbaImage| {
        imageops::rotate90(&img)
    })
}

pub fn rotate_90_deg_ccw(texture: &Arc<RwLock<Texture>>) -> Result<(), Box<dyn std::error::Error>> {
    process_texture!(texture, |img: RgbaImage| {
        imageops::rotate270(&img)
    })
}

pub fn fill_rect(
    base: &mut RgbaImage,
    color: &Rgba<u8>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) {
    let img_width = base.width() as usize;
    let img_height = base.height() as usize;
    let x = x as usize;
    let y = y as usize;
    let width = width as usize;
    let height = height as usize;

    let x_end = (x + width).min(img_width);
    let y_end = (y + height).min(img_height);
    if x >= x_end || y >= y_end {
        return;
    }

    let raw = base.as_mut();
    raw.par_chunks_mut(img_width * 4)
        .enumerate()
        .for_each(|(py, row)| {
            if py < y || py >= y_end {
                return;
            }
            for px in x..x_end {
                let offset = px * 4;
                row[offset..offset + 4].copy_from_slice(&color.0);
            }
        });
}

pub fn get_dominant_color(img: &RgbaImage, filter: FilterType) -> Rgba<u8> {
    let resized = resize_img(img, 1, 1, filter);
    *resized.get_pixel(0, 0)
}

pub fn overlay_image(
    base: &mut RgbaImage,
    overlay: &RgbaImage,
    x: u32,
    y: u32,
) {
    let (ow, oh) = overlay.dimensions();
    let (bw, bh) = (base.width(), base.height());

    let x_start = x.max(0) as usize;
    let y_start = y.max(0) as usize;
    let x_end = (x + ow).min(bw) as usize;
    let y_end = (y + oh).min(bh) as usize;
    if x_start >= x_end || y_start >= y_end {
        return;
    }

    let raw = base.as_mut();
    let img_width = bw as usize;
    let overlay_width = ow as usize;
    let overlay_raw = overlay.as_raw();

    raw.par_chunks_mut(img_width * 4)
        .enumerate()
        .for_each(|(py, row)| {
            if py < y_start || py >= y_end {
                return;
            }
            let src_row = py - y as usize;
            let overlay_row_start = src_row * overlay_width * 4;
            for px in x_start..x_end {
                let src_col = px - x as usize;
                let overlay_offset = overlay_row_start + src_col * 4;
                let alpha = overlay_raw[overlay_offset + 3] as f32 / 255.0;
                if alpha > 0.0 {
                    let base_offset = px * 4;
                    let dst = &row[base_offset..base_offset + 4];
                    let src = &overlay_raw[overlay_offset..overlay_offset + 4];
                    let blended = Rgba([
                        blend_channel(dst[0], src[0], alpha),
                        blend_channel(dst[1], src[1], alpha),
                        blend_channel(dst[2], src[2], alpha),
                        blend_alpha(dst[3], src[3], alpha),
                    ]);
                    row[base_offset..base_offset + 4].copy_from_slice(&blended.0);
                }
            }
        });
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
    images: &[&RgbaImage],
    tints: Option<&[common::color::Rgba]>,
    filter: FilterType,
) -> RgbaImage {
    use rayon::prelude::*;

    assert!(!images.is_empty());
    if let Some(t) = tints {
        assert_eq!(t.len(), images.len());
    }

    let (w, h) = images[0].dimensions();
    let n = (w * h) as usize;
    let count = images.len() as f32;

    let mut acc = vec![0.0f32; n * 2];
    let mut denom = 0.0f32;

    for (i, img) in images.iter().enumerate() {
        let Rgba([tr, tg, tb, _]) = tints
            .map(|t| t[i].to_image_rs())
            .unwrap_or_else(|| get_dominant_color(img, filter));

        let tint = [to_linear(tr), to_linear(tg), to_linear(tb)];
        let t2: f32 = tint[0] * tint[0] + tint[1] * tint[1] + tint[2] * tint[2];

        if t2 < 1e-6 {
            continue;
        }
        denom += t2;

        let raw = img.as_raw();

        acc.par_chunks_mut(2)
            .zip(raw.par_chunks_exact(4))
            .for_each(|(na, px)| {
                na[0] += to_linear(px[0]) * tint[0]
                    + to_linear(px[1]) * tint[1]
                    + to_linear(px[2]) * tint[2];
                na[1] += px[3] as f32 / 255.0;
            });
    }

    let mut raw_out = vec![0u8; n * 4];
    raw_out
        .par_chunks_mut(4)
        .zip(acc.par_chunks(2))
        .for_each(|(out, na)| {
            let v = to_srgb(na[0] / denom);
            let a = ((na[1] / count) * 255.0) as u8;
            out[0] = v;
            out[1] = v;
            out[2] = v;
            out[3] = a;
        });

    RgbaImage::from_raw(w, h, raw_out).unwrap()
}

pub fn extract_from_sheet(sheet: &RgbaImage, rows: u32, columns: u32) -> Vec<RgbaImage> {
    let (width, height) = sheet.dimensions();
    let sprite_w = width / columns;
    let sprite_h = height / rows;

    (0..(rows * columns))
        .into_par_iter()
        .map(|idx| {
            let i = idx % columns;
            let j = idx / columns;
            let x = i * sprite_w;
            let y = j * sprite_h;
            imageops::crop_imm(sheet, x, y, sprite_w, sprite_h).to_image()
        })
        .collect()
}

pub fn extract_from_sheet_trimmed(sheet: &RgbaImage, rows: u32, columns: u32) -> Vec<RgbaImage> {
    let (width, height) = sheet.dimensions();
    let sprite_w = width / columns;
    let sprite_h = height / rows;

    (0..(rows * columns))
        .into_par_iter()
        .map(|idx| {
            let i = idx % columns;
            let j = idx / columns;
            let x = i * sprite_w;
            let y = j * sprite_h;
            let sprite = imageops::crop_imm(sheet, x, y, sprite_w, sprite_h).to_image();
            trim_image(&sprite, 0.1)
        })
        .collect()
}

pub fn concat_into_sheet(sprites: &[&RgbaImage], rows: u32, columns: u32) -> Option<RgbaImage> {
    if sprites.is_empty() {
        return None;
    }

    let (sprite_w, sprite_h) = sprites[0].dimensions();
    let width = sprite_w * columns;
    let height = sprite_h * rows;

    let sprite_raws: Vec<&[u8]> = sprites.iter().map(|s| &s.as_raw()[..]).collect();

    let mut sheet_buffer = vec![0u8; (width * height) as usize * 4];

    sheet_buffer.par_chunks_mut((width * 4) as usize)
        .enumerate()
        .for_each(|(py, row_out)| {
            let py = py as u32;
            let sprite_row = py / sprite_h;
            let local_y = py % sprite_h;
            if sprite_row >= rows {
                return;
            }
            for col in 0..columns {
                let sprite_idx = (sprite_row * columns + col) as usize;
                if sprite_idx >= sprite_raws.len() {
                    continue;
                }
                let src_start = (local_y * sprite_w) as usize * 4;
                let dest_offset = (col * sprite_w) as usize * 4;
                let src_slice = &sprite_raws[sprite_idx][src_start..src_start + (sprite_w as usize) * 4];
                let dest_slice = &mut row_out[dest_offset..dest_offset + (sprite_w as usize) * 4];
                dest_slice.copy_from_slice(src_slice);
            }
        });

    RgbaImage::from_raw(width, height, sheet_buffer)
}
