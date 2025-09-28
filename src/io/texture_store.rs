use std::{collections::HashMap, sync::{Arc, RwLock}};
use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array, ArrayBuffer, Array};
use image::ImageError;
use crate::io::{Store, Texture};

#[wasm_bindgen]
#[derive(Clone)]
pub struct TextureStore {
    #[wasm_bindgen(skip)]
    textures: HashMap<String, Arc<RwLock<Texture>>>,
}

#[wasm_bindgen]
impl TextureStore {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        TextureStore {
            textures: HashMap::new(),
        }
    }

    #[wasm_bindgen(js_name = loadFromArrayBuffer)]
    pub fn load_from_array_buffer_wasm(&mut self, path: String, buffer: &ArrayBuffer) -> Result<(), JsValue> {
        self.load_from_array_buffer(path, buffer)
            .map_err(|e| JsValue::from_str(&format!("Failed to load texture from array buffer: {}", e)))
    }

    #[wasm_bindgen(js_name = loadFromUint8Array)]
    pub fn load_from_uint8_array_wasm(&mut self, path: String, array: &Uint8Array) -> Result<(), JsValue> {
        self.load_from_uint8_array(path, array)
            .map_err(|e| JsValue::from_str(&format!("Failed to load texture from uint8 array: {}", e)))
    }

    #[wasm_bindgen(js_name = insertTexture)]
    pub fn insert_texture_wasm(&mut self, texture: Texture) {
        self.insert(texture);
    }

    #[wasm_bindgen(js_name = contains)]
    pub fn contains_wasm(&self, path: &str) -> bool {
        self.contains(path)
    }

    #[wasm_bindgen(js_name = remove)]
    pub fn remove_wasm(&mut self, path: &str) -> bool {
        self.remove(path).is_some()
    }

    #[wasm_bindgen(js_name = getLength)]
    pub fn get_length(&self) -> usize {
        self.len()
    }

    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty_wasm(&self) -> bool {
        self.is_empty()
    }

    #[wasm_bindgen(js_name = getPaths)]
    pub fn get_paths_wasm(&self) -> Array {
        let paths = self.get_paths();
        let js_array = Array::new();
        for path in paths {
            js_array.push(&JsValue::from_str(&path));
        }
        js_array
    }

    #[wasm_bindgen(js_name = clear)]
    pub fn clear_wasm(&mut self) {
        self.clear();
    }

    #[wasm_bindgen(js_name = makeUnique)]
    pub fn make_unique_wasm(&mut self, new_path: String, texture: Texture) -> String {
        self.make_unique(new_path, texture)
    }

    #[wasm_bindgen(js_name = copy)]
    pub fn copy_wasm(&mut self, original_path: &str, new_path: String) -> Option<String> {
        self.copy(original_path, new_path)
    }

    #[wasm_bindgen(js_name = makeUniqueCopy)]
    pub fn make_unique_copy_wasm(&mut self, original_path: &str, new_base_path: String) -> Option<String> {
        self.make_unique_copy(original_path, new_base_path)
    }

    #[wasm_bindgen(js_name = allLoaded)]
    pub fn all_loaded_wasm(&self) -> bool {
        self.all_loaded()
    }

    #[wasm_bindgen(js_name = loadedCount)]
    pub fn loaded_count_wasm(&self) -> usize {
        self.loaded_count()
    }

    #[wasm_bindgen(js_name = unloadedPaths)]
    pub fn unloaded_paths_wasm(&self) -> Array {
        let paths = self.unloaded_paths();
        let js_array = Array::new();
        for path in paths {
            js_array.push(&JsValue::from_str(&path));
        }
        js_array
    }

    #[wasm_bindgen(js_name = hasTexture)]
    pub fn has_texture_wasm(&self, path: &str) -> bool {
        self.contains(path)
    }

    #[wasm_bindgen(js_name = getTexturePath)]
    pub fn get_texture_path_wasm(&self, path: &str) -> Option<String> {
        self.get(path).map(|texture| texture.get_path())
    }

    #[wasm_bindgen(js_name = textureHasData)]
    pub fn texture_has_data_wasm(&self, path: &str) -> bool {
        self.get(path).map(|texture| texture.has_data()).unwrap_or(false)
    }
}

impl Store<Texture> for TextureStore {
    type Data = image::DynamicImage;
    
    fn insert(&mut self, texture: Texture) {
        let path = texture.path.clone();
        self.textures.insert(path, Arc::new(RwLock::new(texture)));
    }
    
