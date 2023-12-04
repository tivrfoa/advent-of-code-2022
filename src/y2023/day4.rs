use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

pub fn part1(input: String) -> String {
	let mut sum = 0;

	for line in input.lines() {
		let mut used = HashSet::new();
		let mut qt = 0;
		let mut p = 1;
		let (_, points) = line.split_once(": ").unwrap();
		let (win, my) = points.split_once(" | ").unwrap();
		let win: Vec<u32> = win.split_to_nums(' ');
		let my: Vec<u32> = my.split_to_nums(' ');

		for n in my {
			if win.contains(&n) && !used.contains(&n) {
				qt += 1;
				if qt > 1 {
					p *= 2;
				}
				used.insert(n);
			}
		}

		sum += if qt == 0 { 0 } else { p };
	}

	sum.to_string()
}

pub fn part2(input: String) -> String {
	let mut sum = 0;
	let mut num_cards: Vec<u32> = vec![1; 200];

	for (i, line) in input.lines().enumerate() {
		let mut used = HashSet::new();
		let mut qt = 0;
		let (_, points) = line.split_once(": ").unwrap();
		let (win, my) = points.split_once(" | ").unwrap();
		let win: Vec<u32> = win.split_to_nums(' ');
		let my: Vec<u32> = my.split_to_nums(' ');

		for n in my {
			if win.contains(&n) && !used.contains(&n) {
				qt += 1;
				used.insert(n);
			}
		}

		for j in i+1..=i+qt {
			num_cards[j] += num_cards[i];
		}

		sum += num_cards[i];
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
        let input = util::read_file("inputs/2023/day4-sample.txt");
        assert_eq!("13", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2023/day4.txt");
        assert_eq!("18519", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2023/day4-sample2.txt");
        assert_eq!("30", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2023/day4.txt");
        assert_eq!("11787590", part2(input));
    }
}
