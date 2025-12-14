use js_sys::{Uint8Array, ArrayBuffer};

#[derive(Clone, Debug)]
pub enum BinaryState<T> {
    Empty,
    Unloaded(Vec<u8>),
    Loaded(T),
}

impl<T> Default for BinaryState<T> {
    fn default() -> Self {
        BinaryState::Empty
    }
}

impl<T> BinaryState<T> {
    pub fn is_empty(&self) -> bool {
        matches!(self, BinaryState::Empty)
    }

    pub fn is_unloaded(&self) -> bool {
        matches!(self, BinaryState::Unloaded(_))
    }

    pub fn is_loaded(&self) -> bool {
        matches!(self, BinaryState::Loaded(_))
    }

    pub fn has_data(&self) -> bool {
        !self.is_empty()
    }

    pub fn as_loaded(&self) -> Option<&T> {
        match self {
            BinaryState::Loaded(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_unloaded(&self) -> Option<&Vec<u8>> {
        match self {
            BinaryState::Unloaded(bytes) => Some(bytes),
            _ => None,
        }
    }

    pub fn take_loaded(&mut self) -> Option<T> {
        match std::mem::replace(self, BinaryState::Empty) {
            BinaryState::Loaded(data) => Some(data),
            other => {
                *self = other;
                None
            }
        }
    }

    pub fn take_unloaded(&mut self) -> Option<Vec<u8>> {
        match std::mem::replace(self, BinaryState::Empty) {
            BinaryState::Unloaded(bytes) => Some(bytes),
            other => {
                *self = other;
                None
            }
        }
    }
}

pub trait Binary {
    type Error;
    type LoadedData;
    
    fn state(&self) -> &BinaryState<Self::LoadedData>;
    fn state_mut(&mut self) -> &mut BinaryState<Self::LoadedData>;
    
    fn from_bytes(path: String, bytes: Vec<u8>) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn from_uint8_array(path: String, array: &Uint8Array) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn from_array_buffer(path: String, buffer: &ArrayBuffer) -> Result<Self, Self::Error>
    where
        Self: Sized;
    
    fn from_bytes_unloaded(path: String, bytes: Vec<u8>) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn from_uint8_array_unloaded(path: String, array: &Uint8Array) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn from_array_buffer_unloaded(path: String, buffer: &ArrayBuffer) -> Result<Self, Self::Error>
    where
        Self: Sized;
    
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error>;
    fn to_uint8_array(&self) -> Result<Uint8Array, Self::Error>;
    fn to_array_buffer(&self) -> Result<ArrayBuffer, Self::Error>;
    
    fn set_from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), Self::Error>;
    fn set_from_uint8_array(&mut self, array: &Uint8Array) -> Result<(), Self::Error>;
    fn set_from_array_buffer(&mut self, buffer: &ArrayBuffer) -> Result<(), Self::Error>;
    
    fn set_from_bytes_unloaded(&mut self, bytes: Vec<u8>) -> Result<(), Self::Error>;
    fn set_from_uint8_array_unloaded(&mut self, array: &Uint8Array) -> Result<(), Self::Error>;
    fn set_from_array_buffer_unloaded(&mut self, buffer: &ArrayBuffer) -> Result<(), Self::Error>;
    
    fn load(&mut self) -> Result<(), Self::Error>;
    fn unload(&mut self) -> Result<(), Self::Error>;
    
    fn is_loaded(&self) -> bool {
        self.state().is_loaded()
    }
    
    fn is_unloaded(&self) -> bool {
        self.state().is_unloaded()
    }
    
    fn is_empty(&self) -> bool {
        self.state().is_empty()
    }
    
    fn has_data(&self) -> bool {
        self.state().has_data()
    }
}
