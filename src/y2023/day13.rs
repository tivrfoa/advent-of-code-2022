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
        let (_, mut v) = solve_vertical(&grid);
        if v == 0 {
            (_, v) = solve_horizontal(&grid);
        }
        total += v;
    }

    total.to_string()
}

pub fn solve_vertical(grid: &[Vec<char>]) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();

    for m in 0..cols - 1 {
        let mut lo = m;
        let mut hi = m + 1;
        loop {
            if !are_cols_eq(&grid, lo, hi) {
                break;
            }

            if lo == 0 || hi + 1 == cols {
                println!("Found vertical: {m}");
                return (m, m + 1);
            }
            lo -= 1;
            hi += 1;
        }
    }

    (0, 0)
}

pub fn solve_horizontal(grid: &[Vec<char>]) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();

    // try horizontal
    for m in 0..rows - 1 {
        let mut lo = m;
        let mut hi = m + 1;
        loop {
            if !are_rows_eq(&grid, lo, hi) {
                break;
            }

            if lo == 0 || hi + 1 == rows {
                println!("Found horizontal: {m}");
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
        dbg_grid(&grid);
        let rows = grid.len();
        let cols = grid[0].len();
        dbg!(rows, cols);
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let prev = grid[y][x];
                if prev == '.' {
                    grid[y][x] = '#';
                } else {
                    grid[y][x] = '.';
                }
                let mut qt = 0;
                let (p, v) = solve_vertical(&grid);
                if v > 0 && (x == p ||
                        (x < p && p - x + 1 <= cols - p - 1) ||
                        (x > p && p + 1 >= x - p)) {
                    println!("vertical: y = {y}, x = {x}, p = {p}");
                    qt += v;
                }
                dbg!(qt);
                let (p, v) = solve_horizontal(&grid);
                if v > 0 && (y == p ||
                        (y < p && p - y + 1 <= rows - p - 1) ||
                        (y > p && p + 1 >= y - p)) {
                    println!("horizontal: y = {y}, x = {x}, p = {p}");
                    qt += v;
                }
                if qt > 0 {
                    total += qt;
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
        assert_eq!("", part2(input));
    }
}
