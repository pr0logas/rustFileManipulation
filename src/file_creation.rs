use std::error::Error;
use std::fs::File;

enum Result<T, E> {
    OK(T),
    Err(E),
}

pub fn check_file(name: &str) -> Box<String> {
    let f = File::open(name).expect("No such a file");
    f
}

pub fn create_file(name: &str) {
    let f = File::create(name).expect("File creation failed");
}
