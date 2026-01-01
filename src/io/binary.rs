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

#[macro_export]
macro_rules! impl_binary_wasm {
    ($type:ty, $error_msg:expr) => {
        #[allow(unused_imports)]
        use js_sys::{ArrayBuffer, Uint8Array};

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen]
        impl $type {
            #[wasm_bindgen(js_name = fromArrayBuffer)]
            pub fn from_array_buffer_wasm(path: String, buffer: &ArrayBuffer) -> Result<$type, JsValue> {
                <$type as Binary>::from_array_buffer(path, buffer)
                    .map_err(|e| JsValue::from_str(&format!("{}: {}", $error_msg, e)))
            }
            
            #[wasm_bindgen(js_name = fromArrayBufferUnloaded)]
            pub fn from_array_buffer_unloaded_wasm(path: String, buffer: &ArrayBuffer) -> Result<$type, JsValue> {
                <$type as Binary>::from_array_buffer_unloaded(path, buffer)
                    .map_err(|e| JsValue::from_str(&format!("{}: {}", $error_msg, e)))
            }
            
            #[wasm_bindgen(js_name = fromUint8Array)]
            pub fn from_uint8_array_wasm(path: String, array: &Uint8Array) -> Result<$type, JsValue> {
                <$type as Binary>::from_uint8_array(path, array)
                    .map_err(|e| JsValue::from_str(&format!("{}: {}", $error_msg, e)))
            }

            #[wasm_bindgen(js_name = fromUint8ArrayUnloaded)]
            pub fn from_uint8_array_unloaded_wasm(path: String, array: &Uint8Array) -> Result<$type, JsValue> {
                <$type as Binary>::from_uint8_array_unloaded(path, array)
                    .map_err(|e| JsValue::from_str(&format!("{}: {}", $error_msg, e)))
            }

            #[wasm_bindgen(js_name = getPath)]
            pub fn get_path_wasm(&self) -> String {
                <$type as Binary>::get_path(self).to_string()
            }

            #[wasm_bindgen(js_name = hasData)]
            pub fn has_data_wasm(&self) -> bool {
                <$type as Binary>::has_data(self)
            }

            #[wasm_bindgen(js_name = isLoaded)]
            pub fn is_loaded_wasm(&self) -> bool {
                <$type as Binary>::is_loaded(self)
            }

            #[wasm_bindgen(js_name = isUnloaded)]
            pub fn is_unloaded_wasm(&self) -> bool {
                <$type as Binary>::is_unloaded(self)
            }

            #[wasm_bindgen(js_name = isEmpty)]
            pub fn is_empty_wasm(&self) -> bool {
                <$type as Binary>::is_empty(self)
            }

            #[wasm_bindgen(js_name = load)]
            pub fn load_wasm(&mut self) -> Result<(), JsValue> {
                <$type as Binary>::load(self)
                    .map_err(|e| JsValue::from_str(&format!("{}: {}", $error_msg, e)))
            }

            #[wasm_bindgen(js_name = unload)]
            pub fn unload_wasm(&mut self) -> Result<(), JsValue> {
                <$type as Binary>::unload(self)
                    .map_err(|e| JsValue::from_str(&format!("{}: {}", $error_msg, e)))
            }
        }
    };
}

pub trait Binary: Sized {
    type Error;
    type LoadedData;
    
    fn state(&self) -> &BinaryState<Self::LoadedData>;
    fn state_mut(&mut self) -> &mut BinaryState<Self::LoadedData>;
    fn get_path(&self) -> &str;
    fn new_with_state(path: String, state: BinaryState<Self::LoadedData>) -> Self;
    
    fn decode_bytes(bytes: &[u8]) -> Result<Self::LoadedData, Self::Error>;
    fn encode_to_bytes(data: &Self::LoadedData) -> Result<Vec<u8>, Self::Error>;
    
    fn from_bytes(path: String, bytes: &[u8]) -> Result<Self, Self::Error> {
        let data = Self::decode_bytes(bytes)?;
        Ok(Self::new_with_state(path, BinaryState::Loaded(data)))
    }

    fn from_uint8_array(path: String, array: &Uint8Array) -> Result<Self, Self::Error> {
        let bytes = array.to_vec();
        Self::from_bytes(path, &bytes)
    }

    fn from_array_buffer(path: String, buffer: &ArrayBuffer) -> Result<Self, Self::Error> {
        let uint8_array = Uint8Array::new(buffer);
        Self::from_uint8_array(path, &uint8_array)
    }
    
    fn from_bytes_unloaded(path: String, bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::new_with_state(path, BinaryState::Unloaded(bytes)))
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
        match self.state() {
            BinaryState::Loaded(data) => Self::encode_to_bytes(data),
            BinaryState::Unloaded(bytes) => Ok(bytes.clone()),
            BinaryState::Empty => Err(Self::empty_error()),
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
        let data = Self::decode_bytes(&bytes)?;
        *self.state_mut() = BinaryState::Loaded(data);
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
        *self.state_mut() = BinaryState::Unloaded(bytes);
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
        match self.state() {
            BinaryState::Unloaded(bytes) => {
                let data = Self::decode_bytes(bytes)?;
                *self.state_mut() = BinaryState::Loaded(data);
                Ok(())
            }
            BinaryState::Loaded(_) => Ok(()),
            BinaryState::Empty => Err(Self::empty_error()),
        }
    }
    
    fn unload(&mut self) -> Result<(), Self::Error> {
        match self.state() {
            BinaryState::Loaded(data) => {
                let bytes = Self::encode_to_bytes(data)?;
                *self.state_mut() = BinaryState::Unloaded(bytes);
                Ok(())
            }
            BinaryState::Unloaded(_) => Ok(()),
            BinaryState::Empty => Ok(()),
        }
    }
    
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
    
    fn get_data(&self) -> Option<&Self::LoadedData> {
        self.state().as_loaded()
    }
    
    fn get_unloaded_data(&self) -> Option<&Vec<u8>> {
        self.state().as_unloaded()
    }
    
    fn take_data(&mut self) -> Option<Self::LoadedData> {
        self.state_mut().take_loaded()
    }
    
    fn take_unloaded_data(&mut self) -> Option<Vec<u8>> {
        self.state_mut().take_unloaded()
    }
    
    fn set_data(&mut self, data: Self::LoadedData) {
        *self.state_mut() = BinaryState::Loaded(data);
    }
    
    fn set_unloaded_data(&mut self, bytes: Vec<u8>) {
        *self.state_mut() = BinaryState::Unloaded(bytes);
    }
    
    fn replace_data(&mut self, data: Self::LoadedData) -> BinaryState<Self::LoadedData> {
        std::mem::replace(self.state_mut(), BinaryState::Loaded(data))
    }
    
    fn replace_unloaded_data(&mut self, bytes: Vec<u8>) -> BinaryState<Self::LoadedData> {
        std::mem::replace(self.state_mut(), BinaryState::Unloaded(bytes))
    }
    
    fn empty_error() -> Self::Error;
}
