use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Num {
	v: String,
	l: usize,
	r: usize,
}

impl Num {
	fn new() -> Self {
		Self {
			v: String::new(),
			l: 0,
			r: 0,
		}
	}
}

fn get_adjacent_part_numbers<'a>(rows: usize, cols: usize, row: usize, col: usize,
		parts: &'a HashMap<(usize, usize), &'a Num>) -> HashSet<&'a Num> {
	let mut nums = HashSet::new();

	for y in row as i32 - 1..=row as i32 + 1 {
		for x in col as i32 - 1..=col as i32 + 1 {
			if 0 <= y && y < rows as i32 && 0 <= x && x < cols as i32 {
				if let Some(num) = parts.get(&(y as usize, x as usize)) {
					nums.insert(*num);
				}
			}
		}
	}

	nums
}

fn has_adjacent_symbol(grid: &[Vec<char>], row: usize, l: usize, r: usize) -> bool {
	let rows = grid.len() as i32;
	let cols = grid[0].len() as i32;

	for y in row as i32 - 1..=row as i32 + 1 {
		for x in l as i32 - 1..=r as i32 + 1 {
			if 0 <= y && y < rows && 0 <= x && x < cols {
				let v = grid[y as usize][x as usize];
				if v != '.' && !('0' <= v && v <= '9') {
					return true;
				}
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
		let mut num = Num::new();
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
					if has_adjacent_symbol(&grid, r, num.l, num.r) {
						sum += num.v.parse::<i64>().unwrap();
					}
					num = Num::new();
				}
			}
		}
		if !num.v.is_empty() {
			if has_adjacent_symbol(&grid, r, num.l, num.r) {
				sum += num.v.parse::<i64>().unwrap();
			}
		}
	}

	sum.to_string()
}

pub fn part2(input: String) -> String {
	let mut grid: Vec<Vec<char>> = vec![];
	let mut parts: Vec<(usize, Num)> = vec![]; // (row, Num)
	let mut parts_map: HashMap<(usize, usize), &Num> = HashMap::new();
	for line in input.lines() {
		grid.push(line.chars().collect());
	}
	let rows = grid.len();
	let cols = grid[0].len();

	for r in 0..rows {
		let mut num = Num::new();
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
					if has_adjacent_symbol(&grid, r, num.l, num.r) {
						parts.push((r, num));
					}
					num = Num::new();
				}
			}
		}
		if !num.v.is_empty() {
			if has_adjacent_symbol(&grid, r, num.l, num.r) {
				parts.push((r, num));
			}
		}
	}

	for (row, num) in &parts {
		for col in num.l..=num.r {
			parts_map.insert((*row, col), num);
		}
	}

	let mut sum: u64 = 0;
	for r in 0..rows {
		for c in 0..cols {
			let v = grid[r][c];
			if v != '*' { continue; }
			let adj = get_adjacent_part_numbers(rows, cols, r, c, &parts_map);
			if adj.len() == 2 {
				sum += adj
					.iter()
					.map(|num| num.v.parse::<u64>().unwrap())
					.product::<u64>();
			}
		}
	}
	sum.to_string()
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
        assert_eq!("467835", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2023/day3.txt");
        assert_eq!("84495585", part2(input));
    }
}
