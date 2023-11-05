use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn part1(input: String) -> String {
	let mut sum = 0;

	let mut set: HashSet<char> = HashSet::new();
	for line in input.lines() {
		if line.is_empty() {
			sum += set.len();
			set = HashSet::new();
			continue;
		}

		for c in line.chars() {
			set.insert(c);
		}
	}

	sum.to_string()
}

fn part2(input: String) -> String {
	let mut sum = 0;

	let mut map: HashMap<char, u16> = HashMap::new();
	let mut n = 0;
	for line in input.lines() {
		if line.is_empty() {
			sum += map.iter().filter(|(_, &v)| v == n).count();
			map = HashMap::new();
			n = 0;
			continue;
		}
		n += 1;
		for c in line.chars() {
			map.entry(c).and_modify(|qt| *qt += 1).or_insert(1);
		}
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
    #[ignore]
    fn p1s() {
        let input = util::read_file("inputs/2020/day6-sample.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day6.txt");
        assert_eq!("6596", part1(input));
    }

    #[test]
    #[ignore]
    fn p2s() {
        let input = util::read_file("inputs/2020/day6-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day6.txt");
        assert_eq!("3219", part2(input));
    }
}
