use std::collections::{HashMap, hash_map};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;
use js_sys::Array;

use crate::Binary;
use crate::common_traits::MapOps;
use crate::utils::io::normalize;

#[macro_export]
macro_rules! impl_store_wasm {
    ($store_type:ty, $item_type:ty) => {
        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen]
        impl $store_type {
            #[wasm_bindgen(js_name = contains)]
            pub fn contains_wasm(&self, path: &str) -> bool {
                <Self as Store<$item_type>>::wasm_contains(self, path)
            }

            #[wasm_bindgen(js_name = remove)]
            pub fn remove_wasm(&mut self, path: &str) -> bool {
                <Self as Store<$item_type>>::wasm_remove(self, path)
            }

            #[wasm_bindgen(js_name = getLength)]
            pub fn get_length(&self) -> usize {
                <Self as Store<$item_type>>::wasm_get_length(self)
            }

            #[wasm_bindgen(js_name = isEmpty)]
            pub fn is_empty_wasm(&self) -> bool {
                <Self as Store<$item_type>>::wasm_is_empty(self)
            }

            #[wasm_bindgen(js_name = getPaths)]
            pub fn get_paths_wasm(&self) -> Array {
                <Self as Store<$item_type>>::wasm_get_paths(self)
            }

            #[wasm_bindgen(js_name = clear)]
            pub fn clear_wasm(&mut self) {
                <Self as Store<$item_type>>::wasm_clear(self)
            }

            #[wasm_bindgen(js_name = copy)]
            pub fn copy_wasm(&mut self, original_path: &str, new_path: &str) -> Option<String> {
                <Self as Store<$item_type>>::wasm_copy(self, original_path, new_path)
            }

            #[wasm_bindgen(js_name = makeUniqueCopy)]
            pub fn make_unique_copy_wasm(&mut self, original_path: &str, new_base_path: &str) -> Option<String> {
                <Self as Store<$item_type>>::wasm_make_unique_copy(self, original_path, new_base_path)
            }         

            #[wasm_bindgen(js_name = extend)]
            pub fn extend_wasm(&mut self, other: &$store_type) {
                <Self as Store<$item_type>>::wasm_extend(self, other)
            }
        }
    };
}

pub trait Store<T: Binary>: Debug {
    type Data;
    type MapType: MapOps<String, Arc<RwLock<T>>>;

    fn create_item(path: String, data: Self::Data, hash: Option<u64>) -> T;
    fn get_item_path(item: &T) -> &str;
    fn set_item_path(item: &mut T, path: String);
    fn clone_item_data(item: &T) -> Self::Data;

    fn map(&self) -> &Self::MapType;
    fn map_mut(&mut self) -> &mut Self::MapType;

    fn insert(&mut self, item: T) -> Arc<RwLock<T>> {
        let path = normalize(Self::get_item_path(&item));
        let arc = Arc::new(RwLock::new(item));
        self.map_mut().insert(path, arc.clone());
        arc
    }

    fn insert_shared(&mut self, arc: Arc<RwLock<T>>) -> Arc<RwLock<T>> {
        let path = normalize(Self::get_item_path(&*arc.read().unwrap()));
        self.map_mut().insert(path, arc.clone());
        arc
    }
    
    fn get(&self, path: &str) -> Option<Arc<RwLock<T>>> {
        let normalized = normalize(path);
        self.map().clone_value(&normalized)
    }
    
    fn get_shared(&self, path: &str) -> Option<Arc<RwLock<T>>> {
        self.get(path)
    }
    
    fn get_mut(&self, path: &str) -> Option<Arc<RwLock<T>>> {
        self.get(path)
    }

    fn get_all<F>(&self, mut predicate: F) -> Vec<(String, Arc<RwLock<T>>)>
    where
        F: FnMut(&T) -> bool,
    {
        let mut result = Vec::new();
        self.map().for_each_entry(|k, arc| {
            let guard = arc.read().unwrap();
            if predicate(&*guard) {
                result.push((k.clone(), Arc::clone(arc)));
            }
        });
        result
    }

