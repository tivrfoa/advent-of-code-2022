use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

fn part1(input: String) -> String {
	let mut valid_qt = 0;

	for line in input.lines() {
		let tmp = util::split_space(&line);
		let (min, max) = util::split_once_i32(tmp[0], '-');
		let letter = util::split_once(tmp[1], ':').0.chars().next().unwrap();
		let pwd = tmp[2];
		let mut qt = 0;
		for c in pwd.chars() {
			if c == letter {
				qt += 1;
			}
		}
		if min <= qt && qt <= max {
			valid_qt += 1;
		}
	}

	valid_qt.to_string()
}

fn part2(input: String) -> String {
	let mut valid_qt = 0;

	for line in input.lines() {
		let tmp = util::split_space(&line);
		let (min, max) = util::split_once_usize(tmp[0], '-');
		let letter = util::split_once(tmp[1], ':').0.chars().next().unwrap();
		let pwd: Vec<char> = tmp[2].chars().collect();
		let mut qt = 0;
		let pos = [min as usize - 1, max as usize - 1];
		for p in pos {
			if pwd[p] == letter {
				qt += 1;
			}
		}
		if qt == 1 {
			valid_qt += 1;
		}
	}

	valid_qt.to_string()
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
        let input = util::read_file("inputs/2020/day2-sample.txt");
        assert_eq!("2", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day2.txt");
        assert_eq!("519", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day2-sample.txt");
        assert_eq!("1", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day2.txt");
        assert_eq!("708", part2(input));
    }
}
