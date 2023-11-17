use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn part1(nums: Vec<usize>) -> String {
    let mut last = nums[nums.len() - 1];
	let mut ss: Vec<(usize, usize)> = vec![(0, 0); 7_000_000];
	let mut t = 1;
	for n in nums {
		ss[n] = (t, 0);
		t += 1;
	}

	while t <= 2020 {
		let (l, r) = ss[last];
		let mut next = 0;
		if r != 0 {
			next = r - l;
		}
        if ss[next].1 == 0 {
            if ss[next].0 == 0 {
                ss[next].0 = t;
            } else {
                ss[next].1 = t;
            }
        } else {
            ss[next].0 = ss[next].1;
            ss[next].1 = t;
        }
		last = next;
		t += 1;
	}

	last.to_string()
}

fn part2(nums: Vec<usize>) -> String {
    let mut last = nums[nums.len() - 1];
    let mut ss: HashMap<usize, (usize, usize)> = HashMap::new();
	let mut t = 1;
	for n in nums {
		ss.insert(n, (t, 0));
		t += 1;
	}

	while t <= 30000000 {
		let (l, r) = ss[&last];
		let mut next = 0;
		if r != 0 {
			next = r - l;
		}
        if ss.contains_key(&next) {
            if let Some(m) = ss.get_mut(&next) {
                if m.1 == 0 {
                    m.1 = t;
                } else {
                    m.0 = m.1;
                    m.1 = t;
                }
            }
        } else {
            ss.insert(next, (t, 0));
        }
		last = next;
		t += 1;
	}

	last.to_string()
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
        // let input = util::read_file("inputs/2020/day15-sample.txt");
        assert_eq!("436", part1(vec![0,3,6]));
    }

    #[test]
    fn p1() {
        // let input = util::read_file("inputs/2020/day15.txt");
        assert_eq!("249", part1(vec![15,12,0,14,3,1]));
    }

    #[test]
    fn p2s() {
        assert_eq!("175594", part2(vec![0,3,6]));
    }

    #[test]
    fn p2() {
        assert_eq!("41687", part2(vec![15,12,0,14,3,1]));
    }
}
