use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn are_cols_eq(grid: &[Vec<char>], l: usize, r: usize) -> bool {
    for i in 0..grid.len() {
        if grid[i][l] != grid[i][r] {
            return false;
        }
    }
    true
}

fn are_rows_eq(grid: &[Vec<char>], t: usize, b: usize) -> bool {
    for i in 0..grid[0].len() {
        if grid[t][i] != grid[b][i] {
            return false;
        }
    }
    true
}

pub fn part1(input: &str) -> String {
    let mut total: usize = 0;

    for ingrid in input.split("\n\n") {
        let grid = ingrid.to_char_grid();
        dbg_grid(&grid);

        // try vertical
        let mut l = 0;
        let mut r = grid[0].len() - 1;
        if are_cols_eq(&grid, l, r) {
            println!("left right equal");
        } else if are_cols_eq(&grid, l + 1, r) {
            l += 1;
        } else {
            r -= 1;
        }
        while l < r {
            if !are_cols_eq(&grid, l, r) {
                break;
            }
            l += 1;
            r -= 1;
        }
        if l > r {
            println!("Found vertical");
            total += l;
            continue;
        }

        // try horizontal
        let mut t = 0;
        let mut b = grid.len() - 1;
        if are_rows_eq(&grid, t, b) {
            println!("top bottom equal");
        } else if are_rows_eq(&grid, t + 1, b) {
            t += 1;
        } else {
            b -= 1;
        }
        while t < b {
            if !are_rows_eq(&grid, t, b) {
                break;
            }
            t += 1;
            b -= 1;
        }
        if t > b {
            total += t * 100;
        }
    }

    total.to_string()
}

pub fn part2(input: &str) -> String {
    "".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2023/day13-sample.txt");
        assert_eq!("405", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day13.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day13-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day13.txt");
        assert_eq!("", part2(input));
    }
}
