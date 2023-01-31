use std::{borrow::Cow, hash::Hash, collections::HashMap};
pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}
pub struct StorageImpl<K: Eq+Hash, V>(pub HashMap<K, V>);
impl <K : Eq + Hash, V> Storage<K, V> for StorageImpl<K, V> {
    fn set(&mut self, key: K, val: V) {
        self.0.insert(key, val);
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }
}

pub struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}
pub struct UserRepositoryDyn {
    storage: Box<dyn Storage<u64, User>>
}
impl UserRepositoryDyn {
    pub fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        Self {storage: storage}
    }
    pub fn get(&self, key: &u64) -> Option<&User> {
        self.storage.get(key)
    }
    pub fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }
    pub fn update(&mut self, user: User) {
        self.storage.set(user.id, user);
    }
    pub fn remove(&mut self, user: User) {
        self.storage.remove(&user.id);
    }
}

pub struct UserRepositoryStatic {
    storage: StorageImpl<u64, User>
}
impl UserRepositoryStatic {
    pub fn new(storage: StorageImpl<u64, User>) -> Self {
        Self {storage: storage}
    }
    pub fn get(&self, key: &u64) -> Option<&User> {
        self.storage.get(key)
    }
    pub fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }
    pub fn update(&mut self, user: User) {
        self.storage.set(user.id, user);
    }
    pub fn remove(&mut self, user: User) {
        self.storage.remove(&user.id);
    }
}