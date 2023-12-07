use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

pub fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let t: Vec<u32> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let d: Vec<u32> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut ways = 1;
    let races = t.len();
    for i in 0..races {
        let mut qt = 0;
        for c in 1..t[i] {
            let dist = (t[i] - c) * c;
            if dist > d[i] {
                qt += 1;
            }
        }
        ways *= qt;
    }

    ways.to_string()
}

// the way this is implemented only works if t / 2
// and t / 2 + 1 are better times
fn binary_search(t: u64, d: u64) -> u64 {
    let half = t / 2;

    // lowest t
    let mut lo = 0;
    let mut hi = half;
    while lo <= hi {
        let md = lo + (hi - lo) / 2;
        let dist = (t - md) * md;
        if dist > d {
            hi = md - 1;
        } else {
            lo = md + 1;
        }
    }

    let left = lo;

    // highest t
    let mut lo = half + 1;
    let mut hi = t - 1;
    while lo <= hi {
        let md = lo + (hi - lo) / 2;
        let dist = (t - md) * md;
        if dist > d {
            lo = md + 1;
        } else {
            hi = md - 1;
        }
    }

    hi - left + 1
}

pub fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let t: u64 = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let d: u64 = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    binary_search(t, d).to_string()
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
        let input = include_str!("../../inputs/2023/day6-sample.txt");
        assert_eq!("288", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day6.txt");
        assert_eq!("211904", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day6-sample.txt");
        assert_eq!("71503", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day6.txt");
        assert_eq!("43364472", part2(input));
    }
}
