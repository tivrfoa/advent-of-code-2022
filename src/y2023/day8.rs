use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

pub fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let mut dir_idx = 0;
    let dirs: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut curr = "AAA";

    for line in lines {
        let (k, v) = line.split_once(" = ").unwrap();
        let (l, r) = v[1..v.len() - 1].split_once(", ").unwrap();
        map.insert(k, (l, r));
    }

    let mut steps = 0;
    while curr != "ZZZ" {
        steps += 1;

        if dirs[dir_idx] == 'L' {
            curr = map[curr].0;
        } else {
            curr = map[curr].1;
        }

        dir_idx += 1;
        if dir_idx == dirs.len() {
            dir_idx = 0;
        }
    }

    steps.to_string()
}

pub fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let mut dir_idx = 0;
    let dirs: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut curr: Vec<&str> = vec![];

    for line in lines {
        let (k, v) = line.split_once(" = ").unwrap();
        let (l, r) = v[1..v.len() - 1].split_once(", ").unwrap();
        map.insert(k, (l, r));
        if &k[2..] == "A" {
            curr.push(k);
        }
    }

    let mut steps = 0;
    loop {
        steps += 1;
        let mut qt_z = 0;

        if dirs[dir_idx] == 'L' {
            for i in 0..curr.len() {
                curr[i] = map[curr[i]].0;
                if &curr[i][2..] == "Z" {
                    qt_z += 1;
                }
            }
        } else {
            for i in 0..curr.len() {
                curr[i] = map[curr[i]].1;
                if &curr[i][2..] == "Z" {
                    qt_z += 1;
                }
            }
        }

        if qt_z == curr.len() {
            return steps.to_string();
        }

        dir_idx += 1;
        if dir_idx == dirs.len() {
            dir_idx = 0;
        }
    }
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
        let input = include_str!("../../inputs/2023/day8-sample.txt");
        assert_eq!("2", part1(input));
    }

    #[test]
    fn sample2() {
        let input = include_str!("../../inputs/2023/day8-sample2.txt");
        assert_eq!("6", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day8.txt");
        assert_eq!("24253", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day8-p2-sample.txt");
        assert_eq!("6", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day8.txt");
        assert_eq!("", part2(input));
    }
}
