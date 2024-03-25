use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

pub fn part1(input: &str, steps: usize) -> String {
    let grid = input.to_char_grid();
    let rows = grid.len();
    let cols = grid[0].len();
    let s_pos = || -> (usize, usize) {
        for y in 0..rows {
            for x in 0..cols {
                if grid[y][x] == 'S' {
                    return (y, x);
                }
            }
        }
        panic!("no S");
    }();
    let mut positions = HashSet::from([s_pos]);
    for _ in 0..steps {
        let mut new_positions = HashSet::new();
        for (r, c) in positions {
            let dirs = util::get_dirs(r, c, rows, cols);
            for (cond, (y, x)) in dirs {
                if cond && grid[y][x] != '#' {
                    new_positions.insert((y, x));
                }
            }
        }
        positions = new_positions;
    }

    // for r in 0..rows {
    //     for c in 0..cols {
    //         if positions.contains(&(r, c)) {
    //             print!("O ");
    //         } else {
    //             print!("{} ", grid[r][c]);
    //         }
    //     }
    //     println!();
    // }

    positions.len().to_string()
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
        let input = include_str!("../../inputs/2023/day21-sample.txt");
        assert_eq!("16", part1(input, 6));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day21.txt");
        assert_eq!("3605", part1(input, 64));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day21-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day21.txt");
        assert_eq!("", part2(input));
    }
}
