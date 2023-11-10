use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn is_around_free_p2(grid: &[Vec<char>], r: usize, c: usize) -> bool {
	let rows = grid.len();
	let cols = grid[0].len();
	for (cond, (rd, cd)) in get_dirs_with_diagonals_i32(r, c, rows, cols) {
		if cond {
			let (r2, c2) = move_while(&grid, r as i32 + rd, c as i32 + cd, rd, cd, '.');
			if grid[r2][c2] == '#' {
				return false;
			}
		}
	}

	true
}

fn is_around_occupied_p2(grid: &[Vec<char>], r: usize, c: usize) -> bool {
	let rows = grid.len();
	let cols = grid[0].len();
	let mut qt = 0;
	for (cond, (rd, cd)) in get_dirs_with_diagonals_i32(r, c, rows, cols) {
		if cond {
			let (r2, c2) = move_while(&grid, r as i32 + rd, c as i32 + cd, rd, cd, '.');
			if grid[r2][c2] == '#' {
				qt += 1;
			}
		}
	}
	qt >= 5
}

fn is_around_free(grid: &[Vec<char>], r: usize, c: usize) -> bool {
	let rows = grid.len();
	let cols = grid[0].len();
	let dirs = get_dirs_with_diagonals(r, c, rows, cols);
	for (cond, (r2, c2)) in dirs {
		if cond && grid[r2][c2] == '#' {
			return false;
		}
	}
	true
}

fn is_around_occupied(grid: &[Vec<char>], r: usize, c: usize) -> bool {
	let rows = grid.len();
	let cols = grid[0].len();
	let dirs = get_dirs_with_diagonals(r, c, rows, cols);
	let mut qt = 0;
	for (cond, (r2, c2)) in dirs {
		if cond && grid[r2][c2] == '#' {
			qt += 1;
		}
	}
	qt >= 4
}

fn part1(input: String) -> String {
	let mut grid = input.to_char_grid();
	let rows = grid.len();
	let cols = grid[0].len();

	loop {
		let mut changed = false;
		let mut new_grid: Vec<Vec<char>> = grid.clone();

		for r in 0..rows {
			for c in 0..cols {
				if grid[r][c] == 'L' {
					if is_around_free(&grid, r, c) {
						changed = true;
						new_grid[r][c] = '#';
					}
				} else if grid[r][c] == '#' {
					if is_around_occupied(&grid, r, c) {
						changed = true;
						new_grid[r][c] = 'L';
					}
				}
			}
		}

		if !changed { break; }
		grid = new_grid;
	}

	let mut ans = 0;
	for row in grid {
		ans += row.iter().filter(|&&c| c == '#').count();
	}
	ans.to_string()
}

fn part2(input: String) -> String {
	let mut grid = input.to_char_grid();
	let rows = grid.len();
	let cols = grid[0].len();

	loop {
		let mut changed = false;
		let mut new_grid: Vec<Vec<char>> = grid.clone();

		for r in 0..rows {
			for c in 0..cols {
				if grid[r][c] == 'L' {
					if is_around_free_p2(&grid, r, c) {
						changed = true;
						new_grid[r][c] = '#';
					}
				} else if grid[r][c] == '#' {
					if is_around_occupied_p2(&grid, r, c) {
						changed = true;
						new_grid[r][c] = 'L';
					}
				}
			}
		}

		if !changed { break; }
		grid = new_grid;
	}

	let mut ans = 0;
	for row in grid {
		ans += row.iter().filter(|&&c| c == '#').count();
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
        let input = util::read_file("inputs/2020/day11-sample.txt");
        assert_eq!("37", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day11.txt");
        assert_eq!("2489", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day11-sample.txt");
        assert_eq!("26", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day11.txt");
        assert_eq!("2180", part2(input));
    }
}
