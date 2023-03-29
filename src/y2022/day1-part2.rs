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

				let pos = match max.binary_search(&curr) {
					Ok(_) => {
						// find index lower
						match max.binary_search(&(curr - 1)) {
							Ok(mut idx) => { // need to find last index equals curr - 1
								while max[idx + 1] == curr - 1 { idx += 1; }
								idx
							},
							Err(idx) => idx
						}
					},
					Err(idx) => idx
				};

				// pos can be greater than length ...
				let pos = pos.min(NUMBER_OF_ELVES_TO_TRACK - 1);
				
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

