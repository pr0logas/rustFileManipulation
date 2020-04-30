use std::fs::OpenOptions;

pub fn append_data(fname: &str, count: i32, data: String) -> &str {
    let mut f = OpenOptions::new()
        .append(true)
        .open(fname)
        .expect("Cannot open file");
    f.write_all(data.as_bytes()).expect("Write failed");
    let status: &str = "OK";
    status
}
