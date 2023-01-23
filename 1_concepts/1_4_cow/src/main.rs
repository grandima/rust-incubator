use std::io::prelude::*;
use std::{
    borrow::{Cow},
    env,
    fs::File,
};
fn create_file() {
    let args: Vec<String> = env::args().collect();
    let path: Cow<str> = if let Some(i) = args.iter().position(|x| x == "--conf") {
        args[i + 1].clone().into()
    } else if let Ok(env_value) = env::var("APP_CONF") {
        Cow::Owned(env_value)
    } else {
        "/etc/app/app.conf".into()
    };
    match File::create(path.as_ref()) {
        Ok(mut file) => {let _ = file.write_all(path.as_ref().as_bytes());},
        Err(e) => {println!("{:?} {:?}", e, path);},
    };
}


fn main() {
    create_file();
}
