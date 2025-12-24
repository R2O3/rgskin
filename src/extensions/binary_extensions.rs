use std::sync::{Arc, RwLock};
use js_sys::{Uint8Array, ArrayBuffer};
use crate::{Binary, BinaryState};

pub trait BinaryArcExt<T: Binary> {
    fn has_data(&self) -> bool;
    fn is_loaded(&self) -> bool;
    fn is_unloaded(&self) -> bool;
    fn is_empty(&self) -> bool;

    fn get_path(&self) -> String;
    fn path_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R;
    
    fn get_data(&self) -> Option<T::LoadedData>
    where
        T::LoadedData: Clone;
    
    fn get_unloaded_data(&self) -> Option<Vec<u8>>;
    
    fn set_data(&self, data: T::LoadedData);
    fn set_unloaded_data(&self, bytes: Vec<u8>);
    fn clear_data(&self);
    
    fn replace_data(&self, data: T::LoadedData) -> BinaryState<T::LoadedData>;
    fn replace_unloaded_data(&self, bytes: Vec<u8>) -> BinaryState<T::LoadedData>;
    fn take_data(&self) -> Option<T::LoadedData>;
    fn take_unloaded_data(&self) -> Option<Vec<u8>>;
    
    fn load_from_bytes(&self, bytes: Vec<u8>) -> Result<(), T::Error>;
    fn load_from_bytes_unloaded(&self, bytes: Vec<u8>) -> Result<(), T::Error>;
    fn load_from_uint8_array(&self, array: &Uint8Array) -> Result<(), T::Error>;
    fn load_from_uint8_array_unloaded(&self, array: &Uint8Array) -> Result<(), T::Error>;
    fn load_from_array_buffer(&self, buffer: &ArrayBuffer) -> Result<(), T::Error>;
    fn load_from_array_buffer_unloaded(&self, buffer: &ArrayBuffer) -> Result<(), T::Error>;
    
    fn to_bytes(&self) -> Result<Vec<u8>, T::Error>;
    fn to_uint8_array(&self) -> Result<Uint8Array, T::Error>;
    fn to_array_buffer(&self) -> Result<ArrayBuffer, T::Error>;
    
    fn load(&self) -> Result<(), T::Error>;
    fn unload(&self) -> Result<(), T::Error>;
    
    fn clone_data(&self) -> Option<T::LoadedData>
    where
        T::LoadedData: Clone;
    
    fn clone_unloaded_data(&self) -> Option<Vec<u8>>;
    
    fn state_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BinaryState<T::LoadedData>) -> R;
    
    fn state_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut BinaryState<T::LoadedData>) -> R;
    
    fn data_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T::LoadedData) -> R;
    
    fn data_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T::LoadedData) -> R;
    
    fn unloaded_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Vec<u8>) -> R;
    
    fn unloaded_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Vec<u8>) -> R;
    
    fn with_data<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T::LoadedData) -> R;
    
    fn with_unloaded<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Vec<u8>) -> R;
}

