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

#[derive(PartialEq, Clone)]
pub struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}
impl Default for User {
    fn default() -> Self {
        Self { id: 0, email: "zero@rust.com".into(), activated: false }
    }
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
    //bool type is used just for the simplicity. In reality I'd use some custom error type.
    pub fn add(&mut self, user: User) -> bool {
        if self.storage.get(&user.id).is_some() {
            return false
        }
        self.storage.set(user.id, user);
        true
    }
    pub fn update(&mut self, user: User) -> bool {
        if self.storage.get(&user.id).is_none() {
            return false;
        }
        self.storage.set(user.id, user);
        true
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
    pub fn add(&mut self, user: User) -> bool {
        if self.storage.get(&user.id).is_some() {
            return false
        }
        self.storage.set(user.id, user);
        true
    }
    pub fn update(&mut self, user: User) -> bool {
        if self.storage.get(&user.id).is_none() {
            return false;
        }
        self.storage.set(user.id, user);
        true
    }
    pub fn remove(&mut self, user: User) {
        self.storage.remove(&user.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_correctness_add() {
        let mut repo_dyn = UserRepositoryDyn::new(Box::new(StorageImpl(HashMap::new())));
        let mut repo_static = UserRepositoryStatic::new(StorageImpl(HashMap::new()));
        repo_dyn.add(User::default());
        repo_static.add(User::default());
        assert!(repo_dyn.get(&0).unwrap() == repo_static.get(&0).unwrap());
    }
    #[test]
    fn test_correctness_update() {
        let user1 = User{id: 0, email: "2".into(), activated: true};
        let mut repo_dyn = UserRepositoryDyn::new(Box::new(StorageImpl(HashMap::new())));
        let mut repo_static = UserRepositoryStatic::new(StorageImpl(HashMap::new()));
        repo_dyn.add(User::default());
        repo_static.add(User::default());
        assert!(repo_dyn.update(user1.clone()) == repo_static.update(user1));
        assert!(repo_dyn.get(&0).unwrap().email == repo_static.get(&0).unwrap().email);
    }

    fn test_correctness_remove() {
        let mut repo_dyn = UserRepositoryDyn::new(Box::new(StorageImpl(HashMap::new())));
        let mut repo_static = UserRepositoryStatic::new(StorageImpl(HashMap::new()));
        repo_dyn.add(User::default());
        repo_static.add(User::default());
        assert!(repo_dyn.get(&0).unwrap() == repo_static.get(&0).unwrap());
        let user1 = User{id: 1, email: "1".into(), activated: false};
        repo_dyn.add(user1.clone());
        repo_static.add(user1.clone());
        repo_dyn.remove(User::default());
        repo_static.remove(User::default());
        assert!(repo_dyn.get(&0).is_none());
        assert!(repo_static.get(&0).is_none());
        
        assert!(repo_dyn.get(&1).unwrap() == &user1);
        assert!(repo_static.get(&1).unwrap() == &user1);
    }
}