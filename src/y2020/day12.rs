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
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day12.txt");
        assert_eq!("", part2(input));
    }
}
