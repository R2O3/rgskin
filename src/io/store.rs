use std::sync::{Arc, RwLock};

pub trait Store<T> {
    fn insert(&mut self, item: T);
    fn get(&self, path: &str) -> Option<std::sync::RwLockReadGuard<'_, T>>;
    fn get_shared(&self, path: &str) -> Option<Arc<RwLock<T>>>;
    fn get_mut(&self, path: &str) -> Option<std::sync::RwLockWriteGuard<'_, T>>;
    fn contains(&self, path: &str) -> bool;
    fn remove(&mut self, path: &str) -> Option<Arc<RwLock<T>>>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn paths(&self) -> impl Iterator<Item = &str>;
    fn clear(&mut self);
    fn make_unique(&mut self, new_path: String, item: T) -> String;
    fn copy(&mut self, original_path: &str, new_path: String) -> Option<String>;
    fn copy_from_data(&mut self, path: String, data: Self::Data) -> String;
    fn make_unique_copy(&mut self, original_path: &str, new_base_path: String) -> Option<String>;
    fn make_unique_from_data(&mut self, path: String, data: Self::Data) -> String;
    
    type Data;
}
