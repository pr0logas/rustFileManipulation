use std::fs::OpenOptions;
use std::io::Write;

pub fn write(fname: &str, count: i32, data: String) -> &str {
    for i in 0..=count {
        let mut f = OpenOptions::new()
            .append(true)
            .open(fname)
            .expect("Cannot open file");
        f.write_all(data.as_bytes()).expect("Write failed");
    }
    let status: &str = "OK";
    status
}
