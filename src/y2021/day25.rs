use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

fn parse(input: String) -> Vec<Vec<char>> {
	let mut grid: Vec<Vec<char>> = vec![];

	for line in input.lines() {
		grid.push(line.chars().collect());
	}

	grid
}

fn part1(input: String) -> String {
	let mut grid: Vec<Vec<char>> = parse(input);
	let rows = grid.len();
	let cols = grid[rows - 1].len();
	let mut steps: u32 = 0;
	loop {
		steps += 1;
		let mut new_grid: Vec<Vec<char>> = grid.clone();
		let mut did_move = false;

		// move east
		for r in 0..rows {
			for c in 0..cols {
				let c2 = if c + 1 < cols { c + 1 } else { 0 };
				if grid[r][c] == '>' && grid[r][c2] == '.' {
					new_grid[r][c] = '.';
					new_grid[r][c2] = '>';
					did_move = true;
				}
			}
		}

		grid = new_grid;
		let mut new_grid: Vec<Vec<char>> = grid.clone();

		// move south
		for r in 0..rows {
			for c in 0..cols {
				let r2 = if r + 1 < rows { r + 1 } else { 0 };
				if grid[r][c] == 'v' && grid[r2][c] == '.' {
					new_grid[r][c] = '.';
					new_grid[r2][c] = 'v';
					did_move = true;
				}
			}
		}

		if !did_move {
			break;
		}
		grid = new_grid;
	}

	steps.to_string()
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
        let input = util::read_file("inputs/2021/day25-sample.txt");
        assert_eq!("58", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day25.txt");
        assert_eq!("435", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day25-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day25.txt");
        assert_eq!("", part2(input));
    }
}
