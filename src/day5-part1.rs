use std::env;
use std::fs;

fn rearrange_stack(stacks: &mut Vec<Vec<char>>, line: &str) {
	let tokens = line.split_whitespace().collect::<Vec<&str>>();
	let qt:   usize = tokens[1].parse().unwrap();
	let from: usize = tokens[3].parse().unwrap();
	let to:   usize = tokens[5].parse().unwrap();

	for _ in 0..qt {
		let tmp = stacks[from].pop().unwrap();
		stacks[to].push(tmp);
	}
}

fn main() {
    let contents = get_file_contents();

	let mut ans: String = "".into();
	// 1 based, so we start with an empty stack
	let mut stacks: Vec<Vec<char>> = vec![vec![]];
	let mut skip_line = false; // for empty line after stack numbers
	let mut read_movements = false;

	for line in contents.lines() {
		if read_movements {
			rearrange_stack(&mut stacks, line);
		} else if skip_line {
			skip_line = false;
			read_movements = true;
		} else if line.trim().starts_with("1") {
			for i in 1..stacks.len() {
				stacks[i].reverse(); // make top item the last item
			}
			skip_line = true;
		} else {
			let mut i_stack = 1;
			if i_stack == stacks.len() {
				stacks.push(vec![]);
			}
			let mut col = 0;
			for c in line.chars() {
				if col == 4 {
					i_stack += 1;
					if i_stack == stacks.len() {
						stacks.push(vec![]);
					}
					col = 0;
				}
				if c != ' ' && c != '[' && c != ']' {
					stacks[i_stack].push(c);
				}

				col += 1;
			}
		}
	}

	println!("{:?}", stacks);

	for i in 1..stacks.len() {
		match stacks[i].pop() {
			Some(c) => ans.push(c),
			None => ans.push(' ')
		}
	}

	println!("{:?}", ans);
}

fn get_file_contents() -> String {
	let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
}
