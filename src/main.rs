mod append_data;
mod file_creation;

fn main() {
    let mut data_text: String = String::from("Rust yra super fast and sonic boom right now");
    let f_name: &str = "data.txt";
    let count: isize = 34;
    file_creation::create_file(f_name);
    file_creation::check_file(f_name);

    append_data
}
