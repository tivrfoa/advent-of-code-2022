use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn part1(input: String) -> String {
	let mut lines = input.lines();
	let start = lines.next().unwrap().parse::<usize>().unwrap();
	let mut min = usize::MAX;
	let mut bus = 0;
	if let Some(line) = lines.next() {
		for id in line.split(',') {
			if id == "x" { continue; }
			let id: usize = id.parse().unwrap();
			let rem = start % id;
			if id - rem < min {
				min = id - rem;
				bus = id;
			}
		}
	}

	(min * bus).to_string()
}

fn part2(input: String) -> String {
	let mut lines = input.lines();
	let _ = lines.next();
	let mut buses = vec![];
	if let Some(line) = lines.next() {
		for (pos, id) in line.split(',').enumerate() {
			if id == "x" { continue; }
			let id: usize = id.parse().unwrap();
			buses.push((pos, id));
		}
	}
	// dbg!(buses);

	let mut t = 0;
	let mut mult = buses[0].1;
	for (pos, id) in buses.into_iter().skip(1) {
		while (t + pos) % id != 0 {
			t += mult;
		}
		mult *= id;
	}

	t.to_string()
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
        let input = util::read_file("inputs/2020/day13-sample.txt");
        assert_eq!("295", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day13.txt");
        assert_eq!("3035", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day13-sample.txt");
        assert_eq!("1068781", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day13.txt");
        assert_eq!("725169163285238", part2(input));
    }
}
