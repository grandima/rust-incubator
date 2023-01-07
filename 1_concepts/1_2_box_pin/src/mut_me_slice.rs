use std::{fmt, pin::Pin, ops::{Deref, DerefMut}};
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
        self.set(&[4, 3, 2, 1]);
        todo!("i don't know how to mutate a slice");
    }
}
impl SayHi for &[u8] {}
#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use super::*;
    #[test]
    fn test_mut() {
        //TODO: test mutating a slice
    }
}