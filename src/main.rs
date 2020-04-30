mod append_data;
mod file_creation;
mod read_file;

fn main() {
    let mut data_text: String = String::from("Rust yra super fast and sonic boom right now");
    let f_name: &str = "data.txt";
    let count: i32 = 34;
    file_creation::create_file(f_name);
    file_creation::check_file(f_name);

    data_text.push_str("\n Forgot something, adding this too; \n");
    let res = append_data::write(f_name, count, data_text);
    if res == "OK" {
        read_file::read(f_name);
    }
}
