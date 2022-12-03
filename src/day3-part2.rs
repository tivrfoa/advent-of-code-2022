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


fn find_badge(letters: &[[bool; 53]; 3]) -> usize {
	'l:
	for i in 0..53 {
		for j in 0..3 {
			if !letters[j][i] {
				continue 'l;
			}
		}
		return i;
	}
	println!("{:?}", letters);
	println!("{:?}", letters.iter().max());
	panic!("Invalid state");
}

fn main() {
	let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

	let mut sum = 0;
	let mut letters = [[false; 53]; 3];
	let mut i = 0;
	for line in contents.lines() {
		if i == 3 {
			sum += find_badge(&letters);
			for j in 0..3 {
				letters[j].fill(false);
			}
			i = 0;
		}

		for c in line.chars() {
			letters[i][get_idx(c)] = true;
		}

		i += 1;
	}

	sum += find_badge(&letters);

	println!("{:?}", sum);
}

