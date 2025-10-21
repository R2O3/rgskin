// would prefer to use strings instead of std path for wasm

use std::path::Path;

#[inline]
pub fn path_to_unix(str: &str) -> String {
    str.replace('\\', "/")
}

#[inline]
pub fn path_to_win(str: &str) -> String {
    str.replace('/', "\\")
}

#[inline]
pub fn get_extension(path: &str) -> String {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase()
}

#[inline]
pub fn has_extension(path: &str, ext: &str) -> bool {
    get_extension(path) == ext.to_lowercase()
}

#[inline]
pub fn has_any_extension(path: &str, extensions: &[&str]) -> bool {
    let file_ext = get_extension(path);
    extensions.iter().any(|&ext| file_ext == ext.to_lowercase())
}

#[inline]
pub fn change_extension(path: &str, new_ext: &str) -> String {
    let path_buf = Path::new(path);
    let ext = if new_ext.starts_with('.') {
        new_ext.to_string()
    } else {
        format!(".{}", new_ext)
    };
    
    path_buf.with_extension(&ext[1..]).to_string_lossy().to_string()
}

#[inline]
pub fn remove_extension(path: &str) -> String {
    path_to_unix(&Path::new(path)
        .with_extension("")
        .to_string_lossy())
}

#[inline]
pub fn get_stem(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("")
        .to_string()
}

#[inline]
pub fn get_filename(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_string()
}

#[inline]
pub fn get_parent(path: &str) -> String {
    Path::new(path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string())
}

#[inline]
pub fn join_paths_unix(base: &str, path: &str) -> String {
    path_to_unix(&Path::new(base).join(path).to_string_lossy().to_string())
}

#[inline]
pub fn join_paths_win(base: &str, path: &str) -> String {
    path_to_win(&Path::new(base).join(path).to_string_lossy().to_string())
}

#[inline]
pub fn is_absolute(path: &str) -> bool {
    Path::new(path).is_absolute()
}

#[inline]
pub fn is_relative(path: &str) -> bool {
    !is_absolute(path)
}

#[inline]
pub fn ensure_trailing_separator(path: &str) -> String {
    if path.is_empty() {
        return path.to_string();
    }
    
    let separator = if cfg!(windows) { '\\' } else { '/' };
    if path.ends_with('/') || path.ends_with('\\') {
        path.to_string()
    } else {
        format!("{}{}", path, separator)
    }
}

#[inline]
pub fn remove_trailing_separator(path: &str) -> String {
    path.trim_end_matches(['/', '\\']).to_string()
}