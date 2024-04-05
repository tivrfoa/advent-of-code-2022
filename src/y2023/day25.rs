use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

// Solution copied from:
// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day25.rs

fn perfect_minimal_hash(lookup: &mut [usize], nodes: &mut Vec<Vec<usize>>, slice: &[u8]) -> usize {
    // Base 26 index.
    let hash = slice[..3].iter().fold(0, |acc, b| 26 * acc + ((b - b'a') as usize));
    let mut index = lookup[hash];

    // First time seeing this key so push a new node and return its index.
    if index == usize::MAX {
        index = nodes.len();
        lookup[hash] = index;
        nodes.push(Vec::with_capacity(10));
    }

    index
}

struct Input {
    edges: Vec<usize>,
    nodes: Vec<(usize, usize)>,
}

impl Input {
    #[inline]
    fn neighbours(&self, node: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (start, end) = self.nodes[node];
        (start..end).map(|edge| (edge, self.edges[edge]))
    }
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut ret = HashMap::new();
    for line in input.lines() {
        let (k, r) = line.split_once(": ").unwrap();
        let mut connections = vec![];
        for conn in r.split(' ') {
            connections.push(conn);
            ret.entry(conn).or_insert(vec![]).push(k);
        }
        ret.entry(k).or_insert(vec![]).append(&mut connections);
    }
    ret
}

pub fn part1(input: &str) -> String {
    let components = parse(input);
    dbg!(components);
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
        let input = include_str!("../../inputs/2023/day25-sample.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day25.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day25-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day25.txt");
        assert_eq!("", part2(input));
    }
}
