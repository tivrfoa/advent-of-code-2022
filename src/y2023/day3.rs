use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

pub fn part1(input: String) -> String {
	let mut sum: i32 = 0;

	for line in input.lines() {
		let mut num = String::new();
		for c in line.chars() {
			if '0' <= c && c <= '9' {
				num.push(c);
			} else {
				if !num.is_empty() {
					let n: i32 = num.parse().unwrap();
					dbg!(n);
					sum += n;
					num = String::new();
				}
			}
		}
	}

	sum.to_string()
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
        let input = util::read_file("inputs/2023/day3-sample.txt");
        assert_eq!("4361", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2023/day3.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2023/day3-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2023/day3.txt");
        assert_eq!("", part2(input));
    }
}
