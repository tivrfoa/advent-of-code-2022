use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn solve(nums: &[i32]) -> i32 {
    let mut seqs: Vec<Vec<i32>> = vec![];
    seqs.push(nums.to_vec());
    while seqs[seqs.len() - 1].iter().find(|&&e| e != 0).is_some() {
        let mut new_seq = Vec::with_capacity(nums.len() - 1);
        let last = &seqs[seqs.len() - 1];
        for i in 0..last.len() - 1 {
            new_seq.push(last[i + 1] - last[i]);
        }
        seqs.push(new_seq);
    }

    let mut n = 0;
    for i in (0..seqs.len() - 1).rev() {
        n = n + seqs[i][seqs[i].len() - 1];
    }

    n
}

pub fn part1(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        sum += solve(&line.split_to_nums(' '));
    }

    sum.to_string()
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
        let input = include_str!("../../inputs/2023/day9-sample.txt");
        assert_eq!("114", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day9.txt");
        assert_eq!("2008960228", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day9-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day9.txt");
        assert_eq!("", part2(input));
    }
}
