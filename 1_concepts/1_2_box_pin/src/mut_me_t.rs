mod mut_me_t {

    use crate::mut_me_mod;
    use std::pin::Pin;
    impl<T> mut_me_mod::MutMeSomehow for T {
        fn mut_me_somehow(v: Pin<&mut Self>) {
            let x = v.get_mut();
            x.clone_from(&x.clone());
        }
    }
    use std::fmt;
    impl<T: fmt::Debug> SayHi for T {}
}
