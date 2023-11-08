use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn part1(input: String) -> String {
	let mut nums: Vec<i32> = input.to_nums();
	nums.sort();
	let mut curr = 0;
	let mut one_qt = 0;
	let mut three_qt = 1;

	for n in nums {
		let diff = n - curr;
		if diff == 1 {
			one_qt += 1;
		} else if diff == 3 {
			three_qt += 1;
		}
		curr = n;
	}

	(one_qt * three_qt).to_string()
}

fn part2(input: String) -> String {
	let mut nums: Vec<i32> = input.to_nums();
	nums.sort();
	let mut mem: HashMap<(usize, i32), u64> = HashMap::new();
	count(&nums, &mut mem, 0, 0).to_string()
}

fn count(nums: &[i32], mem: &mut HashMap<(usize, i32), u64>, l: usize, curr: i32) -> u64 {
	if l == nums.len() {
		return 1;
	}
	if let Some(qt) = mem.get(&(l, curr)) {
		return *qt;
	}
	let mut qt = 0;
	for i in l..nums.len() {
		if nums[i] - curr <= 3 {
			qt += count(nums, mem, i + 1, nums[i]);
		} else {
			break;
		}
	}
	mem.insert((l, curr), qt);
	qt
}

fn part2_elizarov(input: String) -> String {
	let mut nums: Vec<i32> = input.to_nums();
	nums.push(0);
	nums.sort();
	nums.push(nums[nums.len() - 1] + 3);
	let len = nums.len();
	let mut dp = vec![0; len];
	dp[0] = 1i64;
	for i in 1..len {
		let l = if i < 3 { 0 } else { i - 3 };
		for j in l..i {
			if nums[i] - nums[j] <= 3 {
				dp[i] += dp[j];
			}
		}
	}
	dp[len - 1].to_string()
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
        let input = util::read_file("inputs/2020/day10-sample.txt");
        assert_eq!("220", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day10.txt");
        assert_eq!("2470", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day10-sample.txt");
        assert_eq!("19208", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day10.txt");
        // assert_eq!("1973822685184", part2(input));
        assert_eq!("1973822685184", part2_elizarov(input));
    }
}
