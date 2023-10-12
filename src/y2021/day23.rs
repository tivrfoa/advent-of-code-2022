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
		prev: (0, 0),
	};
	
	let mut mem: HashSet<Grid> = HashSet::new();
	let mut pq: BinaryHeap<State> = BinaryHeap::new();
	pq.push(s);

	while let Some(s) = pq.pop() {
		if mem.contains(&s.grid) {
			continue;
		}
		mem.insert(s.grid.clone());
		if s.finished() {
			best = s.cost;
		}
		// if s.cost == 240 {
		// 	println!("here");
		// }
		// if s.cost == 3440 {
		// 	println!("here");
		// }
		if s.cost > best {
			return best.to_string();
		}
		
		// move all Letters if possible
		for r in 1..s.grid.len() {
			for c in 1..COLS {
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
fn dbg_grid<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

#[allow(dead_code)]
fn in_to_nums<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: Debug,
{
    input.split(',').map(|n| n.parse::<T>().unwrap()).collect()
}

#[allow(dead_code)]
fn split_str_to_nums<T: std::str::FromStr>(input: &str, separator: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: Debug,
{
    input
        .split(separator)
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}

#[allow(dead_code)]
fn vec_max<T: std::str::FromStr + std::cmp::Ord + Copy>(vec: &[T]) -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    *vec.iter().max().unwrap()
}

#[allow(dead_code)]
fn vec_min<T: std::str::FromStr + std::cmp::Ord + Copy>(vec: &[T]) -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    *vec.iter().min().unwrap()
}

#[allow(dead_code)]
fn str_to_char_tuple(s: &str) -> (char, char) {
    (s[0..1].chars().next().unwrap(), s[1..2].chars().next().unwrap())
}

#[allow(dead_code)]
trait MapAddOrInsert<K, V> {
    fn add_or_insert(&mut self, k: K, v: V);
}

#[allow(dead_code)]
impl<K: Eq + Hash, V: std::ops::AddAssign + Copy> MapAddOrInsert<K, V> for HashMap<K, V> {
    fn add_or_insert(&mut self, k: K, v: V) {
        self.entry(k).and_modify(|qt| *qt += v).or_insert(v);
    }
}

#[allow(dead_code)]
fn get_dirs(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 4] {
    [
        // left
        (c > 0, (r, if c > 0 { c - 1 } else { 0 })),
        // right
        (c < cols - 1, (r, c + 1)),
        // top
        (r > 0, (if r > 0 { r - 1 } else { 0 }, c)),
        // bottom
        (r < rows - 1, (r + 1, c)),
    ]
}

#[allow(dead_code)]
fn get_dirs_with_diagonals(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 8] {
    [
        // left
        (c > 0, (r, if c > 0 { c - 1 } else { 0 })),
        // right
        (c < cols - 1, (r, c + 1)),
        // top
        (r > 0, (if r > 0 { r - 1 } else { 0 }, c)),
        // bottom
        (r < rows - 1, (r + 1, c)),
        // top left
        (
            r > 0 && c > 0,
            (if r > 0 { r - 1 } else { 0 }, if c > 0 { c - 1 } else { 0 }),
        ),
        // top right
        (
            r > 0 && c < cols - 1,
            (if r > 0 { r - 1 } else { 0 }, c + 1),
        ),
        // bottom left
        (
            r < rows - 1 && c > 0,
            (r + 1, if c > 0 { c - 1 } else { 0 }),
        ),
        // bottom right
        (r < rows - 1 && c < cols - 1, (r + 1, c + 1)),
    ]
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
	grid: Grid,
	prev: (usize, usize),
}

impl State {
	fn finished(&self) -> bool {
		let g = self.grid;
		for r in 2..=g.len() - 2 {
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

	fn check_vertical(&self, col: usize, (r1, c1): Pos, to: Pos, cost: usize) -> Option<usize> {
		let (r2, c2) = (to.0, to.1);
		if r1 < r2 {
			// going down
			if !is_free_y(&self.grid, col, r1+1, r2) {
				return None;
			}
			Some((r2 - r1) * cost)
		} else {
			// going up
			if !is_free_y(&self.grid, col, r2, r1-1) {
				return None;
			}
			Some((r1 - r2) * cost)
		}
	}

	fn check_horizontal(&self, row: usize, from: Pos, to: Pos, cost: usize) -> Option<usize> {
		let (r1, c1) = (from.0, from.1);
		let (r2, c2) = (to.0, to.1);
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

	fn get_cost(&self, row: usize, col: usize) -> usize {
		let c = self.grid[row][col];
		match c {
			'A' => 1,
			'B' => 10,
			'C' => 100,
			'D' => 1000,
			_ => panic!("invalid Amphipod: {c} at ({row}, {col})"),
		}
	}

	fn moveto(&self, from: Pos, to: Pos) -> Option<Self> {
		let (r1, c1) = (from.0, from.1);
		let (r2, c2) = (to.0, to.1);
		if r2 < r1 && r2 != 1 { return None; }

		let final_col = get_final_room_col(self.grid[r1][c1]);

		// It it's in hallway, only go down if it's its final room
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

		let mut new_cost = 0;
		let cost = self.get_cost(r1, c1);

		match c2 {
			3 | 5 | 7 | 9 => {
				if r2 == 1 {
					return None;
				}
				let rows = self.grid.len() - 2;
				for r in r2+1..=rows {
					if self.grid[r1][c1] != self.grid[r][c2] {
						return None;
					}
				}
			}
			_ => ()
		}

		if r1 == r2 {
			// same row
			match self.check_horizontal(r1, from, to, cost) {
				Some(c) => new_cost += c,
				None => return None,
			}
		} else if r1 > r2 {
			// going up
			// first move vertical
			match self.check_vertical(c1, from, to, cost) {
				Some(c) => new_cost += c,
				None => return None,
			}
			match self.check_horizontal(r2, from, to, cost) {
				Some(c) => new_cost += c,
				None => return None,
			}
		} else {
			// going down
			// first move horizontal
			match self.check_horizontal(r1, from, to, cost) {
				Some(c) => new_cost += c,
				None => return None,
			}
			match self.check_vertical(c2, from, to, cost) {
				Some(c) => new_cost += c,
				None => return None,
			}
		}

		Some(Self {
			cost: self.cost + new_cost,
			grid: self.update_grid(from, to),
			prev: to,
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
	//#[ignore]
    fn p1s() {
        let input = util::read_file("inputs/2021/day23-sample.txt");
        assert_eq!("12521", part1(input));
    }

    #[test]
	//#[ignore]
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
	#[ignore = "reason"]
    fn p2() {
        let input = util::read_file("inputs/2021/day23p2.txt");
        assert_eq!("", part2(input));
    }
}
