use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

fn part1(input: String) -> String {
	let nums = util::input_as_vec_i32(&input);
	for i in 0..nums.len() - 1 {
		for j in i+1..nums.len() {
			if nums[i] + nums[j] == 2020 {
				return (nums[i] * nums[j]).to_string();
			}
		}
	}
	panic!("invalid");
}

fn part2(input: String) -> String {
	let nums = util::input_as_vec_i32(&input);
	for i in 0..nums.len() - 1 {
		for j in i+1..nums.len() {
			for z in j+1..nums.len() {
				if nums[i] + nums[j] + nums[z] == 2020 {
					return (nums[i] * nums[j] * nums[z]).to_string();
				}
			}
		}
	}
	panic!("invalid");
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
        let input = util::read_file("inputs/2020/day1-sample.txt");
        assert_eq!("514579", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day1.txt");
        assert_eq!("567171", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day1-sample.txt");
        assert_eq!("241861950", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day1.txt");
        assert_eq!("212428694", part2(input));
    }
}
