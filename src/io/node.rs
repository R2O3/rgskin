#[cfg(all(target_arch = "wasm32", feature = "node"))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", feature = "node"))]
use js_sys;

#[cfg(all(target_arch = "wasm32", feature = "node"))]
#[wasm_bindgen(inline_js = r#"
import { readFileSync, readdirSync, statSync, writeFileSync, mkdirSync } from 'fs';
import { join } from 'path';

export function readFileBytes(path) {
    return new Uint8Array(readFileSync(path));
}

export function readFileString(path) {
    return readFileSync(path, 'utf8');
}

export function readDir(path) {
    return readdirSync(path);
}

export function isDirectory(path) {
    try {
        return statSync(path).isDirectory();
    } catch (e) {
        return false;
    }
}

export function joinPath(path1, path2) {
    return join(path1, path2);
}

export function writeFile(path, data) {
    writeFileSync(path, data);
}

export function createDirAll(path) {
    mkdirSync(path, { recursive: true });
}
"#)]
extern "C" {
    #[wasm_bindgen(catch)]
    fn readFileBytes(path: &str) -> Result<js_sys::Uint8Array, JsValue>;
    
    #[wasm_bindgen(catch)]
    fn readFileString(path: &str) -> Result<String, JsValue>;
    
    #[wasm_bindgen(catch)]
    fn readDir(path: &str) -> Result<js_sys::Array, JsValue>;
    
    #[wasm_bindgen(catch)]
    fn isDirectory(path: &str) -> Result<bool, JsValue>;
    
    #[wasm_bindgen(catch)]
    fn joinPath(path1: &str, path2: &str) -> Result<String, JsValue>;
    
    #[wasm_bindgen(catch)]
    fn writeFile(path: &str, data: &[u8]) -> Result<(), JsValue>;
    
    #[wasm_bindgen(catch)]
    fn createDirAll(path: &str) -> Result<(), JsValue>;
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub fn read_file_bytes(path: &str) -> Result<Vec<u8>, String> {
    let array = readFileBytes(path)
        .map_err(|e| format!("Failed to read file: {:?}", e))?;
    Ok(array.to_vec())
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub fn read_file_string(path: &str) -> Result<String, String> {
    readFileString(path)
        .map_err(|e| format!("Failed to read file: {:?}", e))
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub fn read_dir(path: &str) -> Result<Vec<String>, String> {
    let array = readDir(path)
        .map_err(|e| format!("Failed to read directory: {:?}", e))?;
    
    let mut result = Vec::new();
    for i in 0..array.length() {
        if let Some(entry) = array.get(i).as_string() {
            result.push(entry);
        }
    }
    Ok(result)
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub fn is_directory(path: &str) -> Result<bool, String> {
    isDirectory(path)
        .map_err(|e| format!("Failed to check if directory: {:?}", e))
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub fn join_path(path1: &str, path2: &str) -> String {
    joinPath(path1, path2).unwrap_or_else(|_| format!("{}/{}", path1, path2))
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub fn write_file(path: &str, data: &[u8]) -> Result<(), String> {
    writeFile(path, data)
        .map_err(|e| format!("Failed to write file: {:?}", e))
}

#[cfg(all(target_arch = "wasm32", feature = "node"))]
pub fn create_dir_all(path: &str) -> Result<(), String> {
    createDirAll(path)
        .map_err(|e| format!("Failed to create directory: {:?}", e))
}