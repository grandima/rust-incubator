use std::{ops::Deref};
use rand::Rng;
pub struct Random<T> {
    a: T, b: T, c: T
}
impl <T: PartialEq>PartialEq for Random<T> {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}
impl <T>Deref for Random<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let num = rand::thread_rng().gen_range(0..2);
        match num {
            0 => &self.a,
            1 => &self.b,
            _ => &self.c
        }
    }
}
impl <T>AsRef<T> for Random<T> {
    fn as_ref(&self) -> &T {
        let num = rand::thread_rng().gen_range(0..2);
        match num {
            0 => &self.a,
            1 => &self.b,
            _ => &self.c
        }
    }
}

impl<T: Clone> TryFrom<Vec<T>> for Random<T> {
    type Error = &'static str;
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        match vec.len() == 3 {
            true => Ok(Self{a: vec[0].clone(), b: vec[1].clone(), c: vec[2].clone()}),
            false => Err("bad length")
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_try_from_err() {
        let b = Random::try_from(vec![1, 2, 3, 4]);
        assert_eq!(b.err().unwrap(), "bad length");
    }
    #[test]
    fn test_try_from_true() {
        let l = Random::try_from(vec![1, 2, 3]);
        let r = Random{a: 1, b: 2, c: 3};
        assert!(l.ok().unwrap() == r);
    }
}

