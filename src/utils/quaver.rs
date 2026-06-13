use std::sync::{Arc, RwLock};

use fast_image_resize::FilterType;
use image::RgbaImage;

use crate::{
    Binary, BinaryArcExt, BinaryState, Store, StringPattern, TextureArcExt,
    common::color::Rgba, 
    image_proc::proc::{extract_from_sheet, extract_from_sheet_trimmed, extract_grayscale_base, get_dominant_color}, 
    numeric_enum, quaver, texture::{Texture, TextureProcessor}, traits::KeymodeInvariant,
};

numeric_enum! {
    pub enum QuaDimensions: u32 {
        X = 615,
        Y = 346,
        MaxResolution = 4096, // TODO: change this later
    }
}

pub struct TextureResolver<'a, 'p, S: Store<Texture>> {
    textures: &'a mut S,
    keymode: &'a quaver::Keymode,
    blank_texture: Arc<RwLock<Texture>>,
    frame_processor: &'p mut TextureProcessor<(Vec<Arc<RwLock<Texture>>>, u32, u32)>,
    snap_processor: &'p mut TextureProcessor<(Vec<Rgba>, Option<Arc<RwLock<Texture>>>)>,
}

impl<'a, 'p, S: Store<Texture>> TextureResolver<'a, 'p, S> {
    pub fn new(
        textures: &'a mut S, 
        keymode: &'a quaver::Keymode, 
        blank_texture: Arc<RwLock<Texture>>,
        frame_processor: &'p mut TextureProcessor<(Vec<Arc<RwLock<Texture>>>, u32, u32)>,
        snap_processor: &'p mut TextureProcessor<(Vec<Rgba>, Option<Arc<RwLock<Texture>>>)>,
    ) -> Self {
        Self { 
            textures, 
            keymode, 
            blank_texture,
            frame_processor,
            snap_processor,
        }
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

    pub fn get_frames(&mut self, sheet: StringPattern, trimmed: bool) -> (Vec<Arc<RwLock<Texture>>>, u32, u32) {
        match self.textures.get_shared(&sheet.to_string()) {
            Some(tex) => self.build_frames(tex, &sheet, trimmed),
            None => (Vec::new(), 1, 1),
        }
    }

    pub fn get_frames_from_tex(&mut self, sheet_tex: Option<Arc<RwLock<Texture>>>, trimmed: bool) -> (Vec<Arc<RwLock<Texture>>>, u32, u32) {
        match sheet_tex {
            Some(tex) => {
                let sheet = StringPattern::from(tex.get_path());
                self.build_frames(tex, &sheet, trimmed)
            }
            None => (Vec::new(), 1, 1),
        }
    }

    fn build_frames(&mut self, sheet_tex: Arc<RwLock<Texture>>, sheet: &StringPattern, trimmed: bool) -> (Vec<Arc<RwLock<Texture>>>, u32, u32) {
        let sheet_clone = sheet.clone();

        self.frame_processor.process_once(&sheet_tex, move |tex| {
            if let Some((rows, cols)) = sheet_clone.get_sheet_size() {
                let data = tex.get_data().unwrap();
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
                            format!("{}-{}", sheet_clone, idx),
                            BinaryState::Loaded(img),
                        )))
                    })
                    .collect();
                (frames, rows, cols)
            } else {
                (vec![tex.clone()], 1, 1)
            }
        })
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
        let sheet_arc = match &sheet_tex {
            Some(arc) => arc,
            None => return (None, None, Vec::new()),
        };

        let (frames, rows, cols) = self.get_frames_from_tex(sheet_tex.clone(), trimmed);

        if !validate(rows, cols, frames.len()) {
            return (None, None, Vec::new());
        }

        let textures = &mut self.textures;

        let (colors, base_arc) = self.snap_processor.process_once(&sheet_arc, |_| {
            let colors: Vec<Rgba> = frames.iter()
                .map(|t| {
                    let col = t.image_ref(|img| get_dominant_color(img, FilterType::Hamming));
                    Rgba::from_image_rs(col.unwrap_or(image::Rgba([0, 0, 0, 0])))
                })
                .collect();

            let mut base_arc = None;

            if grayscale && !frames.is_empty() {
                fn stack_locks<'a>(
                    frames: &'a [Arc<RwLock<Texture>>],
                    refs: &mut Vec<&'a RgbaImage>,
                    f: impl FnOnce(&[&RgbaImage]),
                ) {
                    if let Some((first, rest)) = frames.split_first() {
                        first.with_image(|img| {
                            let img_unsafe_ref = unsafe { &*(img as *const RgbaImage) };
                            refs.push(img_unsafe_ref);
                            stack_locks(rest, refs, f);
                        });
                    } else {
                        f(refs);
                    }
                }

                let mut images_refs = Vec::with_capacity(frames.len());
                stack_locks(&frames, &mut images_refs, |refs| {
                    let base_tex = Texture::with_data(
                        base_tex_name.to_string(),
                        extract_grayscale_base(refs, Some(&colors), FilterType::Hamming),
                    );
                    base_arc = Some(textures.insert(base_tex));
                });
            }

            (colors, base_arc)
        });

        let base_result = if !grayscale {
            Some(build_base(frames.first().cloned()))
        } else {
            base_arc.clone().map(|arc| build_base(Some(arc)))
        };

        let snap_cols = build_snap(frames, rows, cols, colors.clone());

        (Some(snap_cols), base_result, colors)
    }
}