    fn get(&self, path: &str) -> Option<std::sync::RwLockReadGuard<'_, Texture>> {
        self.textures.get(path).map(|arc| arc.read().unwrap())
    }
    
    fn get_shared(&self, path: &str) -> Option<Arc<RwLock<Texture>>> {
        self.textures.get(path).map(Arc::clone)
    }
    
    fn get_mut(&self, path: &str) -> Option<std::sync::RwLockWriteGuard<'_, Texture>> {
        self.textures.get(path).map(|arc| arc.write().unwrap())
    }
    
    fn contains(&self, path: &str) -> bool {
        self.textures.contains_key(path)
    }
    
    fn remove(&mut self, path: &str) -> Option<Arc<RwLock<Texture>>> {
        self.textures.remove(path)
    }
    
    fn len(&self) -> usize {
        self.textures.len()
    }
    
    fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }
    
    fn paths(&self) -> impl Iterator<Item = &str> {
        self.textures.keys().map(|s| s.as_str())
    }
    
    fn clear(&mut self) {
        self.textures.clear();
    }
    
    fn make_unique(&mut self, new_path: String, texture: Texture) -> String {
        if !self.contains(&new_path) {
            self.insert(texture);
            return new_path;
        }
        
        let (base_name, extension) = if let Some(dot_pos) = new_path.rfind('.') {
            let base = &new_path[..dot_pos];
            let ext = &new_path[dot_pos..];
            (base.to_string(), ext.to_string())
        } else {
            (new_path.clone(), String::new())
        };
        
        let mut counter = 1;
        loop {
            let candidate_path = format!("{}_{}{}", base_name, counter, extension);
            if !self.contains(&candidate_path) {
                let mut unique_texture = texture;
                unique_texture.path = candidate_path.clone();
                self.insert(unique_texture);
                return candidate_path;
            }
            counter += 1;
        }
    }
    
    fn copy(&mut self, original_path: &str, new_path: String) -> Option<String> {
        if let Some(original_texture_ref) = self.get_shared(original_path) {
            let original_texture = original_texture_ref.read().unwrap();
            
            let texture_copy = Texture {
                path: new_path.clone(),
                data: original_texture.data().clone(),
            };
            
            drop(original_texture);
            
            if self.contains(&new_path) {
                return None;
            }
            
            self.insert(texture_copy);
            Some(new_path)
        } else {
            None
        }
    }
    
    fn copy_from_data(&mut self, path: String, data: Self::Data) -> String {
        let texture = Texture::with_data(path.clone(), data);
        self.insert(texture);
        path
    }
    
    fn make_unique_copy(&mut self, original_path: &str, new_base_path: String) -> Option<String> {
        if let Some(original_texture_ref) = self.get_shared(original_path) {
            let original_texture = original_texture_ref.read().unwrap();
            
            let texture_copy = Texture {
                path: new_base_path.clone(),
                data: original_texture.data().clone(),
            };
            
            drop(original_texture);
            Some(self.make_unique(new_base_path, texture_copy))
        } else {
            None
        }
    }
    
    fn make_unique_from_data(&mut self, path: String, data: Self::Data) -> String {
        let texture = Texture::with_data(path.clone(), data);
        self.make_unique(path, texture)
    }
}

impl TextureStore {
    pub fn with_texture<F, R>(&self, path: &str, f: F) -> Option<R>
    where 
        F: FnOnce(&Texture) -> R,
    {
        let arc = self.textures.get(path)?;
        let texture = arc.read().unwrap();
        Some(f(&*texture))
    }
    
    pub fn with_texture_mut<F, R>(&self, path: &str, f: F) -> Option<R>
    where 
        F: FnOnce(&mut Texture) -> R,
    {
        let arc = self.textures.get(path)?;
        let mut texture = arc.write().unwrap();
        Some(f(&mut *texture))
    }
    
    pub fn for_each<F>(&self, mut f: F)
    where 
        F: FnMut(&Texture),
    {
        for arc in self.textures.values() {
            let texture = arc.read().unwrap();
            f(&*texture);
        }
    }
    
    pub fn for_each_mut<F>(&self, mut f: F)
    where 
        F: FnMut(&mut Texture),
    {
        for arc in self.textures.values() {
            let mut texture = arc.write().unwrap();
            f(&mut *texture);
        }
    }
    
    pub fn load_from_bytes(&mut self, path: String, bytes: Vec<u8>) -> Result<(), ImageError> {
        let texture = Texture::from_bytes(path, bytes)?;
        self.insert(texture);
        Ok(())
    }
    
    pub fn load_from_array_buffer(&mut self, path: String, buffer: &ArrayBuffer) -> Result<(), ImageError> {
        let texture = Texture::from_array_buffer(path, buffer)?;
        self.insert(texture);
        Ok(())
    }
    
    pub fn load_from_uint8_array(&mut self, path: String, array: &Uint8Array) -> Result<(), ImageError> {
        let texture = Texture::from_uint8_array(path, array)?;
        self.insert(texture);
        Ok(())
    }
    
    pub fn get_paths(&self) -> Vec<String> {
        self.textures.keys().cloned().collect()
    }
    
    pub fn all_loaded(&self) -> bool {
        self.textures.values().all(|arc| {
            let texture = arc.read().unwrap();
            texture.has_data()
        })
    }
    
    pub fn loaded_count(&self) -> usize {
        self.textures.values().filter(|arc| {
            let texture = arc.read().unwrap();
            texture.has_data()
        }).count()
    }
    
    pub fn unloaded_paths(&self) -> Vec<String> {
        self.textures.values().filter_map(|arc| {
            let texture = arc.read().unwrap();
            if !texture.has_data() {
                Some(texture.path().to_string())
            } else {
                None
            }
        }).collect()
    }
}

impl Default for TextureStore {
    fn default() -> Self {
        Self::new()
    }
}