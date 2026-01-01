use std::{collections::HashMap, sync::{Arc, RwLock}};
use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array, ArrayBuffer, Array};
use image::ImageError;
use crate::{impl_store_wasm, io::Store, Binary, BinaryState};
use crate::io::texture::Texture;

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

    #[wasm_bindgen(js_name = insertTexture)]
    pub fn insert_texture_wasm(&mut self, texture: Texture) {
        self.insert(texture);
    }

    #[wasm_bindgen(js_name = makeUnique)]
    pub fn make_unique_wasm(&mut self, new_path: &str, texture: Texture) -> String {
        self.make_unique(new_path, texture)
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
        let js_array = Array::new();
        for path in self.unloaded_paths() {
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
        self.with_item(path, |texture| texture.get_path().to_string())
    }

    #[wasm_bindgen(js_name = textureHasData)]
    pub fn texture_has_data_wasm(&self, path: &str) -> bool {
        self.with_item(path, |texture| texture.has_data()).unwrap_or(false)
    }
}

impl_store_wasm!(TextureStore, Texture);

impl Store<Texture> for TextureStore {
    type Data = BinaryState<image::DynamicImage>;
    
    fn create_item(path: String, data: Self::Data) -> Texture {
        Texture {
            path,
            data,
        }
    }
    
    fn get_item_path(item: &Texture) -> &str {
        &item.path
    }
    
    fn set_item_path(item: &mut Texture, path: String) {
        item.path = path;
    }
    
    fn clone_item_data(item: &Texture) -> Self::Data {
        item.state().clone()
    }
    
    fn map(&self) -> &HashMap<String, Arc<RwLock<Texture>>> {
        &self.textures
    }
    
    fn map_mut(&mut self) -> &mut HashMap<String, Arc<RwLock<Texture>>> {
        &mut self.textures
    }
}

impl TextureStore {
    pub fn load_from_bytes(&mut self, path: String, bytes: &[u8]) -> Result<(), ImageError> {
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
    
    pub fn all_loaded(&self) -> bool {
        self.iter().all(|(_, arc)| {
            let texture = arc.read().unwrap();
            texture.has_data()
        })
    }
    
    pub fn loaded_count(&self) -> usize {
        self.iter().filter(|(_, arc)| {
            let texture = arc.read().unwrap();
            texture.has_data()
        }).count()
    }
    
    pub fn unloaded_paths(&self) -> Vec<String> {
        self.iter().filter_map(|(_, arc)| {
            let texture = arc.read().unwrap();
            if !texture.has_data() {
                Some(texture.get_path().to_string())
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