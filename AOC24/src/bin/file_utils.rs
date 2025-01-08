use std::io::Read;
use std::fs::File;

pub fn get_file_content(file_name: &str) -> String {
    let mut result = File::open(file_name).unwrap();

    let mut file_content = String::new();

    result.read_to_string(&mut file_content).unwrap();

    return file_content;
}