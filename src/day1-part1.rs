use std::env;
use std::fs;

const NUMBER_OF_ELVES_TO_TRACK: usize = 3;

fn main() {
	let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

	// it will be in increasing order
	let mut max: [u32; NUMBER_OF_ELVES_TO_TRACK] = [0, 0, 0];
	let mut curr = 0;
	for line in contents.lines() {
		if line.is_empty() {
			if curr > max[0] {
				let mut pos = 0;
				// TODO I could use a binary search
				for i in (1..NUMBER_OF_ELVES_TO_TRACK).rev() {
					if curr > max[i] {
						pos = i;
						break;
					}
				}
				let mut prev = max[pos];
				max[pos] = curr;

				// shift left
				for i in (0..pos).rev() {
					let tmp = max[i];
					max[i] = prev;
					prev = tmp;
				}
			}
			curr = 0;
		} else {
			let cal: u32 = line.parse().unwrap();
			curr += cal;
		}

	}

	// println!("{:?}", max);
	println!("{}", max.iter().sum::<u32>());
}

