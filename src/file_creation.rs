use std::fs::File;
use std::io::Read;

pub fn check_file(name: &str) {
    println!("{:#?}", file);
}

pub fn create_file(name: &str) {
    let file = File::create(name).expect("Create failed");
}
