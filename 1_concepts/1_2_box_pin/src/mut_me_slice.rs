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
    }
}
impl SayHi for &[u8] {}
#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use super::*;
    #[test]
    fn test_mut() {
        let x = [1 as u8, 2, 3, 4];
        let mut slice = &x[0..];
        let pin = Pin::new(&mut slice);
        pin.mut_me_somehow();
        assert_eq!(slice, &[4, 3, 2, 1]);
    }
}