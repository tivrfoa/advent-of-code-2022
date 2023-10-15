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

fn solve((mut w, mut x, mut y, mut z): (i64, i64, i64, i64),
		a: i64, b: i64, c: i64)
		-> (bool, i64, i64, i64, i64) {

	// if z > 10_000_000 {
	// 	return (false, 0, 0, 0, 0);
	// }

	x += z;
	if x < 0 { return (false, 0, 0, 0, 0); }
	x = (x % 26) + b;
	z /= a;
	x = if x != w { 1 } else { 0 };

	z = (25 * x + 1) * z;

	y = (w + c) * x;

	z += y;

	return (true, w, x, y, z);
}

fn part1(input: String) -> String {

	let mut rc = true;
	let (mut w, mut x, mut y, mut z) = (0, 0, 0, 0);
	let mut idx = 0;
	let mut num = String::with_capacity(14);

	while idx < 14 {
		for d in (1..=9).rev() {
			num.push_str(&mut d.to_string());
			let (a, b, c) = abc[idx];
			(rc, w, x, y, z) = solve((d, x, y, z), a, b, c);
			if rc {
				idx += 1;
				break;
			} else {
				num.pop();
			}
		}
	}
	num
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
