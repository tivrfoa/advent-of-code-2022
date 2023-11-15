use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn find_close_paren(chars: &[char]) -> usize {
	let mut qt = 0;

	for i in 0..chars.len() {
		if chars[i] == ')' {
			if qt == 0 {
				return i;
			}
			qt -= 1;
		} else if chars[i] == '(' {
			qt += 1;
		}
	}
	panic!("Failed to find close parentheses");
}

fn eval(chars: &[char]) -> u64 {
	let mut idx = 0;
	let mut no_parens: Vec<u64> = vec![];
	while idx < chars.len() {
		let c = chars[idx];
		if c == '(' {
			let idx_close_paren = idx + 1 + find_close_paren(&chars[idx+1..]);
			no_parens.push(eval(&chars[idx + 1..idx_close_paren]));
			idx = idx_close_paren + 1;
		} else {
			if c == '*' || c == '+' {
				no_parens.push(c as u64);
			} else {
				no_parens.push(c.asu64());
			}
			idx += 1;
		}
	}

	let mut n: u64 = no_parens[0];
	for i in (1..no_parens.len()).step_by(2) {
		let b: u64 = no_parens[i + 1];
		if no_parens[i] == 43 {
			n += b;
		} else {
			n *= b;
		}
	}

	n
}

fn eval2(chars: &[char]) -> u64 {
	let mut idx = 0;
	let mut no_parens: Vec<u64> = vec![];
	while idx < chars.len() {
		let c = chars[idx];
		if c == '(' {
			let idx_close_paren = idx + 1 + find_close_paren(&chars[idx+1..]);
			no_parens.push(eval2(&chars[idx + 1..idx_close_paren]));
			idx = idx_close_paren + 1;
		} else {
			if c == '*' || c == '+' {
				no_parens.push(c as u64);
			} else {
				no_parens.push(c.asu64());
			}
			idx += 1;
		}
	}

	for i in (0..no_parens.len() - 2).step_by(2) {
		if no_parens[i+1] == 43 {
			no_parens[i+2] += no_parens[i];
			no_parens[i+1] = 42;
			no_parens[i] = 1;
		}
	}

	let mut n = no_parens[0];
	for i in (1..no_parens.len()).step_by(2) {
		let b = no_parens[i + 1];
		n *= b;
	}

	n
}

fn part1(input: String) -> String {
	let mut sum = 0;
	for line in input.lines() {
		let tokens: Vec<char> = line.chars().filter(|&c| c != ' ').collect();
		sum += eval(&tokens);
	}

	sum.to_string()
}

fn part2(input: String) -> String {
	let mut sum = 0;
	for line in input.lines() {
		let tokens: Vec<char> = line.chars().filter(|&c| c != ' ').collect();
		sum += eval2(&tokens);
	}

	sum.to_string()
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
        let input = util::read_file("inputs/2020/day18-sample.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day18.txt");
        assert_eq!("31142189909908", part1(input));
    }

    #[test]
    #[ignore]
    fn p2s() {
        let input = util::read_file("inputs/2020/day18-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day18.txt");
        assert_eq!("323912478287549", part2(input));
    }
}
