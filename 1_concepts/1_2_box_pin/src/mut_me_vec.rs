use std::{fmt, pin::Pin};
pub trait MutMeSomehow {
fn mut_me_somehow(self: Pin<&mut Self>);
}
pub trait SayHi: fmt::Debug {
fn say_hi(self: Pin<&Self>) {
    println!("Hi from {:?}", self)
}
}
use std::{ops::DerefMut};
impl<T: Default> MutMeSomehow for Vec<T>
where
    Vec<T>: DerefMut,
    Vec<T>: Unpin,
{
    fn mut_me_somehow(self: Pin<&mut Self>) {
        self.get_mut().push(T::default());
    }
}
impl<T: std::fmt::Debug> SayHi for Vec<T> {}
