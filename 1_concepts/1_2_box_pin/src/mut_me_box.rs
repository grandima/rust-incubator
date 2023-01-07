use std::{fmt, pin::Pin};
pub trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}
pub trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}
impl<T: Default> MutMeSomehow for Box<T> {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        self.set(Box::new(T::default()));
    }
}
impl<T: fmt::Debug> SayHi for Box<T> {}

mod tests {
    use std::pin::Pin;
    use super::*;
    #[test]
    fn test_mut() {
        let mut s = Box::new(5);
        let pin = Pin::new(&mut s);
        pin.mut_me_somehow();
        assert_eq!(s, Box::new(i32::default()));
    }
}