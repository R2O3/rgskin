use js_sys::{Array, Map, Uint8Array};
use std::collections::HashMap;
use wasm_bindgen::{JsError, JsValue};

pub fn js_to_hash(files: Map) -> HashMap<String, Vec<u8>> {
    let mut map = HashMap::new();
    files.for_each(&mut |value, key| {
        if let Some(k) = key.as_string() {
            map.insert(k, Uint8Array::new(&value).to_vec());
        }
    });
    map
}

pub fn hash_to_js(files: HashMap<String, Vec<u8>>) -> Map {
    let map = Map::new();
    for (key, value) in files {
        map.set(&JsValue::from_str(&key), &Uint8Array::from(&value[..]));
    }
    map
}

pub fn arr_to_strs(arr: Array) -> Result<Vec<String>, JsError> {
    arr.iter()
        .map(|v| v.as_string().ok_or_else(|| JsError::new("Expected string in array")))
        .collect()
}
