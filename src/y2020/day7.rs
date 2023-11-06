use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Bag {
	name: String,
	qt: u32,
	bags: Vec<Bag>,
}

impl Bag {
	fn new(name: String, qt: &str) -> Self {
		Self {
			name,
			qt: qt.parse().unwrap(),
			bags: vec![],
		}
	}

	fn set_bags(&mut self, bags: Vec<Bag>) {
		self.bags = bags;
	}
}

fn part1(input: String) -> String {
	let mut ans = HashSet::new();
	let mut bags = vec![];
	for line in input.lines() {
		let (l, r) = line.split_once(" contain ").unwrap();
		let mut inner_bags = vec![];
		if !r.starts_with("no") {
			for b in r.split(", ") {
				let (qt, name) = b.split_delim(' ');
				inner_bags.push(Bag::new(name.into(), qt));
			}
		}
		let mut bag = Bag::new(l.into(), "0");
		bag.set_bags(inner_bags);
		bags.push(bag);
	}

	let mut find = vec!["shiny gold"];
	loop {
		let mut new_find = vec![];
		for f in find {
			for outer in bags.iter() {
				if ans.contains(outer) { continue; }
				for b in outer.bags.iter() {
					if b.name == f {
						ans.insert(outer);
						new_find.push(outer.name.as_str());
					}
				}
			}
		}
		if new_find.is_empty() { break; }
		find = new_find;
	}

	ans.len().to_string()
}

fn part2(input: String) -> String {
	let mut ans = 0;
	let mut bags: HashMap<&str, Vec<(&str, u32)>> = HashMap::new();
	for line in input.lines() {
		let (l, r) = line.split_once(" contain ").unwrap();
		let mut inner_bags = vec![];
		if !r.starts_with("no") {
			for b in r.split(", ") {
				let (qt, name) = b.split_delim(' ');
				inner_bags.push((name, qt.parse::<u32>().unwrap()));
			}
		}
		bags.insert(l, inner_bags);
	}

	count_inner("shiny gold", &bags).to_string()
}

fn count_inner(outer: &str, bags: &HashMap<&str, Vec<(&str, u32)>>) -> u32 {
	let mut qt = 0;
	for (name, q) in &bags[outer] {
		qt += q + q * count_inner(name, bags);
	}
	qt
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
        let input = util::read_file("inputs/2020/day7-sample.txt");
        assert_eq!("4", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day7.txt");
        assert_eq!("252", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day7-sample.txt");
        assert_eq!("32", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day7.txt");
        assert_eq!("35487", part2(input));
    }
}
