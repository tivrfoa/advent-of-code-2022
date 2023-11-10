use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn move_ship((dir, y, x): (char, i32, i32), (cmd, qt): &(char, i32)) -> (char, i32, i32) {
	match cmd {
		'F' => {
			match dir {
				'N' => ('N', y + qt, x),
				'S' => ('S', y - qt, x),
				'E' => ('E', y, x + qt),
				'W' => ('W', y, x - qt),
				_ => panic!("{dir}"),
			}
		}
		'L' => {
			match qt {
				90 => {
					match dir {
						'N' => ('W', y, x),
						'S' => ('E', y, x),
						'E' => ('N', y, x),
						'W' => ('S', y, x),
						_ => panic!("{dir}"),
					}
				},
				180 => {
					match dir {
						'N' => ('S', y, x),
						'S' => ('N', y, x),
						'E' => ('W', y, x),
						'W' => ('E', y, x),
						_ => panic!("{dir}"),
					}
				}
				270 => {
					match dir {
						'N' => ('E', y, x),
						'S' => ('W', y, x),
						'E' => ('S', y, x),
						'W' => ('N', y, x),
						_ => panic!("{dir}"),
					}
				}
				_ => panic!("{qt}"),
			}
		}
		'R' => {
			match qt {
				90 => {
					match dir {
						'N' => ('E', y, x),
						'S' => ('W', y, x),
						'E' => ('S', y, x),
						'W' => ('N', y, x),
						_ => panic!("{dir}"),
					}
				},
				180 => {
					match dir {
						'N' => ('S', y, x),
						'S' => ('N', y, x),
						'E' => ('W', y, x),
						'W' => ('E', y, x),
						_ => panic!("{dir}"),
					}
				}
				270 => {
					match dir {
						'N' => ('W', y, x),
						'S' => ('E', y, x),
						'E' => ('N', y, x),
						'W' => ('S', y, x),
						_ => panic!("{dir}"),
					}
				}
				_ => panic!("{qt}"),
			}
		}
		'N' => (dir, y + qt, x),
		'S' => (dir, y - qt, x),
		'E' => (dir, y, x + qt),
		'W' => (dir, y, x - qt),
		_ => panic!("{cmd}"),
	}
}

struct Point {
	dir: char,
	y: i32,
	x: i32,
}

impl Point {
	fn new(dir: char, y: i32, x: i32) -> Self {
		Self {
			dir,
			y,
			x,
		}
	}
}

fn part1(input: String) -> String {
	let mut dir = 'E';
	let mut y = 0;
	let mut x = 0;

	let mut cmds: Vec<(char, i32)> = {
		let mut cmds = vec![];
		for line in input.lines() {
			let (cmd, qt) = line.split_at(1);
			let cmd = cmd.chars().next().unwrap();
			let qt = qt.parse::<i32>().unwrap();
			cmds.push((cmd, qt));
		}
		cmds
	};

	for cmd in cmds {
		(dir, y, x) = move_ship((dir, y, x), &cmd);
	}

	(y.abs() + x.abs()).to_string()
}

fn rotate_left(n: i32, mut x: i32, mut y: i32) -> (i32, i32) {
	for _ in 0..n {
		(y, x) = (x, -y);
	}
	(y, x)
}

fn rotate_right(n: i32, x: i32, y: i32) -> (i32, i32) {
	rotate_left(4 - n, x, y)
}

fn part2(input: String) -> String {
	let mut sd = 'E';
	let mut sy = 0;
	let mut sx = 0;
	let mut wy = 1;
	let mut wx = 10;

	let mut cmds: Vec<(char, i32)> = {
		let mut cmds = vec![];
		for line in input.lines() {
			let (cmd, qt) = line.split_at(1);
			let cmd = cmd.chars().next().unwrap();
			let qt = qt.parse::<i32>().unwrap();
			cmds.push((cmd, qt));
		}
		cmds
	};

	for (dir, n) in cmds {
		match dir {
			'N' | 'S' | 'E' | 'W' =>
				(_, wy, wx) = move_ship((dir, wy, wx), &('F', n)),
			'R' => (wy, wx) = rotate_right(n / 90, wy, wx),
			'L' => (wy, wx) = rotate_left(n / 90, wy, wx),
			'F' => (sy, sx) = (sy + n * wy, sx + n * wx),
			_ => panic!("{dir}"),
		}
	}

	(sy.abs() + sx.abs()).to_string()
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
        let input = util::read_file("inputs/2020/day12-sample.txt");
        assert_eq!("25", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day12.txt");
        assert_eq!("1457", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day12-sample.txt");
        assert_eq!("286", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day12.txt");
        assert_eq!("", part2(input));
    }
}
