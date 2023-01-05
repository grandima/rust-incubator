use std::{cell::RefCell, ops::DerefMut, pin::Pin, rc::Rc};
mod mut_me_mod {
    use std::pin::Pin;
    pub trait MutMeSomehow {
        fn mut_me_somehow(self: Pin<&mut Self>);
    }
}
mod mut_me_box {
    use crate::mut_me_mod;
    use std::{pin::Pin, ptr::swap};
    impl<T> mut_me_mod::MutMeSomehow for Box<T> {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let x = self.get_mut();
            unsafe {
                swap(x, x);
            }
            todo!("I don't know how to implement for Box<T> except using Box's Un");
        }
    }
}

mod mut_me_rc {
    use crate::mut_me_mod;
    use std::{pin::Pin, ptr::swap, rc::Rc};
    impl<T> mut_me_mod::MutMeSomehow for Rc<T> {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let x = self.get_mut();
            let y = x.clone();
            x.clone_from(&y);
            todo!("Rc also implements Unpin... Don't know how to do here...");
        }
    }
}

mod mut_me_vec {
    use crate::mut_me_mod;
    use std::{pin::Pin, ptr::swap, ops::DerefMut};
    impl<T> mut_me_mod::MutMeSomehow for Vec<T> {
        fn mut_me_somehow(mut self: Pin<&mut Self>) {
            // let y = self.deref_mut().as_mut();
            // let x = self.as_mut().remove(5);
            todo!("Need to think here...");
        }
    }
}
fn main() {
    println!("Implement me!");
}