    fn get_shared_all<F>(&self, predicate: F) -> Vec<(String, Arc<RwLock<T>>)>
    where
        F: FnMut(&T) -> bool,
    {
        self.get_all(predicate)
    }

    fn get_mut_all<F>(&self, predicate: F) -> Vec<(String, Arc<RwLock<T>>)>
    where
        F: FnMut(&T) -> bool,
    {
        self.get_all(predicate) // write is not possible without guard lifetime issues
    }

    fn contains(&self, path: &str) -> bool {
        let normalized = normalize(path);
        self.map().contains_key(&normalized)
    }
    
    fn remove(&mut self, path: &str) -> Option<Arc<RwLock<T>>> {
        let normalized = normalize(path);
        self.map_mut().remove(&normalized)
    }
    
    fn len(&self) -> usize {
        self.map().len()
    }
    
    fn is_empty(&self) -> bool {
        self.map().is_empty()
    }
    
    fn clear(&mut self) {
        self.map_mut().clear();
    }

    fn keys(&self) -> Vec<String> {
        self.map().keys_cloned()
    }

    fn iter(&self) -> Vec<(String, Arc<RwLock<T>>)> {
        let mut vec = Vec::with_capacity(self.len());
        self.map().for_each_entry(|k, v| {
            vec.push((k.clone(), Arc::clone(v)));
        });
        vec
    }

    fn iter_mut(&mut self) -> Vec<(String, Arc<RwLock<T>>)> {
        self.iter()
    }

    fn paths(&self) -> Vec<String> {
        self.keys()
    }

    fn get_paths(&self) -> Vec<String> {
        self.keys()
    }

    fn make_unique(&mut self, new_path: &str, mut item: T) -> String {
        let normalized = normalize(new_path);
        if !self.contains(&normalized) {
            Self::set_item_path(&mut item, new_path.to_string());
            self.insert(item);
            return normalized;
        }
        
        let (base_name, extension) = if let Some(dot_pos) = new_path.rfind('.') {
            let base = &new_path[..dot_pos];
            let ext = &new_path[dot_pos..];
            (base.to_string(), ext.to_string())
        } else {
            (new_path.to_string(), String::new())
        };
        
        let mut counter = 1;
        loop {
            let candidate_path = format!("{}_{}{}", base_name, counter, extension);
            if !self.contains(&candidate_path) {
                Self::set_item_path(&mut item, candidate_path.clone());
                self.insert(item);
                return normalize(&candidate_path);
            }
            counter += 1;
        }
    }
    
    fn copy(&mut self, original_path: &str, new_path: &str) -> Option<String> {
        let normalized_new = normalize(new_path);
        if self.contains(&normalized_new) {
            return None;
        }

        let original_ref = self.get_shared(original_path)?;
        let original = original_ref.read().unwrap();
        let data = Self::clone_item_data(&*original);
        let hash = original.get_hash();
        drop(original);

        let new_item = Self::create_item(new_path.to_string(), data, hash);
        self.insert(new_item);
        Some(normalized_new)
    }

    fn copy_from_data(&mut self, path: &str, data: Self::Data) -> String {
        let normalized = normalize(path);
        let item = Self::create_item(path.to_string(), data, None);
        self.insert(item);
        normalized
    }

    fn make_unique_copy(&mut self, original_path: &str, new_base_path: &str) -> Option<String> {
        let original_ref = self.get_shared(original_path)?;
        let original = original_ref.read().unwrap();
        let data = Self::clone_item_data(&*original);
        let hash = original.get_hash();
        drop(original);

        let new_item = Self::create_item(new_base_path.to_string(), data, hash);
        Some(self.make_unique(new_base_path, new_item))
    }

    fn make_unique_from_data(&mut self, path: &str, data: Self::Data) -> String {
        let item = Self::create_item(path.to_string(), data, None);
        self.make_unique(path, item)
    }

