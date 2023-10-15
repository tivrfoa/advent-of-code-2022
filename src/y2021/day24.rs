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

fn get_reg_idx(s: &str) -> usize {
	match s {
		"w" => 0,
		"x" => 1,
		"y" => 2,
		"z" => 3,
		_ => panic!("{s}"),
	}
}

fn solve(mem: &mut HashMap<(u8, Regs), (bool, Option<String>)>, mut regs: Regs, idx: usize, ops: &[Op]) -> (bool, Option<String>) {

	for i in 0..=3 {
		if regs[i] > 10_000_000 {
			return (false, None);
		}
	}

	if let Some(v) = mem.get(&(idx as u8, regs)) {
		return v.clone();
	}

	if idx == ops.len() {
		return (regs[Z] == 0, Some("".into()));
	}

	let op = &ops[idx];

	if op.op == "inp" {
		for d in (1..=9).rev() {
			regs[0] = d;
			let (ok, v) = solve(mem, regs, idx + 1, ops);
			if ok {
				let mut dig_str = d.to_string();
				if let Some(mut v) = v {
					dig_str.push_str(&mut v);
				}
				// mem.insert((idx, regs), (true, Some(dig_str.clone())));
				return (true, Some(dig_str));
			}
		}

		mem.insert((idx as u8, regs), (false, None));
		return (false, None);
	}

	let a = op.a;
	let b = op.b.as_ref().unwrap();
	let b = match b {
		Var(i) => regs[*i],
		Num(n) => *n,
	};

	match op.op.as_str() {
		"add" => {
			regs[a] += b;
		}
		"mul" => {
			regs[a] *= b;
		}
		"div" => {
			if b == 0 {
				// mem.insert((idx, regs), (false, None));
				return (false, None);
			}
			regs[a] /= b;
		}
		"mod" => {
			if regs[a] < 0 || b < 0 {
				// mem.insert((idx, regs), (false, None));
				return (false, None);
			}
			regs[a] %= b;
		}
		"eql" => {
			regs[a] = if regs[a] == b { 1 } else { 0 };
		}
		_ => panic!("{}", op.op),
	}

	let ret = solve(mem, regs, idx + 1, ops);
	mem.insert((idx as u8, regs), ret.clone());

	ret
}

#[derive(Debug)]
enum VarNum {
	Var(usize),
	Num(i32),
}

use VarNum::*;

impl VarNum {
	fn get_var_num(s: &str) -> VarNum {
		if s == "w" || s == "x" || s == "y" || s == "z" {
			Var(get_reg_idx(s))
		} else {
			Num(s.parse().unwrap())
		}
	}
}

#[derive(Debug)]
struct Op {
	op: String,
	a: usize,
	b: Option<VarNum>,
}

impl Op {
	fn new(op: &str, a: &str, b: Option<&str>) -> Self {
		let a = get_reg_idx(a);
		let b = if let Some(b) = b {
			Some(VarNum::get_var_num(b))
		} else {
			None
		};

		Self {
			op: op.into(),
			a,
			b,
		}
	}

	fn get_ops(s: String) -> Vec<Op> {
		let mut v = vec![];
		for line in s.lines() {
			let line: Vec<&str> = line.split_ascii_whitespace().collect();
			let (op, a, b) = if line.len() == 2 {
				(line[0], line[1], None)
			} else {
				(line[0], line[1], Some(line[2]))
			};
			v.push(Op::new(op, a, b));
		}
		v
	}
}


type Regs = [i32; 4];

fn part1(input: String) -> String {

	let ops = Op::get_ops(input);
	let mut mem: HashMap<(u8, Regs), (bool, Option<String>)> = HashMap::new();

	let (_, v) = solve(&mut mem, [0,0,0,0], 0, &ops);
	v.unwrap()
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
