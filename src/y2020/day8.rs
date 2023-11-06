use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn part1(input: String) -> String {
	let mut v = 0;
	let mut ops: Vec<(&str, i32)> = vec![];
	let mut visited = HashSet::new();

	for line in input.lines() {
		let (op, v) = line.split_delim(' ');
		ops.push((op, v.parse::<i32>().unwrap()));
	}

	let len = ops.len();

	let mut idx: usize = 0;
	loop {
		if visited.contains(&idx) {
			break;
		}
		visited.insert(idx);
		match ops[idx].0 {
			"acc" => {
				v += ops[idx].1;
				idx += 1;
			}
			"jmp" => {
				idx = (idx as i32 + ops[idx].1) as usize;
			}
			"nop" => {
				idx += 1;
			}
			_ => panic!("{}", ops[idx].0),
		}
	}

	v.to_string()
}

fn part2(input: String) -> String {
	let mut ops: Vec<(&str, i32)> = vec![];
	for line in input.lines() {
		let (op, v) = line.split_delim(' ');
		ops.push((op, v.parse::<i32>().unwrap()));
	}

	let len = ops.len();
	let mut guess = 0;
	while guess < len {
		let (op, qt) = ops[guess];
		if op != "jmp" {
			guess += 1;
			continue;
		}
		let mut visited = HashSet::new();
		let mut idx: usize = 0;
		let mut v = 0;
		while idx <= len && !visited.contains(&idx) {
			if idx == len {
				return v.to_string();
			}
			visited.insert(idx);
			if idx == guess {
				idx += 1;
				continue;
			}
			match ops[idx].0 {
				"acc" => { v += ops[idx].1; idx += 1; }
				"jmp" => idx = (idx as i32 + ops[idx].1) as usize,
				"nop" => idx += 1,
				_ => panic!("{}", ops[idx].0),
			}
		}
		guess += 1;
	}

	panic!("Failed!");
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
        let input = util::read_file("inputs/2020/day8-sample.txt");
        assert_eq!("5", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day8.txt");
        assert_eq!("1941", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day8-sample.txt");
        assert_eq!("8", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day8.txt");
        assert_eq!("2096", part2(input));
    }
}
