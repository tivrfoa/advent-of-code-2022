use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn part1(input: String) -> String {
	let nums: Vec<i64> = input.to_nums();
	'i:
	for i in 25..nums.len() {
		let mut valid = false;
		for j in 0..i - 1 {
			for z in j+1..i {
				if nums[j] != nums[z] && nums[j] + nums[z] == nums[i] {
					valid = true;
					continue 'i;
				}
			}
		}
		if !valid {
			return nums[i].to_string();
		}
	}
	panic!("Failed");
}

fn part2(input: String) -> String {
	let target = 1930745883;
	let nums: Vec<i64> = input.to_nums();
	let mut l = 0;
	let mut r = 665;
	for l in 0..=664 {
		for r in (l+1..=665).rev() {
			if nums[l..=r].iter().sum::<i64>() == target {
				let min = nums[l..=r].iter().min().unwrap();
				let max = nums[l..=r].iter().max().unwrap();
				return (min + max).to_string();
			}
		}
	}
	panic!("Failed");
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
    #[ignore]
    fn p1s() {
        let input = util::read_file("inputs/2020/day9-sample.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day9.txt");
        assert_eq!("1930745883", part1(input));
    }

    #[test]
    #[ignore]
    fn p2s() {
        let input = util::read_file("inputs/2020/day9-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day9.txt");
        assert_eq!("268878261", part2(input));
    }
}
