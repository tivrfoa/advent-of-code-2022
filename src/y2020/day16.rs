use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn part1(input: String) -> String {
	let mut error_rate = 0;
	let mut ranges: Vec<(usize, usize, usize, usize)> = vec![];
	let mut my_tickets: Vec<usize> = vec![];
	let mut nearby_tickets: Vec<Vec<usize>> = vec![];
	let mut read_my_tickets = false;
	let mut read_nearby = false;

	for line in input.lines() {
		if line.starts_with("your") || line.starts_with("nearby") {
			continue;
		}
		if line.is_empty() {
			if read_my_tickets {
				read_nearby = true;
			} else {
				read_my_tickets = true;
			}
			continue;
		}
		if read_nearby {
			nearby_tickets.push(line.split_to_nums(','));
		} else if read_my_tickets {
			my_tickets = line.split_to_nums(',');
		} else {
			let tmp: Vec<&str> = line.split(':').collect();
			let (l, r) = tmp[1].trim().split_once(" or ").unwrap();
			let (l1, l2) = l.split_once('-').unwrap();
			let (r1, r2) = r.split_once('-').unwrap();
			ranges.push((
				l1.parse().unwrap(),
				l2.parse().unwrap(),
				r1.parse().unwrap(),
				r2.parse().unwrap()
			));
		}
	}
	// dbg!(ranges, my_tickets, nearby_tickets);

	for near in nearby_tickets {
		'n:
		for n in near {
			for r in &ranges {
				if (r.0 <= n && n <= r.1) || (r.2 <= n && n <= r.3) {
					continue 'n;
				}
			}
			error_rate += n;
			break;
		}
	}

	error_rate.to_string()
}

fn part2(input: String) -> String {
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
        let input = util::read_file("inputs/2020/day16-sample.txt");
        assert_eq!("71", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day16.txt");
        assert_eq!("26941", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day16-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day16.txt");
        assert_eq!("", part2(input));
    }
}
