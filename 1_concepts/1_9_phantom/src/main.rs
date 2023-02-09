use std::marker::PhantomData;

use rand::{seq::SliceRandom, thread_rng, Rng};

struct Fact<T> {
    _marker: PhantomData<T>,
}
impl<T> Fact<T> {
    fn new() -> Fact<T> {
        Self {
            _marker: PhantomData,
        }
    }
}
impl<T> Fact<Vec<T>> {
    fn fact(&self) -> &'static str {
        ["Fact about Vec: Vec is heap-allocated.", 
            "Fact about Vec: Vec may re-allocate on growing.", 
            "It can be converted to and from other data structures, such as slices and arrays.", 
            "Provides many convenient methods for manipulating and transforming its elements, such as push, pop, sort, iter, and more."
            ].choose(&mut thread_rng()).unwrap()
    }
}

impl Fact<String> {
    fn fact(&self) -> &'static str {
        ["Rust's String type is a UTF-8 encoded, growable string type.",
"Unlike C-style strings, Rust's String type is safe and efficient to use, as it manages its own memory and automatically deallocates when it goes out of scope.",
"The String type provides many convenient methods for manipulating and transforming its contents, such as push_str, trim, split, and more."
        ].choose(&mut thread_rng()).unwrap()
    }
}
impl Fact<i32> {
    fn fact(&self) -> &'static str {
        ["i32 is a signed integer type in Rust, with a range of values from -2^31 to 2^31-1.",
            "i32 has a fixed size of 32 bits, which means it requires 4 bytes of memory to store a value.",
            "The i32 type implements a number of important traits, such as Copy, Add, Sub, Mul, Div, and more, which makes it easy to use in a wide range of mathematical and numerical operations."
        ].choose(&mut thread_rng()).unwrap()
    }
}

fn main() {
    println!("{}", Fact::<Vec<i32>>::new().fact());
    println!("{}", Fact::<i32>::new().fact());
    println!("{}", Fact::<String>::new().fact());
}
