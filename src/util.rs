use std::env;
use std::fs;

#[allow(dead_code)]
pub fn get_file_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {file_path}");
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

#[allow(dead_code)]
pub fn read_file(file_path: &str) -> String {
    println!("In file {file_path}");
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

#[allow(dead_code)]
pub fn input_as_vec_i32(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[allow(dead_code)]
pub fn input_line_len(input: &str) -> usize {
    input.lines().next().unwrap().len()
}

#[allow(dead_code)]
pub fn input_as_vec_char(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}
