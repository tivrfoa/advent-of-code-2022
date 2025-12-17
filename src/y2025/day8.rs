use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use new_derive::New;

use util::*;

#[derive(Debug, New)]
struct In {
	x: usize,
	y: usize,
	z: usize,
}

fn parse(input: &str) -> Vec<In> {
	let mut ret = vec![];

	for l in input.lines() {
		let mut vv = l.split(',');
		let x = vv.next().unwrap().parse::<usize>().unwrap();
		let y = vv.next().unwrap().parse::<usize>().unwrap();
		let z = vv.next().unwrap().parse::<usize>().unwrap();
		ret.push(In::new(x, y, z));
	}

	ret
}

fn solve(input: &[In], max_conn: usize) -> usize {

	0
}

pub fn part1(input: &str) -> String {
	let v = parse(input);
	dbg!(v);
    "todo".into()
}

pub fn part2(input: &str) -> String {
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
        other
            .cost
            .cmp(&self.cost)
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
        let input = include_str!("../../inputs/2025/day8-sample.txt");
        assert_eq!("", part1(input));
    }

    //#[test]
    //fn p1() {
    //    let input = include_str!("../../inputs/2025/day8.txt");
    //    assert_eq!("", part1(input));
    //}

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2025/day8-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2025/day8.txt");
        assert_eq!("", part2(input));
    }
}
