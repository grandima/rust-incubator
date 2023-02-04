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
trait Command {}
struct CreateUser {
    should_deactivate_in_storage: bool, 
}
impl Command for CreateUser {
    
}
trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &mut Self::Context) -> Self::Result;
}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = bool;
    
    fn handle_command(&self, cmd: &CreateUser, ctx: &mut Self::Context) -> Self::Result {
        if cmd.should_deactivate_in_storage {
            let mut user = self.clone();
            user.activated = false;
            ctx.add(user);
        } else {
            ctx.add(self.clone());
        }
        true
    }
}
pub trait UserRepository {
    fn get(&self, key: &u64) -> Option<&User>;
    fn add(&mut self, user: User) -> bool;
}
pub struct UserRepositoryImpl<S: Storage<u64, User>> {
    storage: S
}

impl <S: Storage<u64, User>>UserRepository for UserRepositoryImpl<S> {
    fn get(&self, key: &u64) -> Option<&User> {
        self.storage.get(key)
    }
    fn add(&mut self, user: User) -> bool {
        self.storage.set(user.id, user);
        true
    }
}

pub struct MockStorageImpl<K: Eq+Hash, V>(pub Vec<(K, V)>);
impl <K : Eq + Hash, V> Storage<K, V> for MockStorageImpl<K, V> {
    fn set(&mut self, key: K, val: V) {
        if let Some(pair) = self.0.iter_mut().find(| p|p.0 == key) {
            pair.1 = val;
        } else {
            self.0.push((key, val));
        }
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.0.iter().find(|x|&x.0 == key).map(|x|&x.1)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        todo!("We don't need remove now")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mocked() {
        let mut mocked_storage: MockStorageImpl<u64, User> = MockStorageImpl(vec![]);
        let mut user_repo = UserRepositoryImpl{storage: mocked_storage};
        let c = CreateUser{ should_deactivate_in_storage: false};
        let user = User{id: 1, email: "aa@gmail.com".into(), activated: true};
        user.handle_command(&c, &mut user_repo);
        assert!(user_repo.get(&1).unwrap().activated);
        let c = CreateUser{ should_deactivate_in_storage: true};
        user.handle_command(&c, &mut user_repo);
        assert!(!user_repo.get(&1).unwrap().activated);
    }
}
fn main() {
    println!("Implement me!");
}
