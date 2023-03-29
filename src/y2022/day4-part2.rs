use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

	let mut ans = 0;
	for line in contents.lines() {
		let mut pairs: Vec<&str> = line.split(',').collect();
		let mut nums: Vec<u32> = vec![];
		for p in pairs {
			let mut numbers: Vec<&str> = p.split('-').collect();
			nums.push(numbers[0].parse().unwrap());
			nums.push(numbers[1].parse().unwrap());
		}

		if (nums[0] >= nums[2] && nums[0] <= nums[3]) ||
			(nums[2] >= nums[0] && nums[2] <= nums[1]) {
				ans += 1;
		}
	}

	println!("{:?}", ans);
}

