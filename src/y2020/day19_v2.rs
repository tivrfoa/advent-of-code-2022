use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

#[derive(Debug)]
enum Rule {
	Char(u8),
	Seq(Vec<usize>),
	SeqOr(Vec<usize>, Vec<usize>),
}

fn parse(lines: &mut std::str::Lines) -> Vec<Rule> {
	let mut rules: Vec<(usize, Rule)> = vec![];
	while let Some(line) = lines.next() {
		if line.is_empty() { break; }
		let (k, v) = line.split_delim(':');
		let k: usize = k.parse().unwrap();
		let v = v.trim().split_space();
		if v[0].starts_with("\"") {
			let tmp: Vec<&str> = v[0].split('"').collect();
			let c = tmp[1].chars().next().unwrap() as u8;
			rules.push((k, Rule::Char(c)));
		} else if line.contains('|') {
			let mut left: Vec<usize> = vec![];
			let mut nums = vec![];
			for i in 0..v.len() {
				if v[i] == "|" {
					left = nums;
					nums = vec![];
				} else {
					nums.push(v[i].parse().unwrap());
				}
			}

			rules.push((k, Rule::SeqOr(left, nums)));
		} else {
			let mut nums = vec![];
			for n in v {
				nums.push(n.parse().unwrap());
			}
			rules.push((k, Rule::Seq(nums)));
		}
	}
	rules.sort_unstable_by_key(|r| r.0);
    rules.into_iter().map(|r| r.1).collect()
}

fn part1(input: String) -> String {
	let mut qt = 0;
	let mut lines = input.lines();
	let mut rules: Vec<Rule> = parse(&mut lines);
	let msgs: Vec<&str> = lines.collect();

	for msg in msgs {
		if let Some(q) = matches(msg.as_bytes(), &rules, 0) {
			if q == msg.len() {
				qt += 1;
			}
		}
	}

	qt.to_string()
}

fn part2(input: String) -> String {
	let mut qt = 0;
	let mut lines = input.lines();
	let mut rules: Vec<Rule> = parse(&mut lines);
	let msgs: Vec<&str> = lines.collect();

	for msg in msgs {
		if let Some(q) = matches(msg.as_bytes(), &rules, 0) {
			if q == msg.len() {
				qt += 1;
			}
		}
	}

	qt.to_string()
}

fn matches(msg: &[u8], rules: &[Rule], rule: usize) -> Option<usize> {
    if msg.is_empty() {
        return None;
    }

    match &rules[rule] {
        Rule::Char(c) if &msg[0] == c => Some(1),
        Rule::Char(_) => None,
        Rule::Seq(r) => r
            .into_iter()
            .try_fold(0, |c, r| matches(&msg[c..], rules, *r).map(|n| n + c)),
        Rule::SeqOr(r, s) => r
            .into_iter()
            .try_fold(0, |c, r| matches(&msg[c..], rules, *r).map(|n| n + c))
            .or_else(|| {
                s.into_iter()
                    .try_fold(0, |c, r| matches(&msg[c..], rules, *r).map(|n| n + c))
            }),
    }
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
        let input = util::read_file("inputs/2020/day19-sample.txt");
        assert_eq!("2", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day19.txt");
        assert_eq!("205", part1(input));
    }

    #[test]
    #[ignore]
    fn p2s() {
        let input = util::read_file("inputs/2020/day19-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day19_p2.txt");
        assert_eq!("", part2(input));
    }
}
