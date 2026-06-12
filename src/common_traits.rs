use std::{collections::HashMap, hash::Hash};

use dashmap::DashMap;

pub trait MapOps<K: Eq + Hash, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn contains_key(&self, key: &K) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
    fn retain<F: FnMut(&K, &mut V) -> bool>(&mut self, f: F);
    fn clone_value(&self, key: &K) -> Option<V> where V: Clone;
    fn for_each_entry<F: FnMut(&K, &V)>(&self, f: F);
    fn for_each_entry_mut<F: FnMut(&K, &mut V)>(&mut self, f: F);
    fn keys_cloned(&self) -> Vec<K> where K: Clone;
}

impl<K: Eq + Hash, V> MapOps<K, V> for HashMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        HashMap::insert(self, key, value)
    }
    
    fn get(&self, key: &K) -> Option<&V> {
        HashMap::get(self, key)
    }
    
    fn remove(&mut self, key: &K) -> Option<V> {
        HashMap::remove(self, key)
    }
    
    fn contains_key(&self, key: &K) -> bool {
        HashMap::contains_key(self, key)
    }
    
    fn len(&self) -> usize {
        HashMap::len(self)
    }
    
    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }
    
    fn clear(&mut self) {
        HashMap::clear(self)
    }
    
    fn retain<F: FnMut(&K, &mut V) -> bool>(&mut self, f: F) {
        HashMap::retain(self, f)
    }
    
    fn clone_value(&self, key: &K) -> Option<V> where V: Clone {
        self.get(key).cloned()
    }

    fn for_each_entry<F: FnMut(&K, &V)>(&self, mut f: F) {
        for (k, v) in self.iter() {
            f(k, v);
        }
    }

    fn for_each_entry_mut<F: FnMut(&K, &mut V)>(&mut self, mut f: F) {
        for (k, v) in self.iter_mut() {
            f(k, v);
        }
    }

    fn keys_cloned(&self) -> Vec<K> where K: Clone {
        self.keys().cloned().collect()
    }
}

impl<K: Eq + Hash, V> MapOps<K, V> for DashMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        DashMap::insert(self, key, value)
    }
    
    fn get(&self, _key: &K) -> Option<&V> {
        // DashMap.get returns Ref guard we cannot safely return &V
        None
    }
    
    fn remove(&mut self, key: &K) -> Option<V> {
        DashMap::remove(self, key).map(|(_, v)| v)
    }
    
    fn contains_key(&self, key: &K) -> bool {
        DashMap::contains_key(self, key)
    }
    
    fn len(&self) -> usize {
        DashMap::len(self)
    }
    
    fn is_empty(&self) -> bool {
        DashMap::is_empty(self)
    }
    
    fn clear(&mut self) {
        DashMap::clear(self)
    }
    
    fn retain<F: FnMut(&K, &mut V) -> bool>(&mut self, mut f: F) {
        DashMap::retain(self, |k, v| f(k, v))
    }
    
    fn clone_value(&self, key: &K) -> Option<V> where V: Clone {
        self.get(key).map(|v| v.clone())
    }

    fn for_each_entry<F: FnMut(&K, &V)>(&self, mut f: F) {
        for entry in self.iter() {
            f(entry.key(), entry.value());
        }
    }

    fn for_each_entry_mut<F: FnMut(&K, &mut V)>(&mut self, mut f: F) {
        for mut entry in self.iter_mut() {
            let (k, v) = entry.pair_mut();
            f(k, v);
        }
    }

    fn keys_cloned(&self) -> Vec<K> where K: Clone {
        self.iter().map(|entry| entry.key().clone()).collect()
    }
}
