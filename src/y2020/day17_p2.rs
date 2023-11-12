use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

use new_derive::New;

const ACTIVE: char = '#';
const INACTIVE: char = '.';

fn part1(input: String) -> String {
    "".into()
}


// Solution based on Elizarov
// https://github.com/elizarov/AdventOfCode2020/blob/main/Day17_2.kt

#[derive(Copy, Clone, New, Debug, Eq, Hash, PartialEq)]
struct Cube {
	r: i32,
	c: i32,
	z: i32,
	w: i32,
}


fn part2(input: String) -> String {
	let mut cubes: HashSet<Cube> = HashSet::new();
	for (row, line) in input.lines().enumerate() {
		for (col, c) in line.chars().enumerate() {
			if c == ACTIVE {
				cubes.insert(Cube::new(row as i32, col as i32, 0, 0));
			}
		}
	}

	for _ in 0..6 {
		let mut neighbors = vec![];
		for Cube {r, c, z, w} in &cubes {
			for dr in -1..=1 {
				for dc in -1..=1 {
					for dz in -1..=1 {
						for dw in -1..=1 {
							if dr != 0 || dc != 0 || dz != 0 || dw != 0 {
								neighbors.push(Cube::new(r+dr,c+dc,z+dz,w+dw));
							}
						}
					}
				}
			}
		}
		let mut neighbors_count: HashMap<Cube, usize> = neighbors.grouping_by();
		cubes.retain(|cube| {
			if let Some(qt) = neighbors_count.get(&cube) {
				2 <= *qt && *qt <= 3
			} else {
				false
			}
		});
		for (k, v) in neighbors_count {
			if v == 3 {
				cubes.insert(k);
			}
		}
	}

	cubes.len().to_string()
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
        let input = util::read_file("inputs/2020/day17-sample.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day17.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day17-sample.txt");
        assert_eq!("848", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day17.txt");
        assert_eq!("1632", part2(input));
    }
}
