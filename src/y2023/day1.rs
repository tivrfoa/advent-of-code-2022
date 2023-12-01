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
	let mut digits: [(usize, u32); 2] = [(0, 0); 2];
	for line in input.lines() {
		let mut idx = 0;
		digits[1].1 = 10;
		for (i, c) in line.chars().enumerate() {
			match c.to_digit(10) {
				Some(n) => {
					digits[idx] = (i, n);
					if idx == 0 {
						idx += 1;
					}
				}
				None => {
				}
			}
		}

		if idx == 0 {
			digits[0] = (line.len(), 0);
			digits[1] = (0, 0);
		}

		if digits[1].1 == 10 {
			digits[1] = digits[0];
		}

		const DIGITS: [&str; 9] = [
			"one", "two", "three", "four", "five", "six", "seven",
			"eight", "nine"
		];

		let mut first = digits[0].0;
		for (i, d) in DIGITS.iter().enumerate() {
			if let Some(p) = line.find(d) {
				if p < first {
					first = p;
					digits[0] = (p, i as u32 + 1);
				}
			}
		}

		let mut second = digits[1].0;
		for (i, d) in DIGITS.iter().enumerate() {
			dbg!(&line[digits[1].0+1..], d);
			if let Some(p) = line[digits[1].0+1..].find(d) {
				let p = p + digits[1].0+1;
				println!("Found {p}, second = {}, i = {i}", second);
				if p > second {
					second = p;
					digits[1] = (p, i as u32 + 1);
				}
			}
		}
		dbg!(&digits);

		sum += digits[0].1 * 10 + digits[1].1;
	}
	sum.to_string()
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
        assert_eq!("", part2(input));
    }
}
