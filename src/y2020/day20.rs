use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

struct Tile {
	id: usize,
	grid: Vec<Vec<char>>,
	top_bottom: Vec<(usize, usize)>, // count dots
	left_right: Vec<(usize, usize)>,
}

impl Tile {
	fn new(id: usize, grid: Vec<Vec<char>>) -> Self {
		let rows = grid.len();
		let cols = grid[0].len();

		// top bottom -> flipping
		let mut top = 0;
		let mut bottom = rows - 1;
		let mut top_bottom = vec![];

		for _ in 0..cols {
			let qt_top = grid[top].iter().filter(|&c| c == '.').count();
			let qt_bottom = grid[bottom].iter().filter(|&c| c == '.').count();
			top_bottom.push((qt_top, qt_bottom));

			bottom = top;
			top += i;
		}

		// left right -> rotating
		let mut left = 0;
		let mut right = cols - 1;
		let mut left_right = vec![];
		for _ in 0..cols {
			let mut qt_left = 0;
			let mut qt_right = 0;
			for r in 0..rows {
				qt_left += if grid[r][left] == '.' { 1 } else { 0 };
				qt_right += if grid[r][right] == '.' { 1 } else { 0 };

				right = left;
				left += i;
			}
			left_right.push((qt_left, qt_right));
		}

		Self {
			id,
			grid,
			top_bottom,
			top_left,
		}
	}
}

fn part1(input: String) -> String {
    "".into()
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
        let input = util::read_file("inputs/2020/day20-sample.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day20.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day20-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day20.txt");
        assert_eq!("", part2(input));
    }
}
