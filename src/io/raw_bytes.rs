use wasm_bindgen::prelude::*;
use crate::{impl_binary_wasm, io::{Binary, BinaryState}};

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct RawBytes {
    #[wasm_bindgen(getter_with_clone)]
    pub path: String,
    #[wasm_bindgen(skip)]
    pub data: BinaryState<Vec<u8>>,
}

#[wasm_bindgen]
impl RawBytes {
    #[wasm_bindgen(constructor)]
    pub fn new(path: String) -> Self {
        RawBytes { 
            path, 
            data: BinaryState::Empty 
        }
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes_direct(path: String, bytes: Vec<u8>) -> Self {
        RawBytes { 
            path, 
            data: BinaryState::Loaded(bytes)
        }
    }

    #[wasm_bindgen(js_name = getData)]
    pub fn get_data_wasm(&self) -> Option<Uint8Array> {
        self.get_data().map(|bytes| Uint8Array::from(bytes.as_slice()))
    }
}

impl Binary for RawBytes {
    type Error = String;
    type LoadedData = Vec<u8>;
    
    fn state(&self) -> &BinaryState<Self::LoadedData> {
        &self.data
    }
    
    fn state_mut(&mut self) -> &mut BinaryState<Self::LoadedData> {
        &mut self.data
    }
    
    fn get_path(&self) -> &str {
        &self.path
    }
    
    fn new_with_state(path: String, state: BinaryState<Self::LoadedData>) -> Self {
        RawBytes { path, data: state }
    }
    
    fn decode_bytes(bytes: &[u8]) -> Result<Self::LoadedData, Self::Error> {
        Ok(bytes.to_vec())
    }
    
    fn encode_to_bytes(data: &Self::LoadedData) -> Result<Vec<u8>, Self::Error> {
        Ok(data.clone())
    }
    
    fn empty_error() -> Self::Error {
        "No binary data available".to_string()
    }
}

impl RawBytes {
    pub fn with_data(path: String, data: Vec<u8>) -> Self {
        RawBytes { 
            path, 
            data: BinaryState::Loaded(data) 
        }
    }
    
    pub fn with_unloaded_data(path: String, bytes: Vec<u8>) -> Self {
        RawBytes { 
            path, 
            data: BinaryState::Unloaded(bytes) 
        }
    }
}

impl_binary_wasm!(RawBytes, "Failed to process binary data");