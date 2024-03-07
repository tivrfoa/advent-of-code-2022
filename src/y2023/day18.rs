use crate::util;

use util::*;

/*
 * Solution translated from Betaveros
 * https://www.youtube.com/watch?v=Cs7UMXAFV7Q
 */
use std::cell::Cell;
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;
use std::ops::{Add, AddAssign, Mul};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct V(i32, i32);

impl V {
    fn times(self, t: i32) -> Self {
        Self(self.0 * t, self.1 * t)
    }
}

impl Add for V {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for V {
    fn add_assign(&mut self, other: Self) {
        *self = self.add(other);
    }
}

impl Mul for V {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }
}

fn dbg_grid(grid: &[Vec<char>]) {
    for row in grid {
        println!("{:?}", row);
    }
}
/*
let directions: HashMap<&str, Point2D> = HashMap::from([
    ("3", V(-1, 0)),
    ("0", V(0, 1)),
    ("1", V(1, 0)),
    ("2", V(0, -1)),
]);
*/
const DIRECTIONS: [V; 4] = [
    V(0, 1),
    V(1, 0),
    V(0, -1),
    V(-1, 0),
];

fn get_dir_idx(c: char) -> usize {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        _ => panic!("{}", c),
    }
}

fn inv(v: &[i32]) -> HashMap<i32, i32> {
    v.iter().enumerate().map(|(i, x)| (*x, i as i32)).collect()
}

fn fill(pos: V, seen: &mut HashSet<V>) {
    if !seen.insert(pos) {
        return;
    }
    for delta in DIRECTIONS {
        fill(pos + delta, seen);
    }
}


pub fn part1(input: &str) -> String {
    "todo".into()
}

pub fn part2(input: &str) -> String {
    let mut cur = V(0, 0);
    let mut rs = vec![0, 1];
    let mut cs = vec![0, 1];
    for line in input.lines() {
        let tokens: Vec<&str> = line.split(' ').collect();
        let color = tokens[2];
        let count = i32::from_str_radix(&color[2..7], 16).unwrap();
        let d = color[7..8].chars().next().unwrap();
        let direction = DIRECTIONS[get_dir_idx(d)];
        cur += direction.times(count);
        rs.push(cur.0);
        rs.push(cur.0 + 1);
        cs.push(cur.1);
        cs.push(cur.1 + 1);
    }
    assert!(cur == V(0, 0));

    rs.sort();
    cs.sort();

    // r_inv = {x: i for i, x in enumerate(rs)}
    let r_inv = inv(&rs);
    let c_inv = inv(&cs);
    let mut seen: HashSet<V> = HashSet::new();
    seen.insert(V(r_inv[&0], c_inv[&0]));
    for line in input.lines() {
        let tokens: Vec<&str> = line.split(' ').collect();
        let color = tokens[2];
        let count = i32::from_str_radix(&color[2..7], 16).unwrap();
        let d = color[7..8].chars().next().unwrap();
        let direction = DIRECTIONS[get_dir_idx(d)];
        let next_loc = cur + direction.times(count);
        let mut compressed_loc = V(r_inv[&cur.0], c_inv[&cur.1]);
        let next_compressed_loc = V(r_inv[&next_loc.0], c_inv[&next_loc.1]);
        while compressed_loc != next_compressed_loc {
            seen.insert(compressed_loc);
            compressed_loc += direction;
            seen.insert(compressed_loc);
        }
        cur = next_loc;
    }

    let mut to_visit = vec![V(r_inv[&0] + 1, c_inv[&0] + 1)];
    while let Some(pos) = to_visit.pop() {
        if !seen.insert(pos) {
            continue;
        }
        for delta in DIRECTIONS {
            to_visit.push(pos + delta);
        }
    }

    let mut area = 0;
    for v in seen {
        let (cr, cc) = (v.0 as usize, v.1 as usize);
        let y = (rs[cr+1] - rs[cr]) as u64;
        let x = (cs[cc+1] - cs[cc]) as u64;
        area += y * x;
    }

    area.to_string()
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
        let input = include_str!("../../inputs/2023/day18-sample.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day18.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day18-sample.txt");
        assert_eq!("952408144115", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day18.txt");
        assert_eq!("92291468914147", part2(input));
    }
}
