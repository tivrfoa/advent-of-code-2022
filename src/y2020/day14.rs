use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn part1(input: String) -> String {
	let mut map: HashMap<usize, usize> = HashMap::new();
	let mut mask = "";
	for line in input.lines() {
		let (op, val) = line.split_once(" = ").unwrap();
		if op.starts_with("mask") {
			mask = val;
		} else {
			let tmp: Vec<&str> = op.split('[').collect();
			let (op, _) = tmp[1].split_once(']').unwrap();
			let op: usize = op.parse().unwrap();
			let mut num: usize = val.parse().unwrap();
			for (i, c) in mask.chars().enumerate() {
				if c == '1' {
					num = set_one_at(num, 36 - i - 1);
				} else if c == '0' {
					num = set_zero_at(num, 36 - i - 1);
				} else {
					// skip
				}
			}
            map.insert(op, num);
		}
	}

	map.into_iter().map(|(_, v)| v).sum::<usize>().to_string()
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
        let input = util::read_file("inputs/2020/day14-sample.txt");
        assert_eq!("165", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day14.txt");
        assert_eq!("14553106347726", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day14-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day14.txt");
        assert_eq!("", part2(input));
    }
}
