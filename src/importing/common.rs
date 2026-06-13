#![allow(unused)]

use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

use crate::{Binary, Store, texture::{Texture, TextureStore}, utils::io::get_stem};
use crate::io::{BinaryState, StringPattern};
use crate::utils::io::normalize;

pub fn file_matches_target(file_stem: &str, target_filename: &str) -> bool {
    file_stem.to_lowercase() == target_filename.to_lowercase()
}

pub fn path_matches_target(
    file_path: &str,
    file_parent: &str,
    target_relative_path: &str,
    target_parent: &str,
) -> bool {
    if target_parent.is_empty() {
        return file_parent.is_empty() || !file_path.contains('/');
    }
    file_parent.to_lowercase().ends_with(&target_parent.to_lowercase())
}

pub fn extension_matches(extension: &str, allowed_extensions: &[&str]) -> bool {
    let ext_lower = extension.to_lowercase();
    allowed_extensions.contains(&ext_lower.as_str())
}

pub fn choose_best_match<'a>(
    matches: &'a [(String, String)],
    target_stem: &str,
    get_stem_fn: impl Fn(&str) -> String,
) -> Option<&'a (String, String)> {
    matches.iter()
        .find(|(name, _)| get_stem_fn(name) == target_stem)
        .or_else(|| matches.first())
}

pub fn is_at2x(path: &str) -> bool {
    path.ends_with("@2x")
}

pub fn strip_at2x(path: &str) -> &str {
    path.strip_suffix("@2x").unwrap_or(path)
}

pub fn pair_at2x_files<'a>(
    files: &'a HashMap<String, Vec<u8>>,
) -> Vec<TextureEntry<'a>> {
    let mut grouped: HashMap<&str, (Option<&'a [u8]>, Option<&'a [u8]>)> = HashMap::new();

    for (path, bytes) in files {
        let canonical = strip_at2x(path);
        let entry = grouped.entry(canonical).or_default();
        if is_at2x(path) {
            entry.1 = Some(bytes);
        } else {
            entry.0 = Some(bytes);
        }
    }

    grouped
        .into_iter()
        .map(|(canonical, (lores, hires))| match (lores, hires) {
            (Some(lores), Some(hires)) => TextureEntry::WithMip {
                canonical_path: canonical.to_string(),
                hires,
                lores,
            },
            (Some(bytes), None) | (None, Some(bytes)) => TextureEntry::Plain {
                path: canonical.to_string(),
                bytes,
            },
            (None, None) => unreachable!(),
        })
        .collect()
}

enum Decoded {
    Loaded {
        path: String,
        image: image::RgbaImage,
        hash: u64,
        mip: Option<image::RgbaImage>,
    },
    Unloaded {
        path: String,
        bytes: Vec<u8>,
        hash: u64,
    },
}

pub fn build_texture_store_from_files(
    files: &HashMap<String, Vec<u8>>,
    load_only: Option<&[StringPattern]>,
) -> Result<TextureStore, Box<dyn std::error::Error>> {
    let entries = pair_at2x_files(files);

    let decoded: Vec<Result<Decoded, image::ImageError>> = entries
        .par_iter()
        .map(|entry| match entry {
            TextureEntry::WithMip { canonical_path, hires, lores } => {
                let should_load = load_only
                    .map_or(true, |s| should_load_from_set(canonical_path, s));

                if should_load {
                    let hash = xxhash_rust::xxh3::xxh3_64(hires);
                    let image = image::load_from_memory(hires).map(|img| img.to_rgba8())?;
                    let mip = image::load_from_memory(lores).map(|img| img.to_rgba8())?;
                    Ok(Decoded::Loaded {
                        path: canonical_path.clone(),
                        image,
                        hash,
                        mip: Some(mip),
                    })
                } else {
                    let hash = xxhash_rust::xxh3::xxh3_64(hires);
                    Ok(Decoded::Unloaded {
                        path: canonical_path.clone(),
                        bytes: hires.to_vec(),
                        hash,
                    })
                }
            }
            TextureEntry::Plain { path, bytes } => {
                let should_load = load_only
                    .map_or(true, |s| should_load_from_set(path, s));

                if should_load {
                    let hash = xxhash_rust::xxh3::xxh3_64(bytes);
                    let image = image::load_from_memory(bytes).map(|img| img.to_rgba8())?;
                    Ok(Decoded::Loaded {
                        path: path.clone(),
                        image,
                        hash,
                        mip: None,
                    })
                } else {
                    let hash = xxhash_rust::xxh3::xxh3_64(bytes);
                    Ok(Decoded::Unloaded {
                        path: path.clone(),
                        bytes: bytes.to_vec(),
                        hash,
                    })
                }
            }
        })
        .collect();

    let store = TextureStore::new();
    decoded.into_par_iter().try_for_each(|result| {
        let decoded = result?;
        let (path, texture, mip) = match decoded {
            Decoded::Loaded { path, image, hash, mip } => {
                let texture = Texture {
                    path: path.clone(),
                    data: BinaryState::Loaded(image),
                    hash: Some(hash),
                };
                (path, texture, mip)
            }
            Decoded::Unloaded { path, bytes, hash } => {
                let texture = Texture {
                    path: path.clone(),
                    data: BinaryState::Unloaded(bytes),
                    hash: Some(hash),
                };
                (path, texture, None)
            }
        };

        let normalized = normalize(&path);
        let arc = std::sync::Arc::new(std::sync::RwLock::new(texture));
        store.textures.insert(normalized.clone(), arc);
        if let Some(mip_image) = mip {
            store.mipmaps.insert(normalized, vec![mip_image]);
        }
        Ok::<_, image::ImageError>(())
    })?;

    Ok(store)
}

pub enum TextureEntry<'a> {
    WithMip {
        canonical_path: String,
        hires: &'a [u8],
        lores: &'a [u8],
    },
    Plain {
        path: String,
        bytes: &'a [u8],
    },
}

#[derive(Default)]
pub struct SeenFiles {
    seen: HashSet<String>,
}

impl SeenFiles {
    pub fn new() -> Self { Self { seen: HashSet::new() } }

    pub fn try_insert(&mut self, path: &str) -> bool {
        self.seen.insert(path.to_lowercase())
    }

    pub fn contains(&self, path: &str) -> bool {
        self.seen.contains(&path.to_lowercase())
    }
}

pub fn should_load_from_set(path: &str, load_only: &[StringPattern]) -> bool {
    load_only.iter().any(|p| p.matches_path(path))
}
