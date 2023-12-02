use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

const M: u64 = 20201227;
const SUBJECT: u64 = 7;

pub fn part1(input: String) -> String {
	let mut lines = input.lines();
	let card_pk: u64 = lines.next().unwrap().parse().unwrap();
	let door_pk: u64 = lines.next().unwrap().parse().unwrap();
	let mut curr = 1;
	let mut card_loop_size = 0;
	let mut door_loop_size = 0;

	for loop_size in 1..10000000 {
		curr *= SUBJECT;
		curr %= M;
		if curr == card_pk {
			card_loop_size = loop_size;
			break;
		}
		if curr == door_pk {
			door_loop_size = loop_size;
			break;
		}
	}

	let (pk, loop_size) = {
		if card_loop_size > 0 {
			(door_pk, card_loop_size)
		} else {
			(card_pk, door_loop_size)
		}
	};
	let mut encryption_key = 1;
	for _ in 0..loop_size {
		encryption_key *= pk;
		encryption_key %= M;
	}

	encryption_key.to_string()
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
        let input = util::read_file("inputs/2020/day25-sample.txt");
        assert_eq!("14897079", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day25.txt");
        assert_eq!("16457981", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day25-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day25.txt");
        assert_eq!("", part2(input));
    }
}
