/*
It requires distinct types and populate it using recursion.

The solution below is basically the same from Uncle Scientist:
https://www.youtube.com/watch?v=CMhq3M-HE0I

I took most ideas from him.

*/
use crate::util;

use std::str::Chars;

const TEN: u8 = 58;

#[derive(Debug)]
enum Val {
    Num(u8),
	List(Vec<Val>),
}

impl Val {
	fn is_right_order(&self, other: &Val) -> bool {


		true
	}

	fn parse_chars(mut chars: Chars) -> Self {
		// ignore first '['
		chars.next();

		Val::List(Self::parse(&mut chars))
	}

	fn parse(chars: &mut Chars) -> Vec<Val> {

		let mut result: Vec<Val> = vec![];
		let mut num = u8::MAX;

		while let Some(c) = chars.next() {
			match c {
				'[' => {
					result.push(Val::List(Self::parse(chars)));
				}
				']' => {
					if num != u8::MAX {
						result.push(Val::Num(num));
					}
					return result;
				}
				',' => {
					if num != u8::MAX {
						result.push(Val::Num(num));
						num = u8::MAX;
					}
				}
				_ => { // number
					if num != u8::MAX {
						num = TEN;
					} else {
						num = c as u8;
					}
				}
			}
		}

		result
	}
}

#[derive(Debug)]
struct Pair {
	a: Val,
	b: Val,
}

impl Pair {

	fn new(a: Val, b: Val) -> Self {
		Self {
			a,
			b,
		}
	}

	fn is_right_order(&self) -> bool {
		self.a.is_right_order(&self.b)
	}
}

pub fn solve(input: String) -> usize {
	let mut ans = 0;
    let mut pairs: Vec<Pair> = vec![];
	let mut lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

	for line in lines.chunks(2) {
		let a = Val::parse_chars(line[0].chars());
		let b = Val::parse_chars(line[1].chars());
		pairs.push(Pair::new(a, b));
	}

	for pair in &pairs {
		println!("{pair:?}");
	}

	for i in 0..pairs.len() {
		if pairs[i].is_right_order() {
			ans += i + 1;
			println!("index {} is good", i + 1);
		}
	}

	ans
}

fn dbg(grid: &Vec<Vec<u8>>) {
    for i in 0..grid.len() {
        println!("{:?}", grid[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/day13-sample.txt");
        assert_eq!(31, solve(input));
    }

    //#[test]
    //fn part1_input() {
    //    let input = util::read_file("inputs/day13.txt");
    //    assert_eq!(408, solve(input));
    //}

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day13-sample.txt");
    //    assert_eq!(29, solve_part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day13.txt");
    //    assert_eq!(399, solve_part2(input));
    //}
}
