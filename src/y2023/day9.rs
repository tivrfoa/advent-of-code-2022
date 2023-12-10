use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn solve_p1(mut nums: &[i32]) -> i32 {
    let mut seqs: Vec<i32> = vec![];
    let mut seq: Vec<i32> = nums.to_vec();
    seqs.push(nums[nums.len() - 1]);
    while seq.iter().find(|&&e| e != 0).is_some() {
        let mut new_seq = Vec::with_capacity(seq.len() - 1);
        for i in 0..seq.len() - 1 {
            new_seq.push(seq[i + 1] - seq[i]);
        }
        seqs.push(new_seq[new_seq.len() - 1]);
        seq = new_seq;
    }

    let mut n = 0;
    for i in (0..seqs.len() - 1).rev() {
        n = n + seqs[i];
    }

    n
}

pub fn part1(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        sum += solve_p1(&line.split_to_nums(' '));
    }

    sum.to_string()
}

fn solve_p2(nums: &[i32]) -> i32 {
    let mut seqs: Vec<i32> = vec![];
    let mut seq: Vec<i32> = nums.to_vec();
    seqs.push(nums[0]);
    while seq.iter().find(|&&e| e != 0).is_some() {
        let mut new_seq = Vec::with_capacity(seq.len() - 1);
        for i in 0..seq.len() - 1 {
            new_seq.push(seq[i + 1] - seq[i]);
        }
        seqs.push(new_seq[0]);
        seq = new_seq;
    }

    let mut n = 0;
    for i in (0..seqs.len() - 1).rev() {
        n = seqs[i] - n;
    }

    n
}

pub fn part2(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        sum += solve_p2(&line.split_to_nums(' '));
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
        assert_eq!("2", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day9.txt");
        assert_eq!("1097", part2(input));
    }
}