impl<T> BinaryArcExt<T> for Arc<RwLock<T>>
where
    T: Binary,
{
    fn has_data(&self) -> bool {
        self.read().unwrap().has_data()
    }
    
    fn is_loaded(&self) -> bool {
        self.read().unwrap().is_loaded()
    }
    
    fn is_unloaded(&self) -> bool {
        self.read().unwrap().is_unloaded()
    }
    
    fn is_empty(&self) -> bool {
        self.read().unwrap().is_empty()
    }

    fn get_path(&self) -> String {
        self.read().unwrap().path().to_string()
    }
    
    fn path_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        let guard = self.read().unwrap();
        f(guard.path())
    }
    
    fn get_data(&self) -> Option<T::LoadedData>
    where
        T::LoadedData: Clone,
    {
        self.read().unwrap().state().as_loaded().cloned()
    }
    
    fn get_unloaded_data(&self) -> Option<Vec<u8>> {
        self.read().unwrap().state().as_unloaded().cloned()
    }
    
    fn set_data(&self, data: T::LoadedData) {
        *self.write().unwrap().state_mut() = BinaryState::Loaded(data);
    }
    
    fn set_unloaded_data(&self, bytes: Vec<u8>) {
        *self.write().unwrap().state_mut() = BinaryState::Unloaded(bytes);
    }
    
    fn clear_data(&self) {
        *self.write().unwrap().state_mut() = BinaryState::Empty;
    }
    
    fn replace_data(&self, data: T::LoadedData) -> BinaryState<T::LoadedData> {
        std::mem::replace(
            self.write().unwrap().state_mut(),
            BinaryState::Loaded(data)
        )
    }
    
    fn replace_unloaded_data(&self, bytes: Vec<u8>) -> BinaryState<T::LoadedData> {
        std::mem::replace(
            self.write().unwrap().state_mut(),
            BinaryState::Unloaded(bytes)
        )
    }
    
    fn take_data(&self) -> Option<T::LoadedData> {
        self.write().unwrap().state_mut().take_loaded()
    }
    
    fn take_unloaded_data(&self) -> Option<Vec<u8>> {
        self.write().unwrap().state_mut().take_unloaded()
    }
    
    fn load_from_bytes(&self, bytes: Vec<u8>) -> Result<(), T::Error> {
        self.write().unwrap().set_from_bytes(bytes)
    }
    
    fn load_from_bytes_unloaded(&self, bytes: Vec<u8>) -> Result<(), T::Error> {
        self.write().unwrap().set_from_bytes_unloaded(bytes)
    }
    
    fn load_from_uint8_array(&self, array: &Uint8Array) -> Result<(), T::Error> {
        self.write().unwrap().set_from_uint8_array(array)
    }
    
    fn load_from_uint8_array_unloaded(&self, array: &Uint8Array) -> Result<(), T::Error> {
        self.write().unwrap().set_from_uint8_array_unloaded(array)
    }
    
    fn load_from_array_buffer(&self, buffer: &ArrayBuffer) -> Result<(), T::Error> {
        self.write().unwrap().set_from_array_buffer(buffer)
    }
    
    fn load_from_array_buffer_unloaded(&self, buffer: &ArrayBuffer) -> Result<(), T::Error> {
        self.write().unwrap().set_from_array_buffer_unloaded(buffer)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>, T::Error> {
        self.read().unwrap().to_bytes()
    }
    
    fn to_uint8_array(&self) -> Result<Uint8Array, T::Error> {
        self.read().unwrap().to_uint8_array()
    }
    
    fn to_array_buffer(&self) -> Result<ArrayBuffer, T::Error> {
        self.read().unwrap().to_array_buffer()
    }
    
    fn load(&self) -> Result<(), T::Error> {
        self.write().unwrap().load()
    }
    
    fn unload(&self) -> Result<(), T::Error> {
        self.write().unwrap().unload()
    }
    
    fn clone_data(&self) -> Option<T::LoadedData>
    where
        T::LoadedData: Clone,
    {
        self.read().unwrap().state().as_loaded().cloned()
    }
    
    fn clone_unloaded_data(&self) -> Option<Vec<u8>> {
        self.read().unwrap().state().as_unloaded().cloned()
    }
    
    fn state_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BinaryState<T::LoadedData>) -> R,
    {
        let guard = self.read().unwrap();
        f(guard.state())
    }
    
    fn state_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut BinaryState<T::LoadedData>) -> R,
    {
        let mut guard = self.write().unwrap();
        f(guard.state_mut())
    }
    
    fn data_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T::LoadedData) -> R,
    {
        let guard = self.read().unwrap();
        guard.state().as_loaded().map(f)
    }
    
    fn data_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T::LoadedData) -> R,
    {
        let mut guard = self.write().unwrap();
        match guard.state_mut() {
            BinaryState::Loaded(data) => Some(f(data)),
            _ => None,
        }
    }
    
    fn unloaded_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Vec<u8>) -> R,
    {
        let guard = self.read().unwrap();
        guard.state().as_unloaded().map(f)
    }
    
    fn unloaded_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Vec<u8>) -> R,
    {
        let mut guard = self.write().unwrap();
        match guard.state_mut() {
            BinaryState::Unloaded(bytes) => Some(f(bytes)),
            _ => None,
        }
    }
    
    fn with_data<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T::LoadedData) -> R,
    {
        let guard = self.read().unwrap();
        f(guard.state().as_loaded().expect("Data is not loaded"))
    }
    
    fn with_unloaded<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Vec<u8>) -> R,
    {
        let guard = self.read().unwrap();
        f(guard.state().as_unloaded().expect("Data is not unloaded"))
    }
}
