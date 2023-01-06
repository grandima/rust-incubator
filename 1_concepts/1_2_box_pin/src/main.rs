use std::{cell::RefCell, ops::DerefMut, pin::Pin, rc::Rc};
mod mut_me_mod {
    use std::{pin::Pin, fmt};
    pub trait MutMeSomehow {
        fn mut_me_somehow(self: Pin<&mut Self>);
    }
    //The default implementation for this trait is the same for T
    pub trait SayHi: fmt::Debug {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {:?}", self)
        }
    }
}
mod mut_me_box {
    use crate::mut_me_mod;
    use std::{pin::Pin, ptr::swap, fmt};
    impl<T: Default> mut_me_mod::MutMeSomehow for Box<T> {
        fn mut_me_somehow(mut self: Pin<&mut Self>) {
            self.set(Box::new(T::default()));
        }
    }
    impl <T: fmt::Debug> mut_me_mod::SayHi for Box<T> {}
}

mod mut_me_rc {
    use crate::mut_me_mod;
    use std::{pin::Pin, ptr::swap, rc::Rc};
    impl<T> mut_me_mod::MutMeSomehow for Rc<T> {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let x = self.get_mut();
            let y = x.clone();
            x.clone_from(&y);
        }
    }
    impl <T: std::fmt::Debug> mut_me_mod::SayHi for Rc<T> {}
}

mod mut_me_vec {
    use crate::mut_me_mod;
    use std::{ops::DerefMut, pin::Pin};
    impl<T: Default> mut_me_mod::MutMeSomehow for Vec<T>
    where
        Vec<T>: DerefMut,
        Vec<T>: Unpin,
    {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            self.get_mut().push(T::default());
        }
    }
    impl <T: std::fmt::Debug> mut_me_mod::SayHi for Vec<T> {}
}

mod mut_me_t {
    use crate::mut_me_mod;
    use std::{ops::DerefMut, pin::Pin};
    pub trait MutMeSomehow: Clone + Unpin {
        fn mut_me_somehow(v: Pin<&mut Self>) {
            let x = v.get_mut();
            x.clone_from(&x.clone());
        }
    }
}

mod mut_me_string {
    use crate::mut_me_mod;
    use std::{ops::DerefMut, pin::Pin};
    impl mut_me_mod::MutMeSomehow for String {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            self.get_mut().push('1');
        }
    }
    impl mut_me_mod::SayHi for String {}
}

mod mut_me_slice {
    use crate::mut_me_mod;
    use std::{pin::Pin};
    impl mut_me_mod::MutMeSomehow for &[u8] {
        fn mut_me_somehow(mut self: Pin<&mut Self>) {
            todo!("don't know how to implement here");
        }
    }
    impl mut_me_mod::SayHi for &[u8] {}
}

fn main() {
    println!("Implement me!");
}
