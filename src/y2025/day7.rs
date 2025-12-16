use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn parse(input: &str) -> Vec<Vec<char>> {
	input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect()
}

fn find_start_pos(grid: &[Vec<char>]) -> (usize, usize) {
	for (r, row) in grid.iter().enumerate() {
		for (c, v) in row.iter().enumerate() {
			if *v == 'S' { return (r, c); }
		}
	}
	unreachable!()
}

fn solve(grid: &mut Vec<Vec<char>>, (mut r, c): (usize, usize)) -> usize {
	if grid[r][c] == '|' { return 0; }
	let rows = grid.len();
	let cols = grid[0].len();

	while r < rows {
		if grid[r][c] == '^' {
			grid[r][c] = '|';
			let mut sum = 1;
			if c > 0 {
				sum += solve(grid, (r + 1, c - 1));
			}
			if c + 1 < cols {
				sum += solve(grid, (r + 1, c + 1));
			}
			return sum;
		}
		grid[r][c] = '|';
		r += 1;
	}

	0
}

pub fn part1(input: &str) -> String {
	let mut ans = 0;
	let mut grid = parse(input);
	let start = find_start_pos(&grid);
	solve(&mut grid, start).to_string()
}

pub fn part2(input: &str) -> String {
    "".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2025/day7-sample.txt");
        assert_eq!("21", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2025/day7.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2025/day7-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2025/day7.txt");
        assert_eq!("", part2(input));
    }
}
