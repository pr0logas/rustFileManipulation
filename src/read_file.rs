use std::io::Read;

pub fn read(fname: &str) -> String {
    let f = std::fs::File::open(fname).unwrap();
    f
}
