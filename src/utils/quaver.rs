use std::sync::{Arc, RwLock};

use image::{DynamicImage, imageops::FilterType};

use crate::{Binary, BinaryArcExt, BinaryState, Store, StringPattern, TextureArcExt, common::color::Rgba, image_proc::proc::{extract_from_sheet, extract_from_sheet_trimmed, extract_grayscale_base, get_dominant_color}, numeric_enum, quaver, texture::Texture, traits::KeymodeInvariant};

numeric_enum! {
    pub enum QuaDimensions: u32 {
        X = 615,
        Y = 346,
        MaxResolution = 4096, // TODO: change this later
    }
}

pub struct TextureResolver<'a, S: Store<Texture>> {
    textures: &'a mut S,
    keymode: &'a quaver::Keymode,
    blank_texture: Arc<RwLock<Texture>>,
}

impl<'a, S: Store<Texture>> TextureResolver<'a, S> {
    pub fn new(textures: &'a mut S, keymode: &'a quaver::Keymode, blank_texture: Arc<RwLock<Texture>>) -> Self {
        Self { textures, keymode, blank_texture }
    }

    pub fn resolve_path(&self, path: &str, fallback_path: Option<&str>) -> String {
        match fallback_path {
            Some(fallback) if !self.textures.contains(path) && self.keymode.use_fallback => fallback.to_string(),
            _ => path.to_string(),
        }
    }

    pub fn get_texture(&self, path: &str, fallback_path: Option<&str>) -> Arc<RwLock<Texture>> {
        let tex_path = self.resolve_path(path, fallback_path);
        self.textures.get_shared(&tex_path).unwrap_or_else(|| Arc::clone(&self.blank_texture))
    }

    pub fn get_texture_opt(&self, path: &str, fallback_path: Option<&str>) -> Option<Arc<RwLock<Texture>>> {
        let tex_path = self.resolve_path(path, fallback_path);
        self.textures.get_shared(&tex_path)
    }

    pub fn get_frames(&self, sheet: StringPattern, trimmed: bool) -> (Vec<Arc<RwLock<Texture>>>, u32, u32) {
        match self.textures.get_shared(&sheet.to_string()) {
            Some(tex) => self.build_frames(tex, &sheet, trimmed),
            None => (Vec::new(), 1, 1),
        }
    }

    pub fn get_frames_from_tex(&self, sheet_tex: Option<Arc<RwLock<Texture>>>, trimmed: bool) -> (Vec<Arc<RwLock<Texture>>>, u32, u32) {
        match sheet_tex {
            Some(tex) => {
                let sheet = StringPattern::from(tex.get_path());
                self.build_frames(tex, &sheet, trimmed)
            }
            None => (Vec::new(), 1, 1),
        }
    }

    fn build_frames(&self, sheet_tex: Arc<RwLock<Texture>>, sheet: &StringPattern, trimmed: bool) -> (Vec<Arc<RwLock<Texture>>>, u32, u32) {
        if let Some((rows, cols)) = sheet.get_sheet_size() {
            let data = sheet_tex.get_data().unwrap();
            let raw_frames = if trimmed {
                extract_from_sheet_trimmed(&data, rows, cols)
            } else {
                extract_from_sheet(&data, rows, cols)
            };

            let frames = raw_frames
                .into_iter()
                .enumerate()
                .map(|(idx, img)| {
                    Arc::new(RwLock::new(Texture::new_with_state(
                        format!("{}-{}", sheet, idx),
                        BinaryState::Loaded(img),
                    )))
                })
                .collect();
            (frames, rows, cols)
        } else {
            (vec![sheet_tex], 1, 1)
        }
    }

    pub fn get_generic_or_shared_sheet(&self, pattern: StringPattern, rows: u32, cols: u32) -> Option<Arc<RwLock<Texture>>> {
        let with_dimensions = StringPattern::from(format!("{}@{}x{}", pattern, rows, cols));
        let generic = &self.keymode.get_generic(with_dimensions.clone(), 0);
        let shared = &self.keymode.get_shared(with_dimensions, 0);
        self.textures.get_shared(generic).or(self.textures.get_shared(shared))
    }

    pub fn resolve_snap_colored<TSnap, TBase>(
        &mut self,
        sheet_pattern: StringPattern,
        base_tex_name: &str,
        grayscale: bool,
        trimmed: bool,
        validate: impl FnOnce(u32, u32, usize) -> bool,
        build_snap: impl FnOnce(Vec<Arc<RwLock<Texture>>>, u32, u32, Vec<Rgba>) -> TSnap,
        build_base: impl FnOnce(Option<Arc<RwLock<Texture>>>) -> TBase,
    ) -> (Option<TSnap>, Option<TBase>, Vec<Rgba>) {
        let sheet_tex = self.get_generic_or_shared_sheet(sheet_pattern, 9, 1);
        let (frames, rows, cols) = self.get_frames_from_tex(sheet_tex, trimmed);

        if !validate(rows, cols, frames.len()) {
            return (None, None, Vec::new());
        }

        let colors: Vec<Rgba> = frames.iter()
            .map(|t| {
                let col = t.image_ref(|img| get_dominant_color(img, FilterType::Triangle));
                Rgba::from_image_rs(col.unwrap_or(image::Rgba([0, 0, 0, 0])))
            })
            .collect();

        let snap_cols = build_snap(frames.clone(), rows, cols, colors.clone());

        if !grayscale {
            let base_arc = frames.first().cloned();
            return (Some(snap_cols), Some(build_base(base_arc)), colors);
        }

        let images: Vec<DynamicImage> = frames.iter()
            .filter_map(|t| t.image_ref(|img| img.clone()))
            .collect();

        if images.is_empty() {
            return (Some(snap_cols), None, colors);
        }

        let images_refs: Vec<&DynamicImage> = images.iter().collect();
        let base_tex = Texture::with_data(
            base_tex_name.to_string(), 
            extract_grayscale_base(&images_refs, Some(&colors), FilterType::Triangle)
        );
        let base_arc = self.textures.insert(base_tex);

        (Some(snap_cols), Some(build_base(Some(base_arc))), colors)
    }
}
