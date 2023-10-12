use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, Binary};
use std::hash::Hash;
use std::iter::zip;

const ROWS: usize = 5;
const COLS: usize = 13;

type Grid = [[char; COLS]; ROWS];
type Pos = (usize, usize);

const fn get_final_room_col(c: char) -> usize {
	match c {
		'A' => 3,
		'B' => 5,
		'C' => 7,
		'D' => 9,
		_ => unreachable!(),
	}
}

const fn get_cost(c: char) -> usize {
	match c {
		'A' => 1,
		'B' => 10,
		'C' => 100,
		'D' => 1000,
		_ => unreachable!(),
	}
}

fn parse_grid(s: &str) -> Grid {
	let mut g: Grid = [[' '; COLS]; ROWS];

	let mut r = 0;
	for line in s.lines() {
		for (col, c) in line.chars().enumerate() {
			g[r][col] = c;
		}
		r += 1;
	}
	g
}

fn part1(input: String) -> String {
	let mut best = usize::MAX;
	let s = State {
		cost: 0,
		grid: parse_grid(&input),
	};
	println!("------- Solving ---------");
	for r in &s.grid {
		println!("{:?}", r);
	}
	println!("------- -----------------");
	
	let mut mem: HashSet<Grid> = HashSet::new();
	let mut pq: BinaryHeap<State> = BinaryHeap::new();
	pq.push(s);

	while let Some(s) = pq.pop() {
		if mem.contains(&s.grid) {
			continue;
		}
		mem.insert(s.grid.clone());
		if s.finished() {
			for r in &s.grid {
				println!("{:?}", r);
			}
			return s.cost.to_string();
		}
		
		// move all Letters if possible
		for r in 1..ROWS - 1 {
			for c in 1..COLS - 1 {
				if s.grid[r][c] >= 'A' && s.grid[r][c] <= 'D' {
					pq.append(&mut s.find_moves(&mut mem, r, c));
				}
			}
		}
	}
    
	best.to_string()
}

fn part2(input: String) -> String {
    part1(input)
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
	grid: Grid,
}

impl State {
	fn finished(&self) -> bool {
		let g = self.grid;
		for r in 2..ROWS - 1 {
			if g[r][3] != 'A' || g[r][5] != 'B' || g[r][7] != 'C' || g[r][9] != 'D' {
				return false;
			}
		}
		true
	}

	fn find_moves(&self, mem: &mut HashSet<Grid>, row: usize, col: usize) -> BinaryHeap<State> {
		let mut pq: BinaryHeap<State> = BinaryHeap::new();

		for r in 1..ROWS - 1 {
			for c in 1..COLS - 1 {
				if r == row && c == col { continue; }
				if let Some(s) = self.moveto((row, col), (r, c)) {
					if !mem.contains(&s.grid) {
						pq.push(s);
					}
				}
			}
		}

		pq
	}

	fn check_horizontal(&self, row: usize, c1: usize, c2: usize, cost: usize) -> Option<usize> {
		if c2 < c1 {
			// going left
			if !is_free_x(&self.grid, row, c2, c1-1) {
				return None;
			}
			Some((c1 - c2) * cost)
		} else {
			// going right
			if !is_free_x(&self.grid, row, c1+1, c2) {
				return None;
			}
			Some((c2 - c1) * cost)
		}
	}

	fn moveto(&self, from: Pos, to: Pos) -> Option<Self> {
		let (r1, c1) = (from.0, from.1);
		let (r2, c2) = (to.0, to.1);

		if r1 == r2 {
			// from the hallway it can only go to its final room!
			return None;
		}

		// Only go up if it goes to the hallway
		if r2 < r1 && r2 != 1 { return None; }

		let final_col = get_final_room_col(self.grid[r1][c1]);

		// It it's in hallway, only go down if it is to its final room
		if r2 > r1 && c2 != final_col {
			return None;
		}

		// Do not move if it's already in final column and it's filled?
		if c1 == final_col && r1 == 2 {
			let mut filled = true;
			for r in 3..ROWS - 1 {
				if self.grid[r][final_col] != self.grid[r1][c1] {
					filled = false;
					break;
				}
			}
			if filled { return None; }
		}

		match c2 {
			3 | 5 | 7 | 9 => {
				// cannot stay in room entrance
				if r2 == 1 {
					return None;
				}

				// can only go to a room if the others there are the same
				// type
				for r in r2+1..ROWS - 1 {
					if self.grid[r1][c1] != self.grid[r][c2] {
						return None;
					}
				}
			}
			_ => ()
		}

		let mut new_cost = 0;
		let cost = get_cost(self.grid[r1][c1]);

		
		if r1 > r2 {
			// going up
			// first move vertical
			for r in (r2..r1).rev() {
				if self.grid[r][c1] != '.' {
					return None;
				}
			}
			new_cost += (r1 - r2) * cost;
			match self.check_horizontal(r2, c1, c2, cost) {
				Some(c) => new_cost += c,
				None => return None,
			}
		} else {
			// going down
			// first move horizontal
			match self.check_horizontal(r1, c1, c2, cost) {
				Some(c) => new_cost += c,
				None => return None,
			}

			for r in (r1+1..r2).rev() {
				if self.grid[r][c2] != '.' {
					return None;
				}
			}
			new_cost += (r2 - r1) * cost;
		}

		Some(Self {
			cost: self.cost + new_cost,
			grid: self.update_grid(from, to),
		})
	}

	fn update_grid(&self, from: Pos, to: Pos) -> Grid {
		let mut grid = self.grid.clone();
		grid[to.0][to.1] = self.grid[from.0][from.1];
		grid[from.0][from.1] = '.';
		grid
	}
}

fn is_free_x(grid: &Grid, row: usize, cl: usize, cr: usize) -> bool {
	for c in cl..=cr {
		if grid[row][c] != '.' {
			return false;
		}
	}
	true
}

fn is_free_y(grid: &Grid, col: usize, r1: usize, r2: usize) -> bool {
	for r in r1..=r2 {
		if grid[r][col] != '.' {
			return false;
		}
	}
	true
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
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
        let input = util::read_file("inputs/2021/day23-sample.txt");
        assert_eq!("12521", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day23.txt");
        assert_eq!("16244", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day23-sample2.txt");
        assert_eq!("44169", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day23p2.txt");
        assert_eq!("", part2(input));
    }
}
