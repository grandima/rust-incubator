use std::{ops::Deref};
use rand::Rng;
pub struct Random<T> {
    a: T, b: T, c: T
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

