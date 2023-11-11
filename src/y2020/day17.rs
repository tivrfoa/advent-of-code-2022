use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

const ACTIVE: char = '#';
const INACTIVE: char = '.';

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Pos {
	r: i32,
	c: i32,
	z: i32,
}

impl Pos {
	fn new(r: i32, c: i32, z: i32) -> Self {
		Self {
			r, c, z
		}
	}
}

fn increase_grid(map: &mut HashMap<Pos, char>, lb: i32,
		rb: i32, tb: i32, bb: i32, d0: i32, d1: i32)
		-> (i32, i32, i32, i32, i32, i32) {
	// left and right
	for r in tb-1..=bb+1 {
		for z in d0-1..=d1+1 {
			map.insert(Pos::new(r, lb-1, z), INACTIVE);
			map.insert(Pos::new(r, rb+1, z), INACTIVE);
		}
	}

	// top and bottom
	for c in lb..=rb {
		for z in d0-1..=d1+1 {
			map.insert(Pos::new(tb - 1, c, z), INACTIVE);
			map.insert(Pos::new(bb + 1, c, z), INACTIVE);
		}
	}

	// front and back
	for r in tb-1..=bb+1 {
		for c in lb..=rb {
			map.insert(Pos::new(r, c, d0 - 1), INACTIVE);
			map.insert(Pos::new(r, c, d1 + 1), INACTIVE);
		}
	}

	(lb - 1, rb + 1, tb - 1, bb + 1, d0 - 1, d1 + 1)
}

fn part1(input: String) -> String {
	let mut curr: HashMap<Pos, char> = HashMap::new();

	for (row, line) in input.lines().enumerate() {
		for (col, c) in line.chars().enumerate() {
			curr.insert(Pos::new(row as i32, col as i32, 0), c);
		}
	}
	let mut lb = 0;
	let mut rb = input.lines().next().unwrap().len() as i32 - 1;
	let mut tb = 0;
	let mut bb = input.lines().count() as i32 - 1;
	let mut d0 = 0;
	let mut d1 = 0;
	(lb, rb, tb, bb, d0, d1) =
		increase_grid(&mut curr, lb, rb, tb, bb, d0, d1);
	// dbg!(&curr);
	//let mut grid: Vec<Vec<char>> = vec![vec!['.'; 200]; 200];

	// assert_eq!(curr.len(), 27);

	for _ in 0..6 {
		let mut new: HashMap<Pos, char> = HashMap::with_capacity(curr.len() * 3);

		for (p, v) in &curr {
			let mut qt_active = 0;
			for r in p.r-1..=p.r+1 {
				for c in p.c-1..=p.c+1 {
					for z in p.z-1..=p.z+1 {
						if r == p.r && c == p.c && z == p.z { continue; }
						if let Some(neighbor_v) = curr.get(&Pos::new(r, c, z)) {
							if *neighbor_v == ACTIVE {
								qt_active += 1;
							}
						} else {
							// is inactive
							// do nothing
						}
					}
				}
			}
			if *v == ACTIVE {
				if qt_active == 2 || qt_active == 3 {
					new.insert(p.clone(), ACTIVE);
				} else {
					new.insert(p.clone(), INACTIVE);
				}
			} else {
				if qt_active == 3 {
					new.insert(p.clone(), ACTIVE);
				} else {
					new.insert(p.clone(), INACTIVE);
				}
			}
		}

		curr = new;
		(lb, rb, tb, bb, d0, d1) =
			increase_grid(&mut curr, lb, rb, tb, bb, d0, d1);
	}

	dbg!(&curr);
	curr.into_iter().filter(|(k, v)| *v == ACTIVE).count().to_string()
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
        let input = util::read_file("inputs/2020/day17-sample.txt");
        assert_eq!("112", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day17.txt");
        assert_eq!("346", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day17-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day17.txt");
        assert_eq!("", part2(input));
    }
}
