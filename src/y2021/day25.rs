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
		let last_row = grid.len() - 1;
		grid[last_row].push(' ');
	}

	let last_row = vec![' '; grid[grid.len() - 1].len()];
	grid.push(last_row);

	grid
}

fn part1(input: String) -> String {
	let mut grid: Vec<Vec<char>> = parse(input);
	let rows = grid.len() - 1;
	let cols = grid[rows].len() - 1;
	// dbg!(&grid);
	let mut steps: u32 = 0;
	loop {
		steps += 1;
		let mut new_grid: Vec<Vec<char>> = grid.clone();
		let mut did_move = false;

		// move east
		for r in 0..rows {
			for c in 0..cols {
				if grid[r][c] == '>' && grid[r][c+1] == '.' {
					new_grid[r][c] = '.';
					new_grid[r][c+1] = '>';
					did_move = true;
				} else if grid[r][c] == '>' && grid[r][c+1] == ' ' && grid[r][0] == '.' {
					new_grid[r][c] = '.';
					new_grid[r][0] = '>';
					did_move = true;
				}
			}
		}

		grid = new_grid;
		let mut new_grid: Vec<Vec<char>> = grid.clone();

		// move south
		for r in 0..rows {
			for c in 0..cols {
				if grid[r][c] == 'v' && grid[r+1][c] == '.' {
					new_grid[r][c] = '.';
					new_grid[r+1][c] = 'v';
					did_move = true;
				} else if grid[r][c] == 'v' && grid[r+1][c] == ' ' && grid[0][c] == '.' {
					new_grid[r][c] = '.';
					new_grid[0][c] = 'v';
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
