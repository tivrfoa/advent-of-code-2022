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

const MAX: i64 = i32::MAX as i64;
const MIN: i64 = i32::MIN as i64;

const abc: [(i64, i64, i64); 14] = [
	(1, 13, 8),
	(1, 12, 13),
	(1, 12, 8),
	(1, 10, 10),
	(26, -11, 12),
	(26, -13, 1),
	(1, 15, 13),
	(1, 10, 5),
	(26, -2, 10),
	(26, -6, 3),
	(1, 14, 2),
	(26, 0, 2),
	(26, -15, 12),
	(26, -4, 7),
];

// fn solve(mem: &mut HashSet<(u8, i64, i64, i64, i64)>,
fn solve((mut w, mut x, mut y, mut z): (i64, i64, i64, i64),
		a: i64, b: i64, c: i64, num: String)
		-> (bool, i64, i64, i64, i64) {

	if idx == ops.len() {
		if z == 0 {
			panic!("{num}"); // found answer actually xD
		}
		return false;
	}

	if mem.contains(&(idx as u8, w, x, y, z)) {
		return false;
	}
	mem.insert((idx as u8, w, x, y, z));

	if z > 10_000_000 {
		return (false, None);
	}

	x += z;
	if x < 0 { return false; }
	x = (z + x) % 26 + b;
	z /= a;
	x = if x != w { 1 } else { 0 };

	z = (25 * x + 1) * z;

	y = (w + c) * x;

	z += y;

	return true;
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

fn part1(input: String) -> String {

	let ops = Op::get_ops(input);
	let mut mem: HashMap<(u8, i32, i32, i32, i32), (bool, Option<String>)> = HashMap::new();

	let (_, v) = solve(&mut mem, (0, 0, 0, 0), 0, &ops);
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
