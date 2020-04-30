mod file_creation;

fn main() {
    let f_name: &str = "data.txt";
    file_creation::check_file(f_name);
}
