use std::{collections::HashMap, sync::{Arc, RwLock}};
use merge::Merge;
use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array, ArrayBuffer, Array};
use crate::{impl_store_wasm, io::{Binary, BinaryState, RawBytes, Store}};

#[wasm_bindgen]
#[derive(Clone, Merge)]
pub struct BinaryStore {
    #[wasm_bindgen(skip)]
    #[merge(strategy = merge::hashmap::overwrite)]
    binaries: HashMap<String, Arc<RwLock<RawBytes>>>,
}

#[wasm_bindgen]
impl BinaryStore {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        BinaryStore {
            binaries: HashMap::new(),
        }
    }

    #[wasm_bindgen(js_name = insertBinary)]
    pub fn insert_binary_wasm(&mut self, binary: RawBytes) {
        self.insert(binary);
    }

    #[wasm_bindgen(js_name = makeUnique)]
    pub fn make_unique_wasm(&mut self, new_path: &str, binary: RawBytes) -> String {
        self.make_unique(new_path, binary)
    }

    #[wasm_bindgen(js_name = loadFromArrayBuffer)]
    pub fn load_from_array_buffer_wasm(&mut self, path: String, buffer: &ArrayBuffer) -> Result<(), JsValue> {
        self.load_from_array_buffer(path, buffer)
            .map_err(|e| JsValue::from_str(&format!("Failed to load binary from array buffer: {}", e)))
    }

    #[wasm_bindgen(js_name = loadFromUint8Array)]
    pub fn load_from_uint8_array_wasm(&mut self, path: String, array: &Uint8Array) -> Result<(), JsValue> {
        self.load_from_uint8_array(path, array)
            .map_err(|e| JsValue::from_str(&format!("Failed to load binary from uint8 array: {}", e)))
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

    #[wasm_bindgen(js_name = hasBinary)]
    pub fn has_binary_wasm(&self, path: &str) -> bool {
        self.contains(path)
    }

    #[wasm_bindgen(js_name = getBinaryPath)]
    pub fn get_binary_path_wasm(&self, path: &str) -> Option<String> {
        self.with_item(path, |binary| binary.get_path().to_string())
    }

    #[wasm_bindgen(js_name = binaryHasData)]
    pub fn binary_has_data_wasm(&self, path: &str) -> bool {
        self.with_item(path, |binary| binary.has_data()).unwrap_or(false)
    }

    #[wasm_bindgen(js_name = getBinaryData)]
    pub fn get_binary_data_wasm(&self, path: &str) -> Option<Uint8Array> {
        self.with_item(path, |binary| {
            binary.get_data().map(|data| Uint8Array::from(data.as_slice()))
        }).flatten()
    }
}

impl_store_wasm!(BinaryStore, RawBytes);

impl Store<RawBytes> for BinaryStore {
    type Data = BinaryState<Vec<u8>>;
    
    fn create_item(path: String, data: Self::Data) -> RawBytes {
        RawBytes {
            path,
            data,
        }
    }
    
    fn get_item_path(item: &RawBytes) -> &str {
        &item.path
    }
    
    fn set_item_path(item: &mut RawBytes, path: String) {
        item.path = path;
    }
    
    fn clone_item_data(item: &RawBytes) -> Self::Data {
        item.state().clone()
    }
    
    fn map(&self) -> &HashMap<String, Arc<RwLock<RawBytes>>> {
        &self.binaries
    }
    
    fn map_mut(&mut self) -> &mut HashMap<String, Arc<RwLock<RawBytes>>> {
        &mut self.binaries
    }
}

impl BinaryStore {
    pub fn load_from_bytes(&mut self, path: String, bytes: &[u8]) -> Result<(), String> {
        let binary = RawBytes::from_bytes(path, bytes)?;
        self.insert(binary);
        Ok(())
    }
    
    pub fn load_from_array_buffer(&mut self, path: String, buffer: &ArrayBuffer) -> Result<(), String> {
        let binary = RawBytes::from_array_buffer(path, buffer)?;
        self.insert(binary);
        Ok(())
    }
    
    pub fn load_from_uint8_array(&mut self, path: String, array: &Uint8Array) -> Result<(), String> {
        let binary = RawBytes::from_uint8_array(path, array)?;
        self.insert(binary);
        Ok(())
    }
    
    pub fn all_loaded(&self) -> bool {
        self.iter().all(|(_, arc)| {
            let binary = arc.read().unwrap();
            binary.has_data()
        })
    }
    
    pub fn loaded_count(&self) -> usize {
        self.iter().filter(|(_, arc)| {
            let binary = arc.read().unwrap();
            binary.has_data()
        }).count()
    }
    
    pub fn unloaded_paths(&self) -> Vec<String> {
        self.iter().filter_map(|(_, arc)| {
            let binary = arc.read().unwrap();
            if !binary.has_data() {
                Some(binary.get_path().to_string())
            } else {
                None
            }
        }).collect()
    }
}

impl Default for BinaryStore {
    fn default() -> Self {
        Self::new()
    }
}
