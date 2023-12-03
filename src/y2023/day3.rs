use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

struct Num {
	v: String,
	l: usize,
	r: usize,
}

const SYMBOLS: [char; 4] = [
	'*',
	'+',
	'$',
	'#',
];


fn has_adjacent_symbol(grid: &[Vec<char>], row: usize, l: usize, r: usize) -> bool {
	let rows = grid.len();
	let cols = grid[0].len();

	let dirs: [(bool, usize, usize); 6] = [
		(l > 0, row, l - 1), // left
		(r + 1 < cols, row, r + 1), // right
		(l > 0 && row > 0, row - 1, l - 1), // top left
		(r + 1 < cols && row > 0, row - 1, r + 1), // top right
		(l > 0 && row + 1 < rows, row + 1, l - 1), // bottom left
		(r + 1 < cols && row + 1 < rows, row + 1, r + 1), // bottom right
	];

	for (cond, row, col) in dirs {
		if cond {
			let v = grid[row][col];
			if v != '.' && !('0' <= v && v <= '9') {
				return true;
			}
		}
	}

	// top bottom
	for col in l..=r {
		// top
		if row > 0 {
			let v = grid[row - 1][col];
			if v != '.' && !('0' <= v && v <= '9') {
				return true;
			}
		}
		// bottom
		if row + 1 < rows {
			let v = grid[row + 1][col];
			if v != '.' && !('0' <= v && v <= '9') {
				return true;
			}
		}
	}

	false
}

pub fn part1(input: String) -> String {
	let mut sum: i64 = 0;
	let mut grid: Vec<Vec<char>> = vec![];

	for line in input.lines() {
		grid.push(line.chars().collect());
	}
	let rows = grid.len();
	let cols = grid[0].len();

	for r in 0..rows {
		let mut num = Num {
			v: String::new(),
			l: 0,
			r: 0,
		};
		for c in 0..cols {
			let v = grid[r][c];
			if '0' <= v && v <= '9' {
				if num.v.is_empty() {
					num.l = c;
				}
				num.r = c;
				num.v.push(v);
			} else {
				if !num.v.is_empty() {
					let n: i64 = num.v.parse().unwrap();
					println!("{} - {} = {}", num.l, num.r, num.v);
					if has_adjacent_symbol(&grid, r, num.l, num.r) {
						println!("adding");
						sum += n;
					}
					num = Num {
						v: String::new(),
						l: 0,
						r: 0,
					};
				}
			}
		}
		if !num.v.is_empty() {
			let n: i64 = num.v.parse().unwrap();
			println!("{} - {} = {}", num.l, num.r, num.v);
			if has_adjacent_symbol(&grid, r, num.l, num.r) {
				println!("adding");
				sum += n;
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
        assert_eq!("544664", part1(input));
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
