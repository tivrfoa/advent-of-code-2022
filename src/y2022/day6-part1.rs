use std::env;
use std::fs;

fn main() {
    let contents = get_file_contents();

	for line in contents.lines() {
		let line = line.as_bytes();
		'i:
		for i in 3..line.len() {
			for j in i-3..=i {
				for z in j + 1..=i {
					if line[j] == line[z] {
						continue 'i;
					}
				}
			}
			println!("{}", i + 1);
			return;
		}
	}
}

fn get_file_contents() -> String {
	let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
}
