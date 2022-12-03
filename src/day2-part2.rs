use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct PlayResult {
	score: u32,
}

impl PlayResult {
	pub const LOST: PlayResult = PlayResult{ score: 0};
	pub const DRAW: PlayResult = PlayResult{ score: 3};
	pub const WON: PlayResult = PlayResult{ score: 6};
}

#[derive(PartialEq, Eq)]
struct Shape {
	code: char,
	value: u32,
}


impl Shape {
	pub const ROCK: Shape = Shape { code: 'A', value: 1};
	pub const PAPER: Shape = Shape { code: 'B', value: 2};
	pub const SCISSORS: Shape = Shape { code: 'C', value: 3};

	fn getShape(letter: &str) -> &Shape {
		match letter.chars().next().unwrap() {
			'A' | 'X' => &Shape::ROCK,
			'B' | 'Y' => &Shape::PAPER,
			'C' | 'Z' => &Shape::SCISSORS,
			_ => panic!("Invalid letter"),
		}
	}

	fn play(&self, player2: &Shape) -> PlayResult {
		match (self, player2) {
			(&Shape::ROCK, &Shape::ROCK) => PlayResult::DRAW,
			(&Shape::ROCK, &Shape::PAPER) => PlayResult::LOST,
			(&Shape::ROCK, &Shape::SCISSORS) => PlayResult::WON,
			(&Shape::PAPER, &Shape::PAPER) => PlayResult::DRAW,
			(&Shape::PAPER, &Shape::SCISSORS) => PlayResult::LOST,
			(&Shape::PAPER, &Shape::ROCK) => PlayResult::WON,
			(&Shape::SCISSORS, &Shape::SCISSORS) => PlayResult::DRAW,
			(&Shape::SCISSORS, &Shape::ROCK) => PlayResult::LOST,
			(&Shape::SCISSORS, &Shape::PAPER) => PlayResult::WON,
			_ => todo!(),
		}
	}

	fn find_shape_for_result(opponent: &Shape, your_result: &PlayResult) -> &'static Shape {
		match (opponent, your_result) {
			(&Shape::ROCK, &PlayResult::DRAW) => &Shape::ROCK,
			(&Shape::ROCK, &PlayResult::LOST) => &Shape::SCISSORS,
			(&Shape::ROCK, &PlayResult::WON) => &Shape::PAPER,
			(&Shape::PAPER, &PlayResult::DRAW) => &Shape::PAPER,
			(&Shape::PAPER, &PlayResult::LOST) => &Shape::ROCK,
			(&Shape::PAPER, &PlayResult::WON) => &Shape::SCISSORS,
			(&Shape::SCISSORS, &PlayResult::DRAW) => &Shape::SCISSORS,
			(&Shape::SCISSORS, &PlayResult::LOST) => &Shape::PAPER,
			(&Shape::SCISSORS, &PlayResult::WON) => &Shape::ROCK,
			_ => todo!(),
		}
	}

	fn get_result(letter: &str) -> PlayResult {
		match letter.chars().next().unwrap() {
			'X' => PlayResult::LOST,
			'Y' => PlayResult::DRAW,
			'Z' => PlayResult::WON,
			_ => panic!("Invalid letter"),
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

	let mut your_score = 0;
	for line in contents.lines() {
		let mut iter = line.split_whitespace();
		let opponent = Shape::getShape(iter.next().unwrap());
		let result = Shape::get_result(iter.next().unwrap());
		// println!("{:?}", you.play(opponent));

		your_score += Shape::find_shape_for_result(opponent, &result).value + result.score;
	}

	println!("{:?}", your_score);
}

