use std::fmt;
use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageError};
use crate::{impl_binary_wasm, io::{Binary, BinaryState}};

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

    #[wasm_bindgen(js_name = fromBlank)]
    pub fn from_blank(path: String) -> Self {
        use image::{ImageBuffer, Rgba};
        let img = ImageBuffer::from_pixel(1, 1, Rgba([0, 0, 0, 0]));
        let dynamic_img = DynamicImage::ImageRgba8(img);
        Texture { 
            path, 
            data: BinaryState::Loaded(dynamic_img)
        }
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
    
    fn get_path(&self) -> &str {
        &self.path
    }
    
    fn new_with_state(path: String, state: BinaryState<Self::LoadedData>) -> Self {
        Texture { path, data: state }
    }
    
    fn decode_bytes(bytes: &[u8]) -> Result<Self::LoadedData, Self::Error> {
        image::load_from_memory(bytes)
    }
    
    fn encode_to_bytes(data: &Self::LoadedData) -> Result<Vec<u8>, Self::Error> {
        let mut bytes = Vec::new();
        data.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)
            .map_err(|_| ImageError::Encoding(image::error::EncodingError::new(
                image::ImageFormat::Png.into(),
                "Failed to encode image"
            )))?;
        Ok(bytes)
    }
    
    fn empty_error() -> Self::Error {
        ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::Generic("No image data available".to_string())
        ))
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
    
    pub fn get_loaded_data(&self) -> Option<&DynamicImage> {
        self.get_data()
    }
}

impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Texture")
         .field("path", &self.path)
         .field("has data", &self.data.has_data())
         .finish()
    }
}

impl_binary_wasm!(Texture, "Failed to process texture");