use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn part1(input: String) -> String {
	let mut nums: Vec<i32> = input.to_nums();
	nums.sort();
	let mut curr = 0;
	let mut one_qt = 0;
	let mut three_qt = 1;

	for n in nums {
		let diff = n - curr;
		if diff == 1 {
			one_qt += 1;
		} else if diff == 3 {
			three_qt += 1;
		}
		curr += diff;
	}

	(one_qt * three_qt).to_string()
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
        let input = util::read_file("inputs/2020/day10-sample.txt");
        assert_eq!("220", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day10.txt");
        assert_eq!("2470", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day10-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day10.txt");
        assert_eq!("", part2(input));
    }
}
