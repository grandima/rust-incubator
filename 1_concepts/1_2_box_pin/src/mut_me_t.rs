
use std::{fmt::Debug, pin::Pin};
pub trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}
pub trait SayHi: Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}
impl<T: Default + Unpin + Clone> MutMeSomehow for T {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let x = self.get_mut();
        x.clone_from(&T::default());
    }
}
use std::fmt;
impl<T: fmt::Debug> SayHi for T {}

mod tests {
    use std::pin::Pin;
    use super::*;
    #[test]
    fn test_mut() {
        let mut s: i32 = 5;
        let pin = Pin::new(&mut s);
        pin.mut_me_somehow();
        assert_eq!(s, i32::default());
    }
}