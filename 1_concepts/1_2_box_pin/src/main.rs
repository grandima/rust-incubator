mod mut_me_t;
mod mut_me_box;
mod mut_me_rc;
mod mut_me_vec;
mod mut_me_string;
mod mut_me_slice;
fn main() {
    use std::pin::Pin;
    use mut_me_string::*;
    let mut s = String::from("AAA");
        let pin = Pin::new(&mut s);
        pin.mut_me_somehow();
        println!("{:?}", s.chars().last());
    println!("Implement me!");
}
