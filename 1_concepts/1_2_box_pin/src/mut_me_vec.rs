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

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use super::*;
    #[test]
    fn test_mut() {
        let mut s: Vec<i32> = vec![1, 2, 3, 4];
        let pin = Pin::new(&mut s);
        pin.mut_me_somehow();
        assert_eq!(s.last(), Some(&i32::default()));
    }
}