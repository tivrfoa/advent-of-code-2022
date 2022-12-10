use std::env;
use std::fs;

#[allow(dead_code)]
pub fn get_file_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

#[allow(dead_code)]
pub fn read_file(file_path: &str) -> String {
    println!("In file {}", file_path);
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
