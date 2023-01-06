use std::{fmt, pin::Pin};
pub trait MutMeSomehow {
fn mut_me_somehow(self: Pin<&mut Self>);
}
pub trait SayHi: fmt::Debug {
fn say_hi(self: Pin<&Self>) {
    println!("Hi from {:?}", self)
}
}
impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        self.set(&[2 as u8, 3, 4]);
    }
}
impl SayHi for &[u8] {}
