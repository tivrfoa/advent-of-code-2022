use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

pub fn part1(input: String) -> String {
	let mut players: Vec<VecDeque<u32>> = vec![VecDeque::new(); 2];
	let mut player_idx = 0;
	for player in input.split("\n\n") {
		let mut lines = player.lines();
		lines.next();
		for line in lines {
			players[player_idx].push_back(line.parse().unwrap());
		}
		player_idx += 1;
	}

	while !players[0].is_empty() && !players[1].is_empty() {
		if players[0][0] > players[1][0] {
			let x = players[0].pop_front().unwrap();
			let y = players[1].pop_front().unwrap();
			players[0].push_back(x);
			players[0].push_back(y);
		} else {
			let x = players[1].pop_front().unwrap();
			let y = players[0].pop_front().unwrap();
			players[1].push_back(x);
			players[1].push_back(y);
		}
	}

	let won = if players[0].is_empty() { 1 } else { 0 };
	let mut score = 0;
	for (i, v) in players[won].iter().rev().enumerate() {
		score += v * (i as u32 + 1);
	}

	score.to_string()
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
        let input = util::read_file("inputs/2020/day22-sample.txt");
        assert_eq!("306", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day22.txt");
        assert_eq!("32472", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day22-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day22.txt");
        assert_eq!("", part2(input));
    }
}
