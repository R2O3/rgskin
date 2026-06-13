use std::fmt;
use wasm_bindgen::prelude::*;
use image::{ImageError, RgbaImage};
use xxhash_rust::xxh3::xxh3_64;
use crate::{impl_binary_wasm, io::{Binary, BinaryState}};

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct Texture {
    #[wasm_bindgen(getter_with_clone)]
    pub path: String,
    #[wasm_bindgen(skip)]
    pub data: BinaryState<RgbaImage>,
    #[wasm_bindgen(skip)]
    pub hash: Option<u64>,
}

#[wasm_bindgen]
impl Texture {
    #[wasm_bindgen(constructor)]
    pub fn new(path: String) -> Self {
        Texture {
            path,
            data: BinaryState::Empty,
            hash: None,
        }
    }

    #[wasm_bindgen(js_name = fromBlank)]
    pub fn from_blank(path: String) -> Self {
        use image::{ImageBuffer, Rgba};
        let img = ImageBuffer::from_pixel(1, 1, Rgba([0, 0, 0, 0]));
        Texture {
            path,
            data: BinaryState::Loaded(img),
            hash: None,
        }
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_name = getHash)]
    pub fn get_hash_wasm(&self) -> Option<u64> {
        self.hash
    }
}

impl Binary for Texture {
    type Error = ImageError;
    type LoadedData = RgbaImage;

    fn state(&self) -> &BinaryState<Self::LoadedData> {
        &self.data
    }

    fn state_mut(&mut self) -> &mut BinaryState<Self::LoadedData> {
        &mut self.data
    }

    fn get_path(&self) -> &str {
        &self.path
    }

    fn get_hash(&self) -> Option<u64> {
        self.hash
    }

    fn new_with_state(path: String, state: BinaryState<Self::LoadedData>) -> Self {
        Texture { path, data: state, hash: None }
    }

    fn decode_bytes(bytes: &[u8]) -> Result<Self::LoadedData, Self::Error> {
        image::load_from_memory(bytes).map(|img| img.to_rgba8())
    }

    fn encode_to_bytes(data: &Self::LoadedData) -> Result<Vec<u8>, Self::Error> {
        let mut bytes = Vec::new();
        data.write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )
        .map_err(|_| {
            ImageError::Encoding(image::error::EncodingError::new(
                image::ImageFormat::Png.into(),
                "Failed to encode image",
            ))
        })?;
        Ok(bytes)
    }

    fn empty_error() -> Self::Error {
        ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::Generic("No image data available".to_string()),
        ))
    }

    fn from_bytes(path: String, bytes: &[u8]) -> Result<Self, Self::Error> {
        let hash = xxh3_64(bytes);
        let data = Self::decode_bytes(bytes)?;
        Ok(Texture { path, data: BinaryState::Loaded(data), hash: Some(hash) })
    }

    fn from_bytes_unloaded(path: String, bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let hash = xxh3_64(&bytes);
        Ok(Texture { path, data: BinaryState::Unloaded(bytes), hash: Some(hash) })
    }

    fn set_from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), Self::Error> {
        self.hash = Some(xxh3_64(&bytes));
        let data = Self::decode_bytes(&bytes)?;
        *self.state_mut() = BinaryState::Loaded(data);
        Ok(())
    }

    fn set_from_bytes_unloaded(&mut self, bytes: Vec<u8>) -> Result<(), Self::Error> {
        self.hash = Some(xxh3_64(&bytes));
        *self.state_mut() = BinaryState::Unloaded(bytes);
        Ok(())
    }
}

impl Texture {
    pub fn with_data(path: String, data: RgbaImage) -> Self {
        Texture { path, data: BinaryState::Loaded(data), hash: None }
    }

    pub fn with_unloaded_data(path: String, bytes: Vec<u8>) -> Self {
        let hash = xxh3_64(&bytes);
        Texture { path, data: BinaryState::Unloaded(bytes), hash: Some(hash) }
    }

    pub fn get_loaded_data(&self) -> Option<&RgbaImage> {
        self.get_data()
    }
}

impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Texture")
            .field("path", &self.path)
            .field("has_data", &self.data.has_data())
            .field("hash", &self.hash.map(|h| format!("{h:016x}")))
            .finish()
    }
}

impl_binary_wasm!(Texture, "Failed to process texture");