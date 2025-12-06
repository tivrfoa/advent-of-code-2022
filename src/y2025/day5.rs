use crate::util;

use std::cell::Cell;
use std::cmp::{max, Ordering};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

#[derive(Debug)]
struct Interval {
	start: usize,
	end: usize,
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start
            .cmp(&other.start)
            .then(self.end.cmp(&other.end))
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for Interval {}

/**

I think we can keep a separate list of only the start positions ...


when search a new ID, I can simulating it is an interval, like: ID-0

*/
impl Interval {
	fn parse(input: &str) -> (Vec<Interval>, Vec<usize>) {
		let mut intervals = vec![];
		let mut ids = vec![];
		let mut lines = input.lines();

		while let Some(line) = lines.next() {
			if line.is_empty() { break; }
			let (l, r) = line.split_once('-').unwrap();
			let l: usize = l.parse().unwrap();
			let r: usize = r.parse().unwrap();
			let interval = Self::new(l, r);
			Self::add_interval(&mut intervals, interval);
		}
		
		while let Some(line) = lines.next() {
			ids.push(line.parse().unwrap());
		}

		(intervals, ids)
	}

	fn new(start: usize, end: usize) -> Self {
		Self { start, end }
	}

	fn merge_right(intervals: &mut Vec<Interval>, idx: usize) {
		while idx + 1 < intervals.len() && intervals[idx].end >= intervals[idx + 1].start {
			intervals[idx].end = max(intervals[idx].end, intervals[idx + 1].end);
			intervals.remove(idx + 1);
		}
	}

	fn add_interval(intervals: &mut Vec<Interval>, new_interval: Interval) {
		if intervals.is_empty() {
			intervals.push(new_interval);
			return;
		}
		match intervals.binary_search(&new_interval) {
			Ok(idx) => {
				eprintln!("repeated interval {}-{}", new_interval.start,
					new_interval.end);
			}
			Err(idx) => {
				if idx == 0 {
					if new_interval.end >= intervals[idx].start {
						intervals[idx].start = new_interval.start;
						intervals[idx].end = max(intervals[idx].end, new_interval.end);
						Self::merge_right(intervals, idx);
					} else {
						intervals.insert(idx, new_interval);
					}
				} else if idx == intervals.len() {
					if intervals[idx - 1].end >= new_interval.start {
						intervals[idx - 1].end = max(intervals[idx - 1].end, new_interval.end);
					} else {
						intervals.push(new_interval);
					}
				} else {
					if intervals[idx - 1].end >= new_interval.start {
						intervals[idx - 1].end = max(intervals[idx - 1].end, new_interval.end);
						Self::merge_right(intervals, idx - 1);
					} else if new_interval.end >= intervals[idx].start {
						intervals[idx].start = new_interval.start;
						intervals[idx].end = max(intervals[idx].end, new_interval.end);
						Self::merge_right(intervals, idx);
					} else {
						intervals.insert(idx, new_interval);
					}
				}
			}
		}
	}
}

/*
The brute force approach is easy to implement:
	- just check for every range

Other approaches:

1. we can merge intervals
2. Merge new intervals if they overlap. Keep them sorted otherwise.


? Is there a fast way to find overlaps in existing intervals?

1. binary search the interval based on the start position to find the place
where it should be inserted;
2. if there is an interval before it, check if the end of the previous interval
is greater than the new interval start position;
2.1. yes -> merge
2.2. no -> insert the interval in that position

*/
pub fn part1(input: &str) -> String {
	let mut ans = 0;
	let (intervals, ids) = Interval::parse(input);
	let len = intervals.len();
	//dbg!(&intervals, &ids);
	for id in ids {
		let fake_interval = Interval::new(id, usize::MAX);
		match intervals.binary_search(&fake_interval) {
			Ok(idx) => panic!("impossible for fake interval"),
			Err(idx) => {
				if idx == 0 { continue; }
				if idx == len {
					if intervals[len - 1].end >= id {
						// println!("1 - {id} is fresh - idx {idx}");
						ans += 1;
					}
				} else if intervals[idx - 1].start == id || intervals[idx - 1].end >= id {
					// println!("2 - {id} is fresh - idx {idx}");
					ans += 1;
				}
			}
		}
	}
	ans.to_string()
}

pub fn part2(input: &str) -> String {
	let mut ans = 0;
	let (intervals, _) = Interval::parse(input);
	for i in intervals {
		ans += i.end - i.start + 1;
	}
	ans.to_string()
}

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
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
        let input = include_str!("../../inputs/2025/day5-sample.txt");
        assert_eq!("3", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2025/day5.txt");
        assert_eq!("640", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2025/day5-sample.txt");
        assert_eq!("14", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2025/day5.txt");
        assert_eq!("365804144481581", part2(input));
    }
}
