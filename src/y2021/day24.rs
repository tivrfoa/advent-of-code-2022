use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;
use std::thread;

const W: usize = 0;
const Z: usize = 3;

struct ALU {
	regs: [i64; 4],
	digits: Vec<i64>,
	idx: usize,
}

impl ALU {
	fn new(num: &str) -> Self {
		let digits = num.chars()
			.map(|d| d.to_digit(10).unwrap() as i64)
			.collect::<Vec<_>>();
		Self {
			regs: [0; 4],
			digits,
			idx: 0,
		}
	}

	fn get_reg_idx(s: &str) -> usize {
		match s {
			"w" => 0,
			"x" => 1,
			"y" => 2,
			"z" => 3,
			_ => panic!("{s}"),
		}
	}

	fn next_digit(&mut self) -> i64 {
		let i = self.idx;
		self.idx += 1;
		self.digits[i]
	}

	fn do_op(&mut self, op: &str, a: &str, b: Option<&str>) -> bool {
		if op == "inp" {
			self.regs[W] = self.next_digit();
			return true;
		}

		let a = Self::get_reg_idx(a);
		let b = b.unwrap();
		let b: i64 = if b == "w" || b == "x" || b == "y" || b == "z" {
			self.regs[Self::get_reg_idx(b)]
		} else {
			b.parse().unwrap()
		};

		match op {
			"add" => {
				self.regs[a] += b;
			}
			"mul" => {
				self.regs[a] *= b;
			}
			"div" => {
				if b == 0 {
					self.regs[Z] = 1;
					return false;
				}
				self.regs[a] /= b;
			}
			"mod" => {
				if self.regs[a] < 0 || b < 0 {
					self.regs[Z] = 1;
					return false;
				}
				self.regs[a] %= b;
			}
			"eql" => {
				self.regs[a] = if self.regs[a] == b { 1 } else { 0 };
			}
			_ => panic!("{op}"),
		}

		true
	}

	fn is_valid(&self) -> bool {
		self.regs[Z] == 0
	}
}

fn solve(input: String, l: i64, r: i64) -> Option<String> {
	println!("-------------------\n--- {l} to {r} --\n-----------------------");
	let mut ans = None;
	let mut lo: i64 = l;
	let mut hi: i64 = r;

	while lo <= hi {
		let md = lo + (hi - lo) / 2;
		let s_num = md.to_string();
		if s_num.contains("0") {
			lo += 1;
			continue;
		}
		let mut alu = ALU::new(&s_num);

		for line in input.lines() {
			let line: Vec<&str> = line.split_ascii_whitespace().collect();
			let (op, a, b) = if line.len() == 2 {
				(line[0], line[1], None)
			} else {
				(line[0], line[1], Some(line[2]))
			};
			if !alu.do_op(op, a, b) { break; }
		}
		if alu.is_valid() {
			ans = Some(md.to_string());
			lo = md + 1;
		} else {
			// TODO what to do here?!
			lo = md + 1;
		}
	}

	ans
}

fn part1(input: String) -> String {
	let start = 11_111_111_111_111;
	let step = start / 7;
	let mut l = start;
	let mut r = l + step;
	let mut nums = vec![];

	for _ in 0..7 {
		let s = input.clone();
		nums.push(thread::spawn(move || {
			solve(s, l, r)
		}));
		l = r + 1;
		r = l + step;
	}

	for nt in nums.into_iter().rev() {
		if let Some(n) = nt.join().unwrap() {
			return n;
		}
	}

	panic!("Mission failed");

	/*
	*/
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
    #[ignore]
    fn p1s() {
        let input = util::read_file("inputs/2021/day24-sample.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day24.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day24-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day24.txt");
        assert_eq!("", part2(input));
    }
}
