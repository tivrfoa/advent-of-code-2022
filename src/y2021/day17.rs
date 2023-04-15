use crate::util;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

/*

Solution copied from Neal Wu:
https://www.youtube.com/watch?v=iXt1hRdQrHE

*/

/// @return -1 or 0 or 1
fn sign(x: i64) -> i64 {
    if x > 0 {
        1
    } else if x < 0 {
        -1
    } else {
        0
    }
}

const STEPS: u16 = 1_000;

struct Target {
    x: (i64, i64),
    y: (i64, i64),
}

fn simulate(target: &Target, mut vx: i64, mut vy: i64) -> i64 {
    let (mut x, mut y) = (0, 0);
    let mut max_y = 0;

    for _ in 0..STEPS {
        x += vx;
        y += vy;
        max_y = max_y.max(y);

        if target.x.0 <= x && x <= target.x.1 && target.y.0 <= y && y <= target.y.1 {
            return max_y;
        }

        vx -= if vx == 0 { 0 } else { 1 };
        vy -= 1;
    }

    -1
}

fn part1(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> String {
    let target = Target {
        x: (min_x, max_x),
        y: (min_y, max_y),
    };

    let mut best = 0;

    for vx in -1200..=1200 {
        for vy in -1200..=1200 {
            let sim = simulate(&target, vx, vy);
            best = best.max(sim);
        }
    }

    best.to_string()
}

fn part2(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> String {
    let target = Target {
        x: (min_x, max_x),
        y: (min_y, max_y),
    };

    let mut count = 0;

    for vx in -1200..=1200 {
        for vy in -1200..=1200 {
            let sim = simulate(&target, vx, vy);
            count += if sim >= 0 { 1 } else { 0 };
        }
    }

    count.to_string()
}

#[allow(dead_code)]
fn dbg_grid<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

#[allow(dead_code)]
fn in_to_nums<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: Debug,
{
    input.split(',').map(|n| n.parse::<T>().unwrap()).collect()
}

#[allow(dead_code)]
fn split_str_to_nums<T: std::str::FromStr>(input: &str, separator: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: Debug,
{
    input
        .split(separator)
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}

#[allow(dead_code)]
fn vec_max<T: std::str::FromStr + std::cmp::Ord + Copy>(vec: &[T]) -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    *vec.iter().max().unwrap()
}

#[allow(dead_code)]
fn vec_min<T: std::str::FromStr + std::cmp::Ord + Copy>(vec: &[T]) -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    *vec.iter().min().unwrap()
}

#[allow(dead_code)]
fn str_to_char_tuple(s: &str) -> (char, char) {
    (
        s[0..1].chars().next().unwrap(),
        s[1..2].chars().next().unwrap(),
    )
}

#[allow(dead_code)]
trait MapAddOrInsert<K, V> {
    fn add_or_insert(&mut self, k: K, v: V);
}

#[allow(dead_code)]
impl<K: Eq + Hash, V: std::ops::AddAssign + Copy> MapAddOrInsert<K, V> for HashMap<K, V> {
    fn add_or_insert(&mut self, k: K, v: V) {
        self.entry(k).and_modify(|qt| *qt += v).or_insert(v);
    }
}

#[allow(dead_code)]
fn get_dirs(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 4] {
    [
        // left
        (c > 0, (r, if c > 0 { c - 1 } else { 0 })),
        // right
        (c < cols - 1, (r, c + 1)),
        // top
        (r > 0, (if r > 0 { r - 1 } else { 0 }, c)),
        // bottom
        (r < rows - 1, (r + 1, c)),
    ]
}

#[allow(dead_code)]
fn get_dirs_with_diagonals(
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
) -> [(bool, (usize, usize)); 8] {
    [
        // left
        (c > 0, (r, if c > 0 { c - 1 } else { 0 })),
        // right
        (c < cols - 1, (r, c + 1)),
        // top
        (r > 0, (if r > 0 { r - 1 } else { 0 }, c)),
        // bottom
        (r < rows - 1, (r + 1, c)),
        // top left
        (
            r > 0 && c > 0,
            (if r > 0 { r - 1 } else { 0 }, if c > 0 { c - 1 } else { 0 }),
        ),
        // top right
        (
            r > 0 && c < cols - 1,
            (if r > 0 { r - 1 } else { 0 }, c + 1),
        ),
        // bottom left
        (
            r < rows - 1 && c > 0,
            (r + 1, if c > 0 { c - 1 } else { 0 }),
        ),
        // bottom right
        (r < rows - 1 && c < cols - 1, (r + 1, c + 1)),
    ]
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
        assert_eq!("45", part1(20, 30, -10, -5));
    }

    #[test]
    fn p1() {
        assert_eq!("9870", part1(119, 176, -141, -84));
    }

    #[test]
    fn p2s() {
        assert_eq!("112", part2(20, 30, -10, -5));
    }

    #[test]
    fn p2() {
        assert_eq!("5523", part2(119, 176, -141, -84));
    }
}
