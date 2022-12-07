use std::env;
use std::fs;

struct Dir {
	name: String,
	size: usize,
	dirs_index: Vec<usize>,
}

impl Dir {
	fn new(name: String) -> Self {
		Self {
			name,
			size: 0,
			dirs_index: vec![],
		}
	}
}

fn main() {
	let contents = get_file_contents();

	let mut dirs: Vec<Dir> = vec![];
	let mut root = Dir {
		name: "/".into(),
		size: 0,
		dirs_index: vec![],
	};
	dirs.push(root);

	let mut curr_dir = &mut dirs[0];
	let mut curr_size = 0;
	let mut curr_dir_index = 0;
	let mut prev_dir_index_stack = vec![];

	for line in contents.lines().skip(1) {
		if line.starts_with("$ cd") {
			curr_size = 0;
			let dir = line.split_ascii_whitespace().skip(2).next().unwrap();

			if dir == ".." {
				let size = curr_dir.size; // adds up the size of child dir
				curr_dir_index = prev_dir_index_stack.pop().unwrap();
				curr_dir = &mut dirs[curr_dir_index];
				curr_dir.size += size;
			} else {
				let len = curr_dir.dirs_index.len();
				for i in 0..len {
					if dirs[curr_dir.dirs_index[i]].name == dir {
						curr_dir_index = curr_dir.dirs_index[i];
						curr_dir = &mut dirs[curr_dir_index];
						break;
					}
				}
			}
		} else if line.starts_with("$ ls") {
			// just skip?
		} else if line.starts_with("dir ") {
			let dir = line.split_ascii_whitespace().skip(1).next().unwrap();
			dirs.push(Dir::new(dir.into()));
			let len = dirs.len();
			curr_dir.dirs_index.push(len - 1);
		} else {
			let size: usize = line
					.split_ascii_whitespace()
					.next()
					.unwrap()
					.parse()
					.unwrap();
			curr_dir.size += size;
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
