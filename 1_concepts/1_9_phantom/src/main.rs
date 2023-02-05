use std::{marker::PhantomData};

use rand::Rng;

struct Fact<T> {
    _marker: PhantomData<*const T>,
}
impl <T> Fact<T> {
    fn new() -> Fact<T> {
        Self {
            _marker: PhantomData,
        }
    }
}
impl <T> Fact<Vec<T>> {
    fn fact(&self) -> String {
        format!("vec fact: {}", rand::thread_rng().gen_range(1..5)).to_string()
    }
}

impl Fact<String> {
    fn fact(&self) -> String {
        format!("string fact: {}", rand::thread_rng().gen_range(1..5)).to_string()
    }
}
impl Fact<i32> {
    fn fact(&self) -> String {
        let mut rng = rand::thread_rng();
        format!("i32 fact: {}", rng.gen_range(1..4)).to_string()
    }
}

fn main() {
    println!("{}", Fact::<Vec<i32>>::new().fact());
    println!("{}", Fact::<i32>::new().fact());
    println!("{}", Fact::<String>::new().fact());
}
