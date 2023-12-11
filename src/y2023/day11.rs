use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

pub fn part1(input: &str) -> String {
    let grid = input.to_char_grid();
    // expand grid
    let grid = {
        let mut new_grid = vec![];
        for r in 0..grid.len() {
            let mut is_empty = true;
            for c in 0..grid[0].len() {
                if grid[r][c] != '.' {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                new_grid.push(grid[r].clone());
            }
            new_grid.push(grid[r].clone());
        }
        let rows = new_grid.len();
        let mut c = 0;
        while c < new_grid[0].len() {
            let mut is_empty = true;
            for r in 0..rows {
                if new_grid[r][c] != '.' {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                for r in 0..rows {
                    new_grid[r].insert(c, '.');
                }
                c += 2;
            } else {
                c += 1;
            }
        }

        new_grid
    };
    let rows = grid.len();
    let cols = grid[0].len();
    let galaxies = {
        let mut g = vec![];
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '#' {
                    g.push((r, c));
                }
            }
        }
        g
    };

    let glen = galaxies.len();
    let mut g_pairs: Vec<Vec<usize>> = Vec::with_capacity(glen);
    for i in 0..glen - 1 {
        let mut pairs = Vec::with_capacity(glen - i);
        for j in i+1..galaxies.len() {
            let d = galaxies[i].0.abs_diff(galaxies[j].0) +
                    galaxies[i].1.abs_diff(galaxies[j].1);
            pairs.push(d);
        }
        g_pairs.push(pairs);
    }

    let mut sum = 0;
    for p in g_pairs {
        sum += p.iter().sum::<usize>();
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
        let input = include_str!("../../inputs/2023/day11-sample.txt");
        assert_eq!("374", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day11.txt");
        assert_eq!("9543156", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day11-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day11.txt");
        assert_eq!("", part2(input));
    }
}
