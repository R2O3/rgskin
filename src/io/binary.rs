use js_sys::{Uint8Array, ArrayBuffer};

pub trait Binary {
    type Error;
    
    fn from_bytes(path: String, bytes: Vec<u8>) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn from_uint8_array(path: String, array: &Uint8Array) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn from_array_buffer(path: String, buffer: &ArrayBuffer) -> Result<Self, Self::Error>
    where
        Self: Sized;
    
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error>;
    fn to_uint8_array(&self) -> Result<Uint8Array, Self::Error>;
    fn to_array_buffer(&self) -> Result<ArrayBuffer, Self::Error>;
    
    fn set_from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), Self::Error>;
    fn set_from_uint8_array(&mut self, array: &Uint8Array) -> Result<(), Self::Error>;
    fn set_from_array_buffer(&mut self, buffer: &ArrayBuffer) -> Result<(), Self::Error>;
}