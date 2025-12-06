use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

/*
What is the grand total found by adding together all of the answers
to the individual problems?
*/
pub fn part1(input: &str) -> String {
	let mut ans = 0;
	let mut grid: Vec<Vec<usize>> = vec![];
	let mut operations = vec![];

	for line in input.lines() {
		let line = line.trim();
		if line.starts_with("+") || line.starts_with("*") {
			for c in line.split_ascii_whitespace() {
				operations.push(c);
			}
		} else {
			grid.push(line.split_ascii_whitespace()
				.map(|s| s.parse::<usize>().unwrap())
				.collect());
		}
	}
	// dbg!(grid, operations);
	let rows = grid.len();
	let cols = grid[0].len();
	for c in 0..cols {
		let mut res = grid[0][c];
		if operations[c] == "+" {
			for r in 1..rows {
				res += grid[r][c];
			}
		} else {
			for r in 1..rows {
				res *= grid[r][c];
			}
		}
		ans += res;
	}

	ans.to_string()
}


pub fn part2(input: &str) -> String {
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
        other
            .cost
            .cmp(&self.cost)
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
        let input = include_str!("../../inputs/2025/day6-sample.txt");
        assert_eq!("4277556", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2025/day6.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2025/day6-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2025/day6.txt");
        assert_eq!("", part2(input));
    }
}
