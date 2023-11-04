use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn map(key: &str) -> usize {
	match key {
		"byr" => 0,
		"iyr" => 1,
		"eyr" => 2,
		"hgt" => 3,
		"hcl" => 4,
		"ecl" => 5,
		"pid" => 6,
		"cid" => 7,
		_ => panic!("{key}"),
	}
}

fn part1(input: String) -> String {
	let mut qt_valid = 0;
	let mut keys = [0; 8];

	for line in input.lines() {
		if line.is_empty() {
			if keys[0..7].iter().filter(|&&v| v == 1).sum::<u8>() == 7 {
				qt_valid += 1;
			}
			keys = [0; 8];
			continue;
		}
		let kvs = line.split_space();
		for kv in kvs {
			keys[map(kv.left(':'))] = 1;
		}
	}

	qt_valid.to_string()
}

fn part2(input: String) -> String {
    "".into()
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
        let input = util::read_file("inputs/2020/day4-sample.txt");
        assert_eq!("2", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day4.txt");
        assert_eq!("230", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day4-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day4.txt");
        assert_eq!("", part2(input));
    }
}
