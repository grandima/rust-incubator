use std::{sync::{Arc, Mutex}, vec};

pub struct GlobalStack<T> {
    pub vec: Arc<Mutex<Vec<T>>>
}
impl <T> GlobalStack<T> {
    pub fn new() -> Self {
        Self { vec: Arc::new(Mutex::new(vec![])) }
    }
    pub fn push(&self, val: T) {
        self.vec.lock().unwrap().push(val);
    }
    pub fn pop(&self) -> Option<T> {
        self.vec.lock().unwrap().pop()
    }
}
impl <T>Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        Self { vec: self.vec.clone() }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mut() {
        let s1: GlobalStack<String> = GlobalStack::new();
        let s2 = s1.clone();
        s1.push("a".to_string());
        assert_eq!(s2.pop().unwrap(), "a".to_string());
    }
}