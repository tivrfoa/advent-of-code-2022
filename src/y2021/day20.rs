use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

fn get_algo(s: &str) -> Vec<char> {
	s.lines().next().unwrap().chars().collect()
}

const space: usize = 200;

fn get_input(s: &str) -> Vec<Vec<char>> {
	let rows = s.lines().skip(2).count() + space * 2;
	let cols = s.lines().skip(2).next().unwrap().len() + space * 2;
	let mut ret = vec![vec!['.'; cols]; rows];

	let mut row = space;
	for line in s.lines().skip(2) {
		let mut col = space;
		for c in line.chars() {
			ret[row][col] = c;
			col += 1;
		}
		row += 1;
	}

	ret
}

fn part1(input: String) -> String {
	let algo: Vec<char> = get_algo(&input);
	let mut input: Vec<Vec<char>> = get_input(&input);
	// for r in 0..input.len() {
	// 	for c in 0..input[0].len() {
	// 		print!("{} ", input[r][c]);
	// 	}
	// 	println!();
	// }
	let rows = input.len();
	let cols = input[0].len();
	dbg!(rows); dbg!(cols);
	let mut output: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];

	const pos: [[(i16, i16); 3]; 3] = [
		[
			(-1, -1),
			(-1, 0),
			(-1, 1),
		],
		[
			(0, -1),
			(0, 0),
			(0, 1),
		],
		[
			(1, -1),
			(1, 0),
			(1, 1),
		]
	];

	for t in 1..=2 {
		let q = space - 2 * t;
		for r in q..rows - q {
			for c in q..cols - q {
				let mut num = String::new();
				for nn in pos {
					let mut n = String::new();
					for (r1, c1) in nn {
						let c = input[rc(r, r1)][rc(c, c1)];
						n.push(if c == '#' { '1' } else { '0'});
					}
					num.push_str(&mut n);
				}
				// println!("{num}");
				let idx: usize = usize::from_str_radix(&num, 2).unwrap();
				// println!("{num} -> {idx} -> {}", algo[idx]);
				output[r][c] = algo[idx];
			}
		}
		input = output.clone();
	}
	for r in 0..input.len() {
		for c in 0..input[0].len() {
			print!("{} ", input[r][c]);
		}
		println!();
	}

	let mut lits = 0;
	for row in output {
		lits += row.into_iter().filter(|&c| c == '#').count();
	}
	lits.to_string()
}

fn rc(p: usize, p1: i16) -> usize {
	(p as i16 + p1) as usize
}

fn part2(input: String) -> String {
    "".into()
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
        let input = util::read_file("inputs/2021/day20-sample.txt");
        assert_eq!("35", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day20.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day20-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day20.txt");
        assert_eq!("", part2(input));
    }
}