    fn dedupe(&mut self, path: &str) -> HashMap<String, String> {
        let normalized = normalize(path);

        let (target_hash, target_arc) = {
            let arc = match self.map().clone_value(&normalized) {
                Some(a) => a,
                None => return HashMap::new(),
            };
            let hash = match arc.read().unwrap().get_hash() {
                Some(h) => h,
                None => return HashMap::new(),
            };
            (hash, arc)
        };

        let mut culled = HashMap::new();
        let keys = self.map().keys_cloned();
        for key in keys {
            if key == normalized {
                continue;
            }
            if let Some(arc) = self.map().clone_value(&key) {
                if arc.read().unwrap().get_hash() == Some(target_hash) {
                    self.map_mut().insert(key.clone(), Arc::clone(&target_arc));
                    culled.insert(key, normalized.clone());
                }
            }
        }
        culled
    }

    fn dedupe_all(&mut self) -> HashMap<String, String> {
        let mut canonical: HashMap<u64, (Arc<RwLock<T>>, String)> = HashMap::new();
        let mut replacements: Vec<(String, Arc<RwLock<T>>, String)> = Vec::new();

        let entries: Vec<(String, Arc<RwLock<T>>)> = self.iter();
        for (path, arc) in entries {
            if let Some(hash) = arc.read().unwrap().get_hash() {
                match canonical.entry(hash) {
                    hash_map::Entry::Vacant(e) => {
                        e.insert((Arc::clone(&arc), path.to_string()));
                    }
                    hash_map::Entry::Occupied(e) => {
                        let (canonical_arc, canonical_path) = e.get();
                        replacements.push((path, Arc::clone(canonical_arc), canonical_path.clone()));
                    }
                }
            }
        }

        let mut culled = HashMap::new();
        for (path, arc, canonical_path) in replacements {
            self.map_mut().insert(path.clone(), arc);
            culled.insert(path, canonical_path);
        }
        culled
    }

    fn with_item<F, R>(&self, path: &str, f: F) -> Option<R>
    where 
        F: FnOnce(&T) -> R,
    {
        let arc = self.get(path)?;
        let guard = arc.read().unwrap();
        Some(f(&*guard))
    }
    
    fn with_item_mut<F, R>(&self, path: &str, f: F) -> Option<R>
    where 
        F: FnOnce(&mut T) -> R,
    {
        let arc = self.get(path)?;
        let mut guard = arc.write().unwrap();
        Some(f(&mut *guard))
    }
    
    fn for_each<F>(&self, mut f: F)
    where 
        F: FnMut(&T),
    {
        self.map().for_each_entry(|_, arc| {
            let guard = arc.read().unwrap();
            f(&*guard);
        });
    }
    
    fn for_each_mut<F>(&self, mut f: F)
    where 
        F: FnMut(&mut T),
    {
        let paths = self.map().keys_cloned();
        for path in paths {
            if let Some(arc) = self.map().clone_value(&path) {
                let mut guard = arc.write().unwrap();
                f(&mut *guard);
            }
        }
    }

    fn retain<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.map_mut().retain(|_, arc| {
            let guard = arc.read().unwrap();
            predicate(&*guard)
        });
    }

    fn extend(&mut self, other: Self::MapType) {
        other.for_each_entry(|k, v| {
            let normalized = normalize(k);
            self.map_mut().insert(normalized, Arc::clone(v));
        });
    }

    fn wasm_contains(&self, path: &str) -> bool {
        self.contains(path)
    }

    fn wasm_remove(&mut self, path: &str) -> bool {
        self.remove(path).is_some()
    }

    fn wasm_get_length(&self) -> usize {
        self.len()
    }

    fn wasm_is_empty(&self) -> bool {
        self.is_empty()
    }

    fn wasm_get_paths(&self) -> Array {
        let js_array = Array::new();
        for path in self.get_paths() {
            js_array.push(&JsValue::from_str(&path));
        }
        js_array
    }

    fn wasm_clear(&mut self) {
        self.clear();
    }

    fn wasm_make_unique_copy(&mut self, original_path: &str, new_base_path: &str) -> Option<String> {
        self.make_unique_copy(original_path, new_base_path)
    }

    fn wasm_copy(&mut self, original_path: &str, new_path: &str) -> Option<String> {
        self.copy(original_path, new_path)
    }

    fn wasm_extend(&mut self, other: &Self) {
        let entries = other.iter();
        for (path, item) in entries {
            let normalized = normalize(&path);
            self.map_mut().insert(normalized, item);
        }
    }
}
