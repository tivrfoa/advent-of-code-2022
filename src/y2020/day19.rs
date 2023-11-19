use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

#[derive(Debug)]
struct Rule {
	c: char,
	rules: Vec<usize>,
}

fn parse(lines: &mut std::str::Lines) -> HashMap<usize, Rule> {
	let mut rules: HashMap<usize, Rule> = HashMap::new();
	while let Some(line) = lines.next() {
		if line.is_empty() { break; }
		let (k, v) = line.split_delim(':');
		let k: usize = k.parse().unwrap();
		let v = v.trim().split_space();
		if v[0].starts_with("\"") {
			let tmp: Vec<&str> = v[0].split('"').collect();
			let c = tmp[1].as_char();
			rules.insert(k, Rule {
				c,
				rules: vec![],
			});
		} else {
			let mut nums = vec![];
			for n in v {
				if n != "|" {
					nums.push(n.parse().unwrap());
				}
			}
			rules.insert(k, Rule {
				c: 'r',
				rules: nums,
			});
		}
	}

	rules
}

fn compute_rule(computed_rules: &mut Vec<Option<Vec<String>>>,
		rules: &HashMap<usize, Rule>, rule_idx: usize) -> Vec<String> {

	if let Some(comp) = &computed_rules[rule_idx] {
		return comp.clone();
	}
	let rule = &rules[&rule_idx];
	if rule.c != 'r' {
		let v = vec![rule.c.to_string()];
		computed_rules[rule_idx] = Some(v.clone());
		return v;
	}

	let mut comp: Vec<String> = vec![];
	if rule.rules.len() == 4 {
		let mut left: Vec<String> = vec![];
		for idx in 0..2 {
			let i = rule.rules[idx];
			if left.is_empty() {
				left = compute_rule(computed_rules, rules, i);
			} else {
				let mut r = compute_rule(computed_rules, rules, i);
				let mut new_comp = Vec::with_capacity(r.len() * left.len());
				let len = left.len();
				for l in 0..len {
					for ri in 0..r.len() {
						let mut s = left[l].clone();
						s.push_str(&mut r[ri]);
						new_comp.push(s);
					}
				}
				left = new_comp;
			}
		}
		comp = left;

		let mut left: Vec<String> = vec![];
		for idx in 2..4 {
			let i = rule.rules[idx];
			if left.is_empty() {
				left = compute_rule(computed_rules, rules, i);
			} else {
				let mut r = compute_rule(computed_rules, rules, i);
				let mut new_comp = Vec::with_capacity(r.len() * left.len());
				let len = left.len();
				for l in 0..len {
					for ri in 0..r.len() {
						let mut s = left[l].clone();
						s.push_str(&mut r[ri]);
						new_comp.push(s);
					}
				}
				left = new_comp;
			}
		}

		comp.append(&mut left);
	} else {
		for i in &rule.rules {
			if comp.is_empty() {
				comp = compute_rule(computed_rules, rules, *i);
			} else {
				let mut r = compute_rule(computed_rules, rules, *i);
				let mut new_comp = Vec::with_capacity(r.len() * comp.len());
				let len = comp.len();
				for l in 0..len {
					for ri in 0..r.len() {
						let mut s = comp[l].clone();
						s.push_str(&mut r[ri]);
						new_comp.push(s);
					}
				}
				comp = new_comp;
			}
		}
	}
	computed_rules[rule_idx] = Some(comp.clone());
	comp
}

fn part1(input: String) -> String {
	let mut qt = 0;
	let mut lines = input.lines();
	let mut rules: HashMap<usize, Rule> = parse(&mut lines);
	let mut msgs: Vec<&str> = lines.collect();
	let mut computed_rules: Vec<Option<Vec<String>>> = vec![None; rules.len()];
	let zero_rules = compute_rule(&mut computed_rules, &rules, 0);
	dbg!(computed_rules);

	for msg in msgs {
		if zero_rules.contains(&msg.to_string()) {
			qt += 1;
		}
	}

	qt.to_string()
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
        let input = util::read_file("inputs/2020/day19-sample.txt");
        assert_eq!("2", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day19.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day19-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day19.txt");
        assert_eq!("", part2(input));
    }
}
