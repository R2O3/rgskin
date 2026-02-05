#![allow(unused)]

use std::collections::HashSet;

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

pub fn should_load_from_set(path: &str, load_only: &HashSet<String>) -> bool {
    let path_lower = path.to_lowercase();
    load_only.iter().any(|s| s.to_lowercase() == path_lower)
}
