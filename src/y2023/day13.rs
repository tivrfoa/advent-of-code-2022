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
        let prev = total;
        let grid = ingrid.to_char_grid();
        let rows = grid.len();
        let cols = grid[0].len();
        dbg_grid(&grid);

        // try vertical
        'l:
        for m in 0..cols - 1 {
            let mut lo = m;
            let mut hi = m + 1;
            loop {
                if !are_cols_eq(&grid, lo, hi) {
                    break;
                }

                if lo == 0 || hi + 1 == cols {
                    println!("Found vertical: {m}");
                    total += m + 1;
                    break 'l;
                }
                lo -= 1;
                hi += 1;
            }
        }

        // try horizontal
        'l:
        for m in 0..rows - 1 {
            let mut lo = m;
            let mut hi = m + 1;
            loop {
                if !are_rows_eq(&grid, lo, hi) {
                    break;
                }

                if lo == 0 || hi + 1 == rows {
                    println!("Found horizontal: {m}");
                    total += (m + 1) * 100;
                    break 'l;
                }
                lo -= 1;
                hi += 1;
            }
        }

        if prev == total {
            println!("Did not find any reflection for this grid ...");
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
        assert_eq!("32035", part1(input));
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
