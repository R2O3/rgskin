use std::sync::{Arc, RwLock};
use js_sys::{Uint8Array, ArrayBuffer};
use crate::{Binary, BinaryArcExt, BinaryState};

pub trait BinaryArcExtOption<T: Binary> {
    fn has_data(&self) -> bool;
    fn is_loaded(&self) -> bool;
    fn is_unloaded(&self) -> bool;
    fn is_empty(&self) -> bool;

    fn get_path(&self) -> Option<String>;
    fn path_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&str) -> R;
    
    fn get_data(&self) -> Option<T::LoadedData>
    where
        T::LoadedData: Clone;
    
    fn get_unloaded_data(&self) -> Option<Vec<u8>>;
    
    fn set_data(&self, data: T::LoadedData);
    fn set_unloaded_data(&self, bytes: Vec<u8>);
    fn clear_data(&self);
    
    fn replace_data(&self, data: T::LoadedData) -> Option<BinaryState<T::LoadedData>>;
    fn replace_unloaded_data(&self, bytes: Vec<u8>) -> Option<BinaryState<T::LoadedData>>;
    fn take_data(&self) -> Option<T::LoadedData>;
    fn take_unloaded_data(&self) -> Option<Vec<u8>>;
    
    fn load_from_bytes(&self, bytes: Vec<u8>) -> Option<Result<(), T::Error>>;
    fn load_from_bytes_unloaded(&self, bytes: Vec<u8>) -> Option<Result<(), T::Error>>;
    fn load_from_uint8_array(&self, array: &Uint8Array) -> Option<Result<(), T::Error>>;
    fn load_from_uint8_array_unloaded(&self, array: &Uint8Array) -> Option<Result<(), T::Error>>;
    fn load_from_array_buffer(&self, buffer: &ArrayBuffer) -> Option<Result<(), T::Error>>;
    fn load_from_array_buffer_unloaded(&self, buffer: &ArrayBuffer) -> Option<Result<(), T::Error>>;
    
    fn to_bytes(&self) -> Option<Result<Vec<u8>, T::Error>>;
    fn to_uint8_array(&self) -> Option<Result<Uint8Array, T::Error>>;
    fn to_array_buffer(&self) -> Option<Result<ArrayBuffer, T::Error>>;
    
    fn load(&self) -> Option<Result<(), T::Error>>;
    fn unload(&self) -> Option<Result<(), T::Error>>;
    
    fn clone_data(&self) -> Option<T::LoadedData>
    where
        T::LoadedData: Clone;
    
    fn clone_unloaded_data(&self) -> Option<Vec<u8>>;
    
    fn state_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&BinaryState<T::LoadedData>) -> R;
    
    fn state_mut<F, R>(&self, f: F) -> Option<R>
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
    
    fn with_data<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T::LoadedData) -> R;
    
    fn with_unloaded<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Vec<u8>) -> R;
}

