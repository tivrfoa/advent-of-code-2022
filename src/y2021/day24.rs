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

fn solve(mem: &mut HashSet<(usize, i64, i64, i64, i64)>,
		(w, x, y, z): (i64, i64, i64, i64),
		idx: usize, mut num: String)
		-> (bool, i64, i64, i64, i64) {
	if idx == 14 {
		if z == 0 {
			panic!("Answer: {num}");
		}
		return (false, 0, 0, 0, 0);
	}

	if mem.contains(&(idx, w, x, y, z)) {
		return (false, 0, 0, 0, 0);
	}
	mem.insert((idx, w, x, y, z));

/*
inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
*/
	let (a, b, c) = abc[idx];
	for d in (1..=9).rev() {
		let (mut w2, mut x2, mut y2, mut z2) = (d, x, y, z);
		x2 += z2;
		if x2 < 0 { return (false, 0, 0, 0, 0); }
		x2 = (x2 % 26) + b;
		z2 /= a;
		x2 = if x2 != w2 { 1 } else { 0 };

		y2 = 25 * x2 + 1;
		z2 *= y2;
		y2 = (w2 + c) * x2;
		z2 += y2;

		num.push_str(&mut d.to_string());
		let (rc, w2, x2, y2, z2) = solve(mem, (w2, x2, y2, z2), idx + 1,
			num.clone());
		assert!(rc == false);
		num.pop();
	}

	(false, 0, 0, 0, 0)
}

fn part1(input: String) -> String {
	let mut mem: HashSet<(usize, i64, i64, i64, i64)> = HashSet::new();
	solve(&mut mem, (0, 0, 0, 0), 0, "".into());
	panic!("ops");
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
