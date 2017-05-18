extern crate serde;
extern crate toml;

extern crate riiir;
use riiir::config;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("examples/riiir.toml").unwrap();
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let deserialized: config::Config = toml::from_str(&buf).unwrap();

    println!("{:#?}", deserialized);
}