use std::io::Read;

pub fn read(fname: &str) -> String {
    let mut f = std::fs::File::open(fname).unwrap();
    let mut contents = String::new();
    let c = f.read_to_string(&mut contents).unwrap();
    let res = format!("{:?}", c);
    res
}
