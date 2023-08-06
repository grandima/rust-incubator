mod request;
use request::Request;
use std::fs;


use std::borrow::Borrow;
use serde::Serialize;
use toml;
use serde_yaml;
fn main() {
    let contents = fs::read_to_string("request.json")
        .expect("Should have been able to read the file");
    let r = serde_json::from_str::<Request>(contents.borrow()).unwrap();
    let toml = toml::to_string(&r).unwrap();
    let yaml = serde_yaml::to_string(&r).unwrap();
    println!("{:?}", yaml);
    println!("{:?}", toml);
}


