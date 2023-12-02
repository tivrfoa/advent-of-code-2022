use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

pub fn part1(input: String) -> String {
	let mut black_up: HashSet<(i16, i16)> = HashSet::new();

	for line in input.lines() {
		let (mut x, mut y) = (0, 0);
		let mut idx = 0;
		while idx < line.len() {
			if line[idx..].starts_with("e") {
				x += 2;
				idx += 1;
			} else if line[idx..].starts_with("w") {
				x -= 2;
				idx += 1;
			} else if line[idx..].starts_with("nw") {
				x -= 1;
				y += 1;
				idx += 2;
			} else if line[idx..].starts_with("ne") {
				x += 1;
				y += 1;
				idx += 2;
			} else if line[idx..].starts_with("se") {
				x += 1;
				y -= 1;
				idx += 2;
			} else if line[idx..].starts_with("sw") {
				x -= 1;
				y -= 1;
				idx += 2;
			}
		}

		if !black_up.insert((x, y)) {
			black_up.remove(&(x, y));
		}
	}

	black_up.len().to_string()
}

fn set_adjacent_tiles(adjacent_tiles: &mut [(i16, i16); 6],
		tiles: &HashSet<(i16, i16)>, x: i16, y: i16) {
	const deltas: [(&str, i16, i16); 6] = [
		("e", 2, 0),
		("w", -2, 0),
		("nw", -1, 1),
		("ne", 1, 1),
		("se", 1, -1),
		("sw", -1, -1),
	];

	for (i, (_, dx, dy)) in deltas.iter().enumerate() {
		adjacent_tiles[i] = (x + dx, y + dy);
	}
}

pub fn part2(input: String) -> String {
	let mut black_up: HashSet<(i16, i16)> = HashSet::new();

	for line in input.lines() {
		let (mut x, mut y) = (0, 0);
		let mut idx = 0;
		while idx < line.len() {
			if line[idx..].starts_with("e") {
				x += 2;
				idx += 1;
			} else if line[idx..].starts_with("w") {
				x -= 2;
				idx += 1;
			} else if line[idx..].starts_with("nw") {
				x -= 1;
				y += 1;
				idx += 2;
			} else if line[idx..].starts_with("ne") {
				x += 1;
				y += 1;
				idx += 2;
			} else if line[idx..].starts_with("se") {
				x += 1;
				y -= 1;
				idx += 2;
			} else if line[idx..].starts_with("sw") {
				x -= 1;
				y -= 1;
				idx += 2;
			}
		}

		if !black_up.insert((x, y)) {
			black_up.remove(&(x, y));
		}
	}

	let mut adjacent_tiles: [(i16, i16); 6] = [(0, 0); 6];

	for day in 0..100 {
		let mut new_black = HashSet::new();
		for (x, y) in &black_up {
			let (x, y) = (*x, *y);
			set_adjacent_tiles(&mut adjacent_tiles, &black_up, x, y);

			// check black
			let mut qt = 0;
			for (ax, ay) in adjacent_tiles {
				if black_up.contains(&(ax, ay)) {
					qt += 1;
				}
			}
			if !(qt == 0 || qt > 2) {
				new_black.insert((x, y));
			}

			// check if white can become black
			let candidates = adjacent_tiles.clone();
			for (x, y) in candidates {
				set_adjacent_tiles(&mut adjacent_tiles, &black_up, x, y);
				let mut qt = 0;
				for (ax, ay) in adjacent_tiles {
					if black_up.contains(&(ax, ay)) {
						qt += 1;
					}
				}
				if qt == 2 {
					new_black.insert((x, y));
				}
			}
		}
		println!("Day {day}: {}", new_black.len());
		black_up = new_black;
	}

	black_up.len().to_string()
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
        let input = util::read_file("inputs/2020/day24-sample.txt");
        assert_eq!("10", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day24.txt");
        assert_eq!("512", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day24-sample.txt");
        assert_eq!("2208", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day24.txt");
        assert_eq!("4120", part2(input));
    }
}
