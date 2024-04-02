use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

struct Pos {
    row: usize,
    col: usize,
}

fn dfs(grid: &[Vec<char>],
        visited: &mut Vec<Vec<bool>>, final_pos: &Pos, r: usize, c: usize) -> Option<u32> {
    if r == final_pos.row && c == final_pos.col {
        return Some(0);
    }
    if visited[r][c] {
        return None;
    }
    visited[r][c] = true;
    let rows = grid.len();
    let cols = grid[0].len();
    let steps = match grid[r][c] {
        '.' => {
            let mut max_steps = 0;
            for (cond, (r, c)) in get_dirs(r, c, rows, cols) {
                if cond && grid[r][c] != '#' {
                    if let Some(steps) = dfs(grid, visited, final_pos, r, c) {
                        max_steps = max_steps.max(steps + 1);
                    }
                }
            }
            if max_steps == 0 {
                None
            } else {
                Some(max_steps)
            }
        }
        '<' => {
            if c == 0 || grid[r][c - 1] == '#' {
                None
            } else if let Some(steps) = dfs(grid, visited, final_pos, r, c - 1) {
                Some(steps + 1)
            } else {
                None
            }
        }
        '>' => {
            if c + 1 == cols || grid[r][c + 1] == '#' {
                None
            } else if let Some(steps) = dfs(grid, visited, final_pos, r, c + 1) {
                Some(steps + 1)
            } else {
                None
            }
        }
        '^' => {
            if r == 0 || grid[r - 1][c] == '#' {
                None
            } else if let Some(steps) = dfs(grid, visited, final_pos, r - 1, c) {
                Some(steps + 1)
            } else {
                None
            }
        }
        'v' => {
            if r + 1 == rows || grid[r + 1][c] == '#' {
                None
            } else if let Some(steps) = dfs(grid, visited, final_pos, r + 1, c) {
                Some(steps + 1)
            } else {
                None
            }
        }
        _ => panic!("Invalid pos: {}", grid[r][c]),
    };

    visited[r][c] = false;
    steps
}

pub fn part1(input: &str) -> String {
    let grid = input.to_char_grid();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = vec![vec![false; cols]; rows];
    dfs(&grid, &mut visited, &Pos { row: rows - 1, col: cols - 2}, 0, 1).unwrap().to_string()
}

pub fn part2(input: &str) -> String {
    "".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2023/day23-sample.txt");
        assert_eq!("94", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day23.txt");
        assert_eq!("2130", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day23-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day23.txt");
        assert_eq!("", part2(input));
    }
}
