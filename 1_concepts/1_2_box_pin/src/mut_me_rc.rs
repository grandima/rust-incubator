use std::{fmt, pin::Pin};
pub trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}
pub trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}
use std::{rc::Rc};
impl<T> MutMeSomehow for Rc<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let x = self.get_mut();
        let y = x.clone();
        x.clone_from(&y);
    }
}
impl<T: std::fmt::Debug> SayHi for Rc<T> {}
