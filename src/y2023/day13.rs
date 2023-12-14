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
        let (_, mut v) = solve_vertical(&grid, usize::MAX);
        if v == 0 {
            (_, v) = solve_horizontal(&grid, usize::MAX);
        }
        total += v;
    }

    total.to_string()
}

pub fn solve_vertical(grid: &[Vec<char>], avoid: usize) -> (usize, usize) {
    let cols = grid[0].len();

    for m in 0..cols - 1 {
        if avoid != usize::MAX && m == avoid { continue; }
        let mut lo = m;
        let mut hi = m + 1;
        loop {
            if !are_cols_eq(&grid, lo, hi) {
                break;
            }

            if lo == 0 || hi + 1 == cols {
                return (m, m + 1);
            }
            lo -= 1;
            hi += 1;
        }
    }

    (0, 0)
}

pub fn solve_horizontal(grid: &[Vec<char>], avoid: usize) -> (usize, usize) {
    let rows = grid.len();

    // try horizontal
    for m in 0..rows - 1 {
        if avoid != usize::MAX && m == avoid { continue; }
        let mut lo = m;
        let mut hi = m + 1;
        loop {
            if !are_rows_eq(&grid, lo, hi) {
                break;
            }

            if lo == 0 || hi + 1 == rows {
                return (m, (m + 1) * 100);
            }
            lo -= 1;
            hi += 1;
        }
    }

    (0, 0)
}

pub fn part2(input: &str) -> String {
    let mut total: usize = 0;
    'l:
    for ingrid in input.split("\n\n") {
        let mut grid = ingrid.to_char_grid();
        let rows = grid.len();
        let cols = grid[0].len();
        let (mut oph, _) = (usize::MAX, 0);
        let (mut opv, original_vertical) = solve_vertical(&grid, usize::MAX);
        if original_vertical == 0 {
            opv = usize::MAX;
            (oph, _) = solve_horizontal(&grid, usize::MAX);
        }

        for y in 0..rows {
            for x in 0..cols {
                let prev = grid[y][x];
                grid[y][x] = if prev == '.' { '#' } else { '.' };
                let (_, v) = solve_horizontal(&grid, oph);
                if v > 0 {
                    total += v;
                    continue 'l;
                }
                let (_, v) = solve_vertical(&grid, opv);
                if v > 0 {
                    total += v;
                    continue 'l;
                }
                grid[y][x] = prev;
            }
        }
        println!("did not find mirror for grid");
    }

    total.to_string()
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
        assert_eq!("400", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day13.txt");
        assert_eq!("24847", part2(input));
    }
}
