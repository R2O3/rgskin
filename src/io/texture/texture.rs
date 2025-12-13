use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array, ArrayBuffer};
use image::{DynamicImage, ImageError};
use crate::io::Binary;

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct Texture {
    #[wasm_bindgen(getter_with_clone)]
    pub path: String,
    #[wasm_bindgen(skip)]
    pub data: Option<DynamicImage>,
}

#[wasm_bindgen]
impl Texture {
    #[wasm_bindgen(constructor)]
    pub fn new(path: String) -> Self {
        Texture { path, data: None }
    }

    #[wasm_bindgen(js_name = fromArrayBuffer)]
    pub fn from_array_buffer_wasm(path: String, buffer: &ArrayBuffer) -> Result<Texture, JsValue> {
        Self::from_array_buffer(path, buffer)
            .map_err(|e| JsValue::from_str(&format!("Failed to load texture: {}", e)))
    }
    
    #[wasm_bindgen(js_name = fromUint8Array)]
    pub fn from_uint8_array_wasm(path: String, array: &Uint8Array) -> Result<Texture, JsValue> {
        Self::from_uint8_array(path, array)
            .map_err(|e| JsValue::from_str(&format!("Failed to load texture: {}", e)))
    }

    #[wasm_bindgen(js_name = getPath)]
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    #[wasm_bindgen(js_name = hasData)]
    pub fn has_data_wasm(&self) -> bool {
        self.has_data()
    }
}

impl Texture {
    pub fn with_data(path: String, data: DynamicImage) -> Self {
        Texture { path, data: Some(data) }
    }
    
    pub fn empty(path: String) -> Self {
        use image::{ImageBuffer, Rgba};
        let img = ImageBuffer::from_pixel(1, 1, Rgba([0, 0, 0, 0]));
        let dynamic_img = DynamicImage::ImageRgba8(img);
        Texture { path, data: Some(dynamic_img) }
    }
    
    pub fn from_array_buffer(path: String, buffer: &ArrayBuffer) -> Result<Self, ImageError> {
        <Self as Binary>::from_array_buffer(path, buffer)
    }
    
    pub fn from_uint8_array(path: String, array: &Uint8Array) -> Result<Self, ImageError> {
        <Self as Binary>::from_uint8_array(path, array)
    }
    
    pub fn from_bytes(path: String, bytes: Vec<u8>) -> Result<Self, ImageError> {
        <Self as Binary>::from_bytes(path, bytes)
    }

    pub fn set_data(&mut self, data: DynamicImage) {
        self.data = Some(data);
    }
    
    pub fn clear(&mut self) {
        self.data = None;
    }
    
    pub fn take_data(&mut self) -> Option<DynamicImage> {
        self.data.take()
    }
    
    pub fn replace_data(&mut self, data: DynamicImage) -> Option<DynamicImage> {
        self.data.replace(data)
    }
    
    pub fn path(&self) -> &str {
        &self.path
    }
    
    pub fn data(&self) -> &Option<DynamicImage> {
        &self.data
    }
    
    pub fn data_mut(&mut self) -> &mut Option<DynamicImage> {
        &mut self.data
    }
    
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

impl Binary for Texture {
    type Error = ImageError;
    
    fn from_bytes(path: String, bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let data = image::load_from_memory(&bytes)?;
        Ok(Texture {
            path,
            data: Some(data),
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
    
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        if let Some(ref img) = self.data {
            let mut bytes = Vec::new();
            img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)
                .map_err(|_| ImageError::Encoding(image::error::EncodingError::new(
                    image::ImageFormat::Png.into(),
                    "Failed to encode image"
                )))?;
            Ok(bytes)
        } else {
            Err(ImageError::Parameter(image::error::ParameterError::from_kind(
                image::error::ParameterErrorKind::Generic("No image data available".to_string())
            )))
        }
    }
    
    fn to_uint8_array(&self) -> Result<Uint8Array, Self::Error> {
        let bytes = self.data.as_ref().unwrap().as_bytes();
        Ok(Uint8Array::from(bytes))
    }
    
    fn to_array_buffer(&self) -> Result<ArrayBuffer, Self::Error> {
        let bytes = self.data.as_ref().unwrap().as_bytes();
        let uint8_array = Uint8Array::from(bytes);
        Ok(uint8_array.buffer())
    }
    
    fn set_from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), Self::Error> {
        let img = image::load_from_memory(&bytes)?;
        self.data = Some(img);
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
}