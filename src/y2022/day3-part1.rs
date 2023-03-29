use std::env;
use std::fs;

// map letter to value
fn get_idx(c: char) -> usize {
    const a: usize = 'a' as usize;
    const A: usize = 'A' as usize;
    if c >= 'a' {
        c as usize - a + 1
    } else {
        c as usize - A + 27
    }
}

fn main() {
	let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

	let mut sum = 0;
	let mut letters = vec![false; 53];
	for line in contents.lines() {
		letters.fill(false);
		let half = line.len() / 2;
		let mut i = 0;
		for c in line.chars() {
			if i >= half {
				let idx = get_idx(c);
				if letters[idx] {
					// println!("repeated letter is: {}", c);
					sum += idx;
					break;
				}
			} else {
				letters[get_idx(c)] = true;
				i += 1;
			}
		}
	}

	println!("{:?}", sum);
}

