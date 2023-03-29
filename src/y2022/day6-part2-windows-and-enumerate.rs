use std::collections::HashSet;
use std::env;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let contents = get_file_contents();
    let line = contents.lines().next().unwrap().as_bytes();

    for (i, w) in line.windows(14).enumerate() {
        let set: HashSet<&u8> = HashSet::from_iter(w);
        if set.len() == 14 {
            println!("{}", i + 14);
            return;
        }
    }
}

fn get_file_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
