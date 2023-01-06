mod mut_me_t {
    use std::{fmt::Debug, pin::Pin};
    pub trait MutMeSomehow {
        fn mut_me_somehow(self: Pin<&mut Self>);
    }
    pub trait SayHi: Debug {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {:?}", self)
        }
    }
    impl<T: Clone + Unpin> MutMeSomehow for T {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let x = self.get_mut();
            x.clone_from(&x.clone());
        }
    }
    use std::fmt;
    impl<T: fmt::Debug> SayHi for T {}
}
