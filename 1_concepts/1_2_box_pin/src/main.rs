mod mut_me_box;
mod mut_me_rc;
mod mut_me_slice;
mod mut_me_string;
mod mut_me_t;
mod mut_me_vec;
fn main() {
    use mut_me_slice::*;
    use std::pin::Pin;
    let x = [1 as u8, 2, 3, 4];
    let mut slice = &x[0..];
    let pin = Pin::new(&mut slice);
    pin.mut_me_somehow();
    println!("{:?}", slice);
}
