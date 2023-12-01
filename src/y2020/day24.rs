use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

pub fn part1(input: String) -> String {
	let mut black_up: HashSet<(i16, i16)> = HashSet::new();

	for line in input.lines() {
		let (mut x, mut y) = (0, 0);
		let mut idx = 0;
		while idx < line.len() {
			if line[idx..].starts_with("e") {
				x += 2;
				idx += 1;
			} else if line[idx..].starts_with("w") {
				x -= 2;
				idx += 1;
			} else if line[idx..].starts_with("nw") {
				x -= 1;
				y += 1;
				idx += 2;
			} else if line[idx..].starts_with("ne") {
				x += 1;
				y += 1;
				idx += 2;
			} else if line[idx..].starts_with("se") {
				x += 1;
				y -= 1;
				idx += 2;
			} else if line[idx..].starts_with("sw") {
				x -= 1;
				y -= 1;
				idx += 2;
			}
		}

		if !black_up.insert((x, y)) {
			black_up.remove(&(x, y));
		}
	}

	black_up.len().to_string()
}

pub fn part2(input: String) -> String {
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
        let input = util::read_file("inputs/2020/day24-sample.txt");
        assert_eq!("10", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day24.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day24-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day24.txt");
        assert_eq!("", part2(input));
    }
}
