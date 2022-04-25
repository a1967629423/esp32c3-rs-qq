#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
#![feature(type_alias_impl_trait)]
#![feature(map_first_last)]
#![feature(async_closure)]
#![no_std]
pub mod client;
pub mod ext;
pub mod provider;
pub mod structs;
extern crate alloc;

pub use client::handler;
pub use client::handler::Handler;
pub use client::handler::QEvent;
pub use client::Client;
pub use engine::error::{RQError, RQResult};
use engine::jce;
pub use engine::msg;
pub use engine::protocol::device;
pub use engine::protocol::version;
pub use nrq_engine as engine;
pub use provider::*;

/// 很蠢的缓存，但是我实在是太懒了
pub(crate) struct SimpleCache<K, V, const N: usize = 1> {
    map: alloc::collections::BTreeMap<K, V>,
}
impl<K, V, const N: usize> SimpleCache<K, V, N> {
    pub fn new() -> Self {
        Self {
            map: alloc::collections::BTreeMap::new(),
        }
    }
}
impl<K: Ord + Clone, V, const N: usize> SimpleCache<K, V, N> {
    // pub fn contains_key(&self,key:&K) -> bool {
    //     self.map.contains_key(key)
    // }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
    pub fn get_or_insert_with<F: FnOnce() -> V>(&mut self, key: K, f: F) -> &mut V {
        if self.map.contains_key(&key) {
            return self.map.get_mut(&key).unwrap();
        }
        let value = f();
        self.insert(key.clone(), value);
        self.map.get_mut(&key).unwrap()
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.map.len() >= N {
            let rk = { self.map.last_key_value().unwrap().0.clone() };
            self.map.remove(&rk);
        }
        self.map.insert(key, value)
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
    // pub fn clear(&mut self) {
    //     self.map.clear();
    // }
    // pub fn is_empty(&self) -> bool {
    //     self.map.is_empty()
    // }
}