impl<T> BinaryArcExtOption<T> for Option<Arc<RwLock<T>>>
where
    T: Binary,
{
    fn has_data(&self) -> bool {
        self.as_ref().map_or(false, |arc| arc.has_data())
    }
    
    fn is_loaded(&self) -> bool {
        self.as_ref().map_or(false, |arc| arc.is_loaded())
    }
    
    fn is_unloaded(&self) -> bool {
        self.as_ref().map_or(false, |arc| arc.is_unloaded())
    }
    
    fn is_empty(&self) -> bool {
        self.as_ref().map_or(true, |arc| arc.is_empty())
    }

    fn get_path(&self) -> Option<String> {
        self.as_ref().map(|arc| arc.get_path())
    }
    
    fn path_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&str) -> R,
    {
        self.as_ref().map(|arc| arc.path_ref(f))
    }
    
    fn get_data(&self) -> Option<T::LoadedData>
    where
        T::LoadedData: Clone,
    {
        self.as_ref().and_then(|arc| arc.get_data())
    }
    
    fn get_unloaded_data(&self) -> Option<Vec<u8>> {
        self.as_ref().and_then(|arc| arc.get_unloaded_data())
    }
    
    fn set_data(&self, data: T::LoadedData) {
        if let Some(arc) = self.as_ref() {
            arc.set_data(data);
        }
    }
    
    fn set_unloaded_data(&self, bytes: Vec<u8>) {
        if let Some(arc) = self.as_ref() {
            arc.set_unloaded_data(bytes);
        }
    }
    
    fn clear_data(&self) {
        if let Some(arc) = self.as_ref() {
            arc.clear_data();
        }
    }
    
    fn replace_data(&self, data: T::LoadedData) -> Option<BinaryState<T::LoadedData>> {
        self.as_ref().map(|arc| arc.replace_data(data))
    }
    
    fn replace_unloaded_data(&self, bytes: Vec<u8>) -> Option<BinaryState<T::LoadedData>> {
        self.as_ref().map(|arc| arc.replace_unloaded_data(bytes))
    }
    
    fn take_data(&self) -> Option<T::LoadedData> {
        self.as_ref().and_then(|arc| arc.take_data())
    }
    
    fn take_unloaded_data(&self) -> Option<Vec<u8>> {
        self.as_ref().and_then(|arc| arc.take_unloaded_data())
    }
    
    fn load_from_bytes(&self, bytes: Vec<u8>) -> Option<Result<(), T::Error>> {
        self.as_ref().map(|arc| arc.load_from_bytes(bytes))
    }
    
    fn load_from_bytes_unloaded(&self, bytes: Vec<u8>) -> Option<Result<(), T::Error>> {
        self.as_ref().map(|arc| arc.load_from_bytes_unloaded(bytes))
    }
    
    fn load_from_uint8_array(&self, array: &Uint8Array) -> Option<Result<(), T::Error>> {
        self.as_ref().map(|arc| arc.load_from_uint8_array(array))
    }
    
    fn load_from_uint8_array_unloaded(&self, array: &Uint8Array) -> Option<Result<(), T::Error>> {
        self.as_ref().map(|arc| arc.load_from_uint8_array_unloaded(array))
    }
    
    fn load_from_array_buffer(&self, buffer: &ArrayBuffer) -> Option<Result<(), T::Error>> {
        self.as_ref().map(|arc| arc.load_from_array_buffer(buffer))
    }
    
    fn load_from_array_buffer_unloaded(&self, buffer: &ArrayBuffer) -> Option<Result<(), T::Error>> {
        self.as_ref().map(|arc| arc.load_from_array_buffer_unloaded(buffer))
    }
    
    fn to_bytes(&self) -> Option<Result<Vec<u8>, T::Error>> {
        self.as_ref().map(|arc| arc.to_bytes())
    }
    
    fn to_uint8_array(&self) -> Option<Result<Uint8Array, T::Error>> {
        self.as_ref().map(|arc| arc.to_uint8_array())
    }
    
    fn to_array_buffer(&self) -> Option<Result<ArrayBuffer, T::Error>> {
        self.as_ref().map(|arc| arc.to_array_buffer())
    }
    
    fn load(&self) -> Option<Result<(), T::Error>> {
        self.as_ref().map(|arc| arc.load())
    }
    
    fn unload(&self) -> Option<Result<(), T::Error>> {
        self.as_ref().map(|arc| arc.unload())
    }
    
    fn clone_data(&self) -> Option<T::LoadedData>
    where
        T::LoadedData: Clone,
    {
        self.as_ref().and_then(|arc| arc.clone_data())
    }
    
    fn clone_unloaded_data(&self) -> Option<Vec<u8>> {
        self.as_ref().and_then(|arc| arc.clone_unloaded_data())
    }
    
    fn state_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&BinaryState<T::LoadedData>) -> R,
    {
        self.as_ref().map(|arc| arc.state_ref(f))
    }
    
    fn state_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut BinaryState<T::LoadedData>) -> R,
    {
        self.as_ref().map(|arc| arc.state_mut(f))
    }
    
    fn data_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T::LoadedData) -> R,
    {
        self.as_ref().and_then(|arc| arc.data_ref(f))
    }
    
    fn data_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T::LoadedData) -> R,
    {
        self.as_ref().and_then(|arc| arc.data_mut(f))
    }
    
    fn unloaded_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Vec<u8>) -> R,
    {
        self.as_ref().and_then(|arc| arc.unloaded_ref(f))
    }
    
    fn unloaded_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Vec<u8>) -> R,
    {
        self.as_ref().and_then(|arc| arc.unloaded_mut(f))
    }
    
    fn with_data<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T::LoadedData) -> R,
    {
        self.as_ref().and_then(|arc| {
            let guard = arc.read().unwrap();
            guard.state().as_loaded().map(f)
        })
    }
    
    fn with_unloaded<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Vec<u8>) -> R,
    {
        self.as_ref().and_then(|arc| {
            let guard = arc.read().unwrap();
            guard.state().as_unloaded().map(f)
        })
    }
}
