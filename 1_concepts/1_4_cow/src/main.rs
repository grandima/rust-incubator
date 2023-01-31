use std::io::prelude::*;
use std::{
    borrow::{Cow},
    env,
    fs::File,
};
fn create_file() {
    //The iter stops at "--conf" because on macos it's not the first argument in the `env::args()`
    let mut iter = env::args().skip_while(|a| a != "--conf");
    iter.next();
    let path: Cow<str> = if let Some(str) = iter.next() {
        str.clone().into()
    } else if let Ok(env_value) = env::var("APP_CONF") {
        Cow::Owned(env_value)
    } else {
        "etc/app/app.conf".into()
    };
    println!("{:?}", path);
    match File::create(path.as_ref()) {
        Ok(mut file) => {let _ = file.write_all(path.as_ref().as_bytes());},
        Err(e) => {println!("{:?} {:?}", e, path);},
    };
}


fn main() {
    create_file();
}
