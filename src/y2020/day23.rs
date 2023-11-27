use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn pick(cups: &[usize], curr: usize, n: usize) -> (usize, usize, bool) {
	(0, 0, true)
}

fn update_cups(cups: Vec<usize>, picked: &[usize],
		destination_idx: usize) -> Vec<usize> {
	let len = cups.len();
	let mut new_cups = cups.clone();
	let mut pos_to_fill = (curr + 1) % len;
	let mut pos_to_look = (curr + 4) % len;

	for _ in 0..4 {
		if pos_to_look == destination_idx {
			break;
		}
		new_cups[pos_to_fill] = cups[pos_to_look];

		pos_to_fill = (pos_to_fill + 1) % len;
		pos_to_look = (pos_to_look + 1) % len;
	}

	new_cups[pos_to_fill] = cups[pos_to_look];

	for i in 0..picked.len() {
		new_cups[pos_to_fill] = picked[i];

		pos_to_fill = (pos_to_fill + 1) % len;
	}

	new_cups
}

fn find_lowest_value(cups: &[usize], curr_idx: usize) -> usize {
	let mut i = (curr_idx + 4) % cups.len();
	let mut min = usize::MAX;

	while i != curr_idx {
		if cups[i] < min {
			min = cups[i];
		}

		i = (i + 1) % cups.len();
	}
	if wrapped {
		for i in l+1..r {
			if i == curr { continue; }
			if cups[i] < min { min = cups[i]; }
		}
		min
	} else {
		let mut min = usize::MAX;
		for i in 0..l {
			if i == curr { continue; }
			if cups[i] < min { min = cups[i]; }
		}
		for i in r+1..cups.len() {
			if i == curr { continue; }
			if cups[i] < min { min = cups[i]; }
		}
		min
	}
}

fn find_destination(cups: &[usize], curr: usize, l: usize, r: usize, wrapped: bool, lowest: usize) -> usize {
	let mut target = cups[curr] - 1;
	for target in (lowest..cups[curr]).rev() {
		if wrapped {
			for i in l+1..r {
				if cups[i] == target {
					return i;
				}
			}
		} else {
			for i in 0..l {
				if cups[i] == target {
					return i;
				}
			}
			for i in r+1..cups.len() {
				if cups[i] == target {
					return i;
				}
			}
		}
	}

	// find highest
	let mut max_idx = 0;
	if wrapped {
		for i in l+1..r {
			if i == curr { continue; }
			if cups[i] > cups[max_idx] {
				max_idx = i;
			}
		}
	} else {
		for i in 0..l {
			if i == curr { continue; }
			if cups[i] > cups[max_idx] {
				max_idx = i;
			}
		}
		for i in r+1..cups.len() {
			if i == curr { continue; }
			if cups[i] > cups[max_idx] {
				max_idx = i;
			}
		}
	}

	max_idx
}

pub fn part1(input: String) -> String {
	let mut cups: Vec<usize> = input.lines().next().unwrap().chars().map(|c| c.to_decimal()).collect();
	let len = cups.len();
	let mut curr = 0;

	for _ in 0..100 {
		let (l, r, wrapped) = pick(&cups, curr, 3);
		let lowest_value = find_lowest(&cups, l, r, wrapped);
		let destination_idx = find_destination(&cups, l, r, wrapped, lowest_value);
		cups = update_cups(cups, &picked, destination_idx);
		curr = (curr + 1) % len;
	}

	cups.into_iter().map(|i| i.to_string()).collect()
}

pub fn part2(input: String) -> String {
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
        let input = util::read_file("inputs/2020/day23-sample.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day23.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day23-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day23.txt");
        assert_eq!("", part2(input));
    }
}
