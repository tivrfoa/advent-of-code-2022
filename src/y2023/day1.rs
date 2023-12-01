use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

pub fn part1(input: String) -> String {
	let mut sum = 0;
	let mut digits: [u32; 2] = [0; 2];
	for line in input.lines() {
		let mut idx = 0;
		digits[1] = 10;
		for c in line.chars() {
			match c.to_digit(10) {
				Some(n) => {
					digits[idx] = n;
					if idx == 0 {
						idx += 1;
					}
				}
				None => {
				}
			}
		}
		if idx == 1 {
			if digits[1] == 10 {
				sum += digits[0] * 10 + digits[0];
			} else {
				sum += digits[0] * 10 + digits[1];
			}
		}
	}
	sum.to_string()
}

pub fn part2(input: String) -> String {
	let mut sum = 0;
	for (lidx, line) in input.lines().enumerate() {
		let num = get_two_digits(line);
		println!("line = {lidx} - {}", num);
		sum += num;
	}
	sum.to_string()
}

fn get_two_digits(s: &str) -> u32 {
	const DIGITS: [(&str, u32); 18] = [
		("1", 1),
		("2", 2),
		("3", 3),
		("4", 4),
		("5", 5),
		("6", 6),
		("7", 7),
		("8", 8),
		("9", 9),
		("one", 1),
		("two", 2),
		("three", 3),
		("four", 4),
		("five", 5),
		("six", 6),
		("seven", 7),
		("eight", 8),
		("nine", 9),
	];
	let mut digits: [(usize, u32); 2] = [(0, 0); 2];

	let mut first = s.len();
	for (sval, val) in DIGITS {
		if let Some(p) = s.find(sval) {
			if p < first {
				first = p;
				digits[0] = (p, val);
			}
		}
	}
	digits[1] = digits[0];

	let mut second = first;
	for (sval, val) in DIGITS {
		if let Some(p) = s[second..].rfind(sval) {
			let p = p + second;
			if p > second {
				second = p;
				digits[1] = (p, val);
			}
		}
	}

	digits[0].1 * 10 + digits[1].1
}


#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = util::read_file("inputs/2023/day1-sample.txt");
        assert_eq!("142", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2023/day1.txt");
        assert_eq!("53334", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2023/day1-sample-p2.txt");
        assert_eq!("281", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2023/day1.txt");
        assert_eq!("52834", part2(input));
    }

    #[test]
    fn test_get_two_digits() {
		assert_eq!(42, get_two_digits("41t279zsgmzmjk12"));
    }
}
