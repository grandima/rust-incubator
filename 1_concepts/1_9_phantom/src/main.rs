use std::{marker::PhantomData};

use rand::Rng;

struct Fact<T> {
    _marker: PhantomData<T>,
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
        match rand::thread_rng().gen_range(1..4) {
            1 => "Fact about Vec: Vec is heap-allocated.",
            2 => "Fact about Vec: Vec may re-allocate on growing.",
            3 => "It can be converted to and from other data structures, such as slices and arrays.",
            _ => "Provides many convenient methods for manipulating and transforming its elements, such as push, pop, sort, iter, and more."
        }.to_string()
    }
}

impl Fact<String> {
    fn fact(&self) -> String {
        match rand::thread_rng().gen_range(1..3) {
            1 => "Rust's String type is a UTF-8 encoded, growable string type.",
            2 => "Unlike C-style strings, Rust's String type is safe and efficient to use, as it manages its own memory and automatically deallocates when it goes out of scope.",
            _ => "The String type provides many convenient methods for manipulating and transforming its contents, such as push_str, trim, split, and more."
        }.to_string()
    }
}
impl Fact<i32> {
    fn fact(&self) -> String {
        match rand::thread_rng().gen_range(1..3) {
            1 => "i32 is a signed integer type in Rust, with a range of values from -2^31 to 2^31-1.",
            2 => "i32 has a fixed size of 32 bits, which means it requires 4 bytes of memory to store a value.",
            _ => "The i32 type implements a number of important traits, such as Copy, Add, Sub, Mul, Div, and more, which makes it easy to use in a wide range of mathematical and numerical operations."
        }.to_string()
    }
}

fn main() {
    println!("{}", Fact::<Vec<i32>>::new().fact());
    println!("{}", Fact::<i32>::new().fact());
    println!("{}", Fact::<String>::new().fact());
}
