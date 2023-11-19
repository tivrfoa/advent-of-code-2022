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

fn part2(input: String) -> String {
	let mut qt = 0;
	let mut lines = input.lines();
	let mut rules: Vec<Rule> = parse(&mut lines);
	let msgs: Vec<&str> = lines.collect();

	for msg in msgs {
		if matches_42(msg.as_bytes(), &rules) {
			qt += 1;
		}
	}

	qt.to_string()
}


fn matches_42(msg: &[u8], rules: &[Rule]) -> bool {
    (0..)
        .try_fold(msg, |msg, depth| match matches(msg, 42, rules) {
            Some(msg) if matches_31(depth, msg, rules) => Err(true),
            Some(msg) => Ok(msg),
            None => Err(false),
        })
        .err()
        .unwrap()
}

fn matches_31(depth: usize, msg: &[u8], rules: &[Rule]) -> bool {
    (0..depth)
        .try_fold(msg, |msg, _| match matches(msg, 31, rules) {
            Some(msg) if msg.is_empty() => Err(true),
            Some(msg) => Ok(msg),
            None => Err(false),
        })
        .err()
        .unwrap_or(false)
}

fn matches<'a>(msg: &'a [u8], rule: usize, rules: &[Rule]) -> Option<&'a [u8]> {
    match &rules[rule] {
        Rule::Char(_) if msg.is_empty() => None,
        Rule::Char(c) if &msg[0] == c => Some(&msg[1..]),
        Rule::Char(_) => None,
        Rule::Seq(a) => a.into_iter().try_fold(msg, |m, &r| matches(m, r, rules)),
        Rule::SeqOr(a, b) => a
            .into_iter()
            .try_fold(msg, |m, &r| matches(m, r, rules))
            .or_else(|| b.into_iter().try_fold(msg, |m, &r| matches(m, r, rules))),
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
