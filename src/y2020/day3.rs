use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

// use mod % column and row
fn solve(grid: &[Vec<char>], (r, c): (usize, usize)) -> u32 {
	let rows = grid.len();
	let cols = grid[0].len();
	let mut qt_trees = 0;
	let mut row = 0;
	let mut col = 0;

	while row < rows {
		if grid[row][col] == '#' {
			qt_trees += 1;
		}
		row += r;
		col = (col + c) % cols;
	}

	qt_trees
}

fn part1(input: String) -> String {
	let grid = input.to_char_grid();
	let slopes = vec![(1, 3)];
	let mut ans = 1;
	for s in slopes {
		ans *= solve(&grid, s);
	}
	ans.to_string()
}

fn part2(input: String) -> String {
	let grid = input.to_char_grid();
	let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
	let mut ans = 1;
	for s in slopes {
		ans *= solve(&grid, s);
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
        let input = util::read_file("inputs/2020/day3-sample.txt");
        assert_eq!("7", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day3.txt");
        assert_eq!("178", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day3-sample.txt");
        assert_eq!("336", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day3.txt");
        assert_eq!("3492520200", part2(input));
    }
}
