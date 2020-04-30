use std::fs::File;

pub fn check_file(name: &str) {
    let _f = File::open(name).expect("No file or can't open");
    println!("File exists: {}", name)
}

pub fn create_file(name: &str) {
    let _f = File::create(name).expect("File creation failed");
    println!("File created: {}", name)
}
