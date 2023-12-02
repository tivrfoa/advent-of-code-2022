use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn max(color: &str) -> u32 {
	match color {
		"red" => 12,
		"green" => 13,
		"blue" => 14,
		_ => panic!("{color}"),
	}
}

pub fn part1(input: String) -> String {
	let mut sum = 0;
	'l:
	for (i, line) in input.lines().enumerate() {
		let game = i + 1;
		let (_, cubes) = line.split_once(": ").unwrap();
		for set in cubes.split("; ") {
			for cube in set.split(", ") {
				let (qt, color) = cube.split_once(' ').unwrap();
				if qt.parse::<u32>().unwrap() > max(color) {
					continue 'l;
				}
			}
		}
		sum += game;
	}

	sum.to_string()
}

pub fn part2(input: String) -> String {
	let mut sum = 0;
	for line in input.lines() {
		let (mut max_blue, mut max_red, mut max_green) = (1, 1, 1);
		let (_, cubes) = line.split_once(": ").unwrap();
		for set in cubes.split("; ") {
			for cube in set.split(", ") {
				let (qt, color) = cube.split_once(' ').unwrap();
				let qt = qt.parse::<u32>().unwrap();
				match color {
					"red" => max_red = max_red.max(qt),
					"green" => max_green = max_green.max(qt),
					"blue" => max_blue = max_blue.max(qt),
					_ => panic!("{color}"),
				};
			}
		}
		sum += max_blue * max_red * max_green;
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
    fn p1s() {
        let input = util::read_file("inputs/2023/day2-sample.txt");
        assert_eq!("8", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2023/day2.txt");
        assert_eq!("2563", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2023/day2-sample.txt");
        assert_eq!("2286", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2023/day2.txt");
        assert_eq!("70768", part2(input));
    }
}
