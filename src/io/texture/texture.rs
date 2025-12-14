use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array, ArrayBuffer};
use image::{DynamicImage, ImageError};
use crate::io::{Binary, BinaryState};

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct Texture {
    #[wasm_bindgen(getter_with_clone)]
    pub path: String,
    #[wasm_bindgen(skip)]
    pub data: BinaryState<DynamicImage>,
}

#[wasm_bindgen]
impl Texture {
    #[wasm_bindgen(constructor)]
    pub fn new(path: String) -> Self {
        Texture { 
            path, 
            data: BinaryState::Empty 
        }
    }

    pub fn from_single_px(path: String) -> Self {
        use image::{ImageBuffer, Rgba};
        let img = ImageBuffer::from_pixel(1, 1, Rgba([0, 0, 0, 0]));
        let dynamic_img = DynamicImage::ImageRgba8(img);
        Texture { 
            path, 
            data: BinaryState::Loaded(dynamic_img)
        }
    }

    #[wasm_bindgen(js_name = fromArrayBuffer)]
    pub fn from_array_buffer_wasm(path: String, buffer: &ArrayBuffer) -> Result<Texture, JsValue> {
        Self::from_array_buffer(path, buffer)
            .map_err(|e| JsValue::from_str(&format!("Failed to load texture: {}", e)))
    }
    
    #[wasm_bindgen(js_name = fromArrayBufferUnloaded)]
    pub fn from_array_buffer_unloaded_wasm(path: String, buffer: &ArrayBuffer) -> Result<Texture, JsValue> {
        Self::from_array_buffer_unloaded(path, buffer)
            .map_err(|e| JsValue::from_str(&format!("Failed to create texture: {}", e)))
    }
    
    #[wasm_bindgen(js_name = fromUint8Array)]
    pub fn from_uint8_array_wasm(path: String, array: &Uint8Array) -> Result<Texture, JsValue> {
        Self::from_uint8_array(path, array)
            .map_err(|e| JsValue::from_str(&format!("Failed to load texture: {}", e)))
    }

    #[wasm_bindgen(js_name = fromUint8ArrayUnloaded)]
    pub fn from_uint8_array_unloaded_wasm(path: String, array: &Uint8Array) -> Result<Texture, JsValue> {
        Self::from_uint8_array_unloaded(path, array)
            .map_err(|e| JsValue::from_str(&format!("Failed to create texture: {}", e)))
    }

    #[wasm_bindgen(js_name = getPath)]
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    #[wasm_bindgen(js_name = hasData)]
    pub fn has_data_wasm(&self) -> bool {
        self.has_data()
    }

    #[wasm_bindgen(js_name = isLoaded)]
    pub fn is_loaded_wasm(&self) -> bool {
        self.is_loaded()
    }

    #[wasm_bindgen(js_name = isUnloaded)]
    pub fn is_unloaded_wasm(&self) -> bool {
        self.is_unloaded()
    }

    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty_wasm(&self) -> bool {
        self.is_empty()
    }

    #[wasm_bindgen(js_name = load)]
    pub fn load_wasm(&mut self) -> Result<(), JsValue> {
        self.load()
            .map_err(|e| JsValue::from_str(&format!("Failed to load texture: {}", e)))
    }

    #[wasm_bindgen(js_name = unload)]
    pub fn unload_wasm(&mut self) -> Result<(), JsValue> {
        self.unload()
            .map_err(|e| JsValue::from_str(&format!("Failed to unload texture: {}", e)))
    }
}

impl Texture {
    pub fn with_data(path: String, data: DynamicImage) -> Self {
        Texture { 
            path, 
            data: BinaryState::Loaded(data) 
        }
    }
    
    pub fn with_unloaded_data(path: String, bytes: Vec<u8>) -> Self {
        Texture { 
            path, 
            data: BinaryState::Unloaded(bytes) 
        }
    }
    
    pub fn set_data(&mut self, data: DynamicImage) {
        self.data = BinaryState::Loaded(data);
    }
    
    pub fn get_data(&self) -> Option<&DynamicImage> {
        self.data.as_loaded()
    }
    
    pub fn take_data(&mut self) -> Option<DynamicImage> {
        self.data.take_loaded()
    }
    
    pub fn replace_data(&mut self, data: DynamicImage) -> BinaryState<DynamicImage> {
        std::mem::replace(&mut self.data, BinaryState::Loaded(data))
    }
    
    pub fn set_unloaded_data(&mut self, bytes: Vec<u8>) {
        self.data = BinaryState::Unloaded(bytes);
    }
    
    pub fn get_unloaded_data(&self) -> Option<&Vec<u8>> {
        self.data.as_unloaded()
    }
    
    pub fn take_unloaded_data(&mut self) -> Option<Vec<u8>> {
        self.data.take_unloaded()
    }
    
    pub fn replace_unloaded_data(&mut self, bytes: Vec<u8>) -> BinaryState<DynamicImage> {
        std::mem::replace(&mut self.data, BinaryState::Unloaded(bytes))
    }
    
    pub fn path(&self) -> &str {
        &self.path
    }
    
