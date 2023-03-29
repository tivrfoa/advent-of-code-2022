use std::env;
use std::fs;

#[derive(Debug)]
struct PlayResult {
	score: u32,
}

impl PlayResult {
	pub const LOST: PlayResult = PlayResult{ score: 0};
	pub const DRAW: PlayResult = PlayResult{ score: 3};
	pub const WON: PlayResult = PlayResult{ score: 6};
}

#[derive(PartialEq, Eq)]
struct ShapeScore {
	code: char,
	value: u32,
}


impl ShapeScore {
	pub const ROCK: ShapeScore = ShapeScore { code: 'A', value: 1};
	pub const PAPER: ShapeScore = ShapeScore { code: 'B', value: 2};
	pub const SCISSORS: ShapeScore = ShapeScore { code: 'C', value: 3};

	fn getShape(letter: &str) -> &ShapeScore {
		match letter.chars().next().unwrap() {
			'A' | 'X' => &ShapeScore::ROCK,
			'B' | 'Y' => &ShapeScore::PAPER,
			'C' | 'Z' => &ShapeScore::SCISSORS,
			_ => panic!("Invalid letter"),
		}
	}

	fn play(&self, player2: &ShapeScore) -> PlayResult {
		match (self, player2) {
			(&ShapeScore::ROCK, &ShapeScore::ROCK) => PlayResult::DRAW,
			(&ShapeScore::ROCK, &ShapeScore::PAPER) => PlayResult::LOST,
			(&ShapeScore::ROCK, &ShapeScore::SCISSORS) => PlayResult::WON,
			(&ShapeScore::PAPER, &ShapeScore::PAPER) => PlayResult::DRAW,
			(&ShapeScore::PAPER, &ShapeScore::SCISSORS) => PlayResult::LOST,
			(&ShapeScore::PAPER, &ShapeScore::ROCK) => PlayResult::WON,
			(&ShapeScore::SCISSORS, &ShapeScore::SCISSORS) => PlayResult::DRAW,
			(&ShapeScore::SCISSORS, &ShapeScore::ROCK) => PlayResult::LOST,
			(&ShapeScore::SCISSORS, &ShapeScore::PAPER) => PlayResult::WON,
			_ => todo!(),
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
		let opponent = ShapeScore::getShape(iter.next().unwrap());
		let you = ShapeScore::getShape(iter.next().unwrap());
		// println!("{:?}", you.play(opponent));

		your_score += you.play(opponent).score + you.value;
	}

	println!("{:?}", your_score);
}

