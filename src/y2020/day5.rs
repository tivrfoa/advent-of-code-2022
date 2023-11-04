use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn part1(input: String) -> String {
	let mut max = 0;

	for line in input.lines() {
		let mut l = 0;
		let mut r = 127;
		for c in line[0..7].chars() {
			if c == 'F' {
				r = (l + r) / 2;
			} else {
				l = (l + r) / 2 + 1;
			}
		}
		assert!(l == r);
		let row = l;

		let mut l = 0;
		let mut r = 7;
		for c in line[7..].chars() {
			if c == 'L' {
				r = (l + r) / 2;
			} else {
				l = (l + r) / 2 + 1;
			}
		}
		assert!(l == r);
		let col = l;

		let seat_id = row * 8 + col;
		max = max.max(seat_id);
	}

	max.to_string()
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
        let input = util::read_file("inputs/2020/day5-sample.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day5.txt");
        assert_eq!("991", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day5-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day5.txt");
        assert_eq!("", part2(input));
    }
}
