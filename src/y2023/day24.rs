use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    a: f64,
    b: f64,
    c: f64,
}

impl Hailstone {
    fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            x,
            y,
            z,
            vx,
            vy,
            vz,
            a: vy,
            b: -vx,
            c: vy * x - vx * y,
        }
    }
}

fn parse(input: &str) -> Vec<Hailstone> {
    let mut ret = vec![];
    for line in input.lines() {
        let line = line.replace(" @ ", ", ");
        let mut it = line.split(", ");
        ret.push(Hailstone::new(
            it.next().unwrap().trim().parse().unwrap(),
            it.next().unwrap().trim().parse().unwrap(),
            it.next().unwrap().trim().parse().unwrap(),
            it.next().unwrap().trim().parse().unwrap(),
            it.next().unwrap().trim().parse().unwrap(),
            it.next().unwrap().trim().parse().unwrap(),
        ));
    }
    ret
}

/*
*
* velocity = distance / time
*
* Solution based on HyperNeutrino
* Advent of Code 2023 Day 24: Never Tell Me The Odds
* https://www.youtube.com/watch?v=guOyA7Ijqgk
*
*/
pub fn part1(input: &str, min: f64, max: f64) -> String {
    let mut total = 0;
    let hailstones = parse(input);

    for (i, hs1) in hailstones.iter().enumerate() {
        for hs2 in &hailstones[i+1..] {
            let (a1, b1, c1) = (hs1.a, hs1.b, hs1.c);
            let (a2, b2, c2) = (hs2.a, hs2.b, hs2.c);
            if a1 * b2 == b1 * a2 {
                // parallel lines 
                continue;
            }
            let x = (c1 * b2 - c2 * b1) / (a1 * b2 - a2 * b1);
            let y = (c2 * a1 - c1 * a2) / (a1 * b2 - a2 * b1);

            if is_between(x, min, max) && is_between(y, min, max) {
                if [hs1, hs2].iter().filter(|&&hs| {
                    (x - hs.x) * hs.vx >= 0.0 && (y - hs.y) * hs.vy >= 0.0
                }).count() == 2 {
                    total += 1;
                }
            }
        }
    }
    total.to_string()
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
        let input = include_str!("../../inputs/2023/day24-sample.txt");
        assert_eq!("2", part1(input, 7., 27.));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day24.txt");
        assert_eq!("28174", part1(input, 200000000000000., 400000000000000.0));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day24-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day24.txt");
        assert_eq!("", part2(input));
    }
}
