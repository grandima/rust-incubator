use std::{borrow::Cow, hash::Hash, collections::HashMap};
pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}
pub struct StorageImpl<K: Eq+Hash, V>(HashMap<K, V>);
impl <K : Eq + Hash, V> StorageImpl<K, V> {
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
pub struct UserRepository<K, V> {
    storage: dyn Storage<K, V>
}
// impl UserRepository {
//     type Error;
    
//     fn get<'a>(&self, key: &u64) -> Option<&'a User>;
//     fn add(&mut self, user: User) -> Result<(), Self::Error>;
//     fn update(&mut self, user: User) -> Result<(), Self::Error>;
//     // and so on
// }