    pub fn get_loaded_data(&self) -> Option<&DynamicImage> {
        self.data.as_loaded()
    }
}

impl Binary for Texture {
    type Error = ImageError;
    type LoadedData = DynamicImage;
    
    fn state(&self) -> &BinaryState<Self::LoadedData> {
        &self.data
    }
    
    fn state_mut(&mut self) -> &mut BinaryState<Self::LoadedData> {
        &mut self.data
    }
    
    fn from_bytes(path: String, bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let data = image::load_from_memory(&bytes)?;
        Ok(Texture {
            path,
            data: BinaryState::Loaded(data),
        })
    }
    
    fn from_uint8_array(path: String, array: &Uint8Array) -> Result<Self, Self::Error> {
        let bytes = array.to_vec();
        Self::from_bytes(path, bytes)
    }
    
    fn from_array_buffer(path: String, buffer: &ArrayBuffer) -> Result<Self, Self::Error> {
        let uint8_array = Uint8Array::new(buffer);
        Self::from_uint8_array(path, &uint8_array)
    }
    
    fn from_bytes_unloaded(path: String, bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Texture {
            path,
            data: BinaryState::Unloaded(bytes),
        })
    }
    
    fn from_uint8_array_unloaded(path: String, array: &Uint8Array) -> Result<Self, Self::Error> {
        let bytes = array.to_vec();
        Self::from_bytes_unloaded(path, bytes)
    }
    
    fn from_array_buffer_unloaded(path: String, buffer: &ArrayBuffer) -> Result<Self, Self::Error> {
        let uint8_array = Uint8Array::new(buffer);
        Self::from_uint8_array_unloaded(path, &uint8_array)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        match &self.data {
            BinaryState::Loaded(img) => {
                let mut bytes = Vec::new();
                img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)
                    .map_err(|_| ImageError::Encoding(image::error::EncodingError::new(
                        image::ImageFormat::Png.into(),
                        "Failed to encode image"
                    )))?;
                Ok(bytes)
            }
            BinaryState::Unloaded(bytes) => Ok(bytes.clone()),
            BinaryState::Empty => Err(ImageError::Parameter(image::error::ParameterError::from_kind(
                image::error::ParameterErrorKind::Generic("No image data available".to_string())
            ))),
        }
    }
    
    fn to_uint8_array(&self) -> Result<Uint8Array, Self::Error> {
        let bytes = self.to_bytes()?;
        Ok(Uint8Array::from(&bytes[..]))
    }
    
    fn to_array_buffer(&self) -> Result<ArrayBuffer, Self::Error> {
        let bytes = self.to_bytes()?;
        let uint8_array = Uint8Array::from(&bytes[..]);
        Ok(uint8_array.buffer())
    }
    
    fn set_from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), Self::Error> {
        let img = image::load_from_memory(&bytes)?;
        self.data = BinaryState::Loaded(img);
        Ok(())
    }
    
    fn set_from_uint8_array(&mut self, array: &Uint8Array) -> Result<(), Self::Error> {
        let bytes = array.to_vec();
        self.set_from_bytes(bytes)
    }
    
    fn set_from_array_buffer(&mut self, buffer: &ArrayBuffer) -> Result<(), Self::Error> {
        let uint8_array = Uint8Array::new(buffer);
        self.set_from_uint8_array(&uint8_array)
    }
    
    fn set_from_bytes_unloaded(&mut self, bytes: Vec<u8>) -> Result<(), Self::Error> {
        self.data = BinaryState::Unloaded(bytes);
        Ok(())
    }
    
    fn set_from_uint8_array_unloaded(&mut self, array: &Uint8Array) -> Result<(), Self::Error> {
        let bytes = array.to_vec();
        self.set_from_bytes_unloaded(bytes)
    }
    
    fn set_from_array_buffer_unloaded(&mut self, buffer: &ArrayBuffer) -> Result<(), Self::Error> {
        let uint8_array = Uint8Array::new(buffer);
        self.set_from_uint8_array_unloaded(&uint8_array)
    }
    
    fn load(&mut self) -> Result<(), Self::Error> {
        match &self.data {
            BinaryState::Unloaded(bytes) => {
                let img = image::load_from_memory(bytes)?;
                self.data = BinaryState::Loaded(img);
                Ok(())
            }
            BinaryState::Loaded(_) => Ok(()),
            BinaryState::Empty => Err(ImageError::Parameter(image::error::ParameterError::from_kind(
                image::error::ParameterErrorKind::Generic("No data to load".to_string())
            ))),
        }
    }
    
    fn unload(&mut self) -> Result<(), Self::Error> {
        match &self.data {
            BinaryState::Loaded(img) => {
                let mut bytes = Vec::new();
                img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)
                    .map_err(|_| ImageError::Encoding(image::error::EncodingError::new(
                        image::ImageFormat::Png.into(),
                        "Failed to encode image"
                    )))?;
                self.data = BinaryState::Unloaded(bytes);
                Ok(())
            }
            BinaryState::Unloaded(_) => Ok(()),
            BinaryState::Empty => Ok(()),
        }
    }
}