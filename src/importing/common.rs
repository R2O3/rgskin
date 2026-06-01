#![allow(unused)]

use std::collections::{HashMap, HashSet};

use crate::{Binary, Store, texture::{Texture, TextureStore}, utils::io::get_stem};
use crate::io::StringPattern;

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

pub fn build_texture_store_from_files(
    files: &HashMap<String, Vec<u8>>,
    load_only: Option<&[StringPattern]>,
) -> Result<TextureStore, Box<dyn std::error::Error>> {
    let mut store = TextureStore::new();

    for entry in pair_at2x_files(files) {
        match entry {
            TextureEntry::WithMip { canonical_path, hires, lores } => {
                let should_load = load_only.map_or(true, |s| should_load_from_set(&canonical_path, s));

                if should_load {
                    let texture = Texture::from_bytes(canonical_path.clone(), hires)?;
                    let mip = image::load_from_memory(lores)?;
                    store.load_with_mipmaps(canonical_path.clone(), hires, vec![mip]);
                } else {
                    store.insert(Texture::with_unloaded_data(canonical_path, hires.to_vec()));
                }
            }
            TextureEntry::Plain { path, bytes } => {
                let should_load = load_only.map_or(true, |s| should_load_from_set(&path, s));

                if should_load {
                    store.load_from_bytes(path, bytes)?;
                } else {
                    store.insert(Texture::with_unloaded_data(path, bytes.to_vec()));
                }
            }
        }
    }

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
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }
    
    pub fn try_insert(&mut self, path: &str) -> bool {
        let path_lower = path.to_lowercase();
        self.seen.insert(path_lower)
    }
    
    pub fn contains(&self, path: &str) -> bool {
        let path_lower = path.to_lowercase();
        self.seen.contains(&path_lower)
    }
}

pub fn should_load_from_set(path: &str, load_only: &[StringPattern]) -> bool {
    load_only.iter().any(|p| p.matches_path(path))
}
