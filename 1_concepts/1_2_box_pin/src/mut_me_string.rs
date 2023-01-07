use std::{fmt, pin::Pin};
pub trait MutMeSomehow {
fn mut_me_somehow(self: Pin<&mut Self>);
}
pub trait SayHi: fmt::Debug {
fn say_hi(self: Pin<&Self>) {
    println!("Hi from {:?}", self)
}
}
impl MutMeSomehow for String {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        self.get_mut().push('1');
    }
}
impl SayHi for String {}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use super::*;
    #[test]
    fn test_mut() {
        let mut s = String::from("AAA");
        let pin = Pin::new(&mut s);
        pin.mut_me_somehow();
        assert_eq!(s.chars().last(), Some('1'));
    }
}