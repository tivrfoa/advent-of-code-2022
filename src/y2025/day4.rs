use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn parse(input: &str) -> Vec<Vec<char>> {
	let mut ret = vec![];

	for line in input.lines() {
		ret.push(line.chars().collect());
	}

	ret
}

fn get_adjacent_positions<T: Copy>(grid: &[Vec<T>], r: usize, c: usize, default: T) -> [T; 8] {
	let rows = grid.len();
	let cols = grid[0].len();
	let nw = if r > 0 && c > 0 { grid[r - 1][c - 1] } else { default };
	let w  = if c > 0 { grid[r][c - 1] } else { default };
	let sw = if r + 1 < rows && c > 0 { grid[r + 1][c - 1] } else { default };
	let n  = if r > 0 { grid[r - 1][c] } else { default };
	let s  = if r + 1 < rows { grid[r + 1][c] } else { default };
	let ne = if r > 0 && c + 1 < cols { grid[r - 1][c + 1] } else { default };
	let e  = if c + 1 < cols { grid[r][c + 1] } else { default };
	let se = if r + 1 < rows && c + 1 < cols { grid[r + 1][c + 1] } else { default };

	[nw, w, sw, n, s, ne, e, se]
}

pub fn part1(input: &str) -> String {
	let mut ans = 0;
	let grid = parse(input);
	for (r, row) in grid.iter().enumerate() {
		for (c, v) in row.iter().enumerate() {
			if *v == '.' { continue; }
			let qt_rp: usize = get_adjacent_positions(&grid, r, c, 'o')
				.iter()
				.filter(|c| **c == '@')
				.count();
			if qt_rp < 4 { ans += 1; }
		}
	}
    ans.to_string()
}

pub fn part2(input: &str) -> String {
	let mut ans = 0;
	let mut grid = parse(input);
	let rows = grid.len();
	let cols = grid[0].len();
	loop {
		let mut removed = false;
		for r in 0..rows {
			for c in 0..cols {
				if grid[r][c] == '.' { continue; }
				let qt_rp: usize = get_adjacent_positions(&grid, r, c, 'o')
					.iter()
					.filter(|c| **c == '@')
					.count();
				if qt_rp < 4 {
					ans += 1;
					grid[r][c] = '.';
					removed = true;
				}
			}
		}
		if !removed { break; }
	}
    ans.to_string()
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
        let input = include_str!("../../inputs/2025/day4-sample.txt");
        assert_eq!("13", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2025/day4.txt");
        assert_eq!("1549", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2025/day4-sample.txt");
        assert_eq!("43", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2025/day4.txt");
        assert_eq!("8887", part2(input));
    }
}
