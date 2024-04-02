use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

#[derive(Clone, PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}
impl Pos {
    fn new(row: usize, col: usize) -> Pos {
        Pos { row, col }
    }
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
            } else {
                dfs(grid, visited, final_pos, r, c - 1).map(|steps| steps + 1)
            }
        }
        '>' => {
            if c + 1 == cols || grid[r][c + 1] == '#' {
                None
            } else {
                dfs(grid, visited, final_pos, r, c + 1).map(|steps| steps + 1)
            }
        }
        '^' => {
            if r == 0 || grid[r - 1][c] == '#' {
                None
            } else {
                dfs(grid, visited, final_pos, r - 1, c).map(|steps| steps + 1) 
            }
        }
        'v' => {
            if r + 1 == rows || grid[r + 1][c] == '#' {
                None
            } else {
                dfs(grid, visited, final_pos, r + 1, c).map(|steps| steps + 1)
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
    let mut grid = input.to_char_grid();
    let rows = grid.len();
    let cols = grid[0].len();
    const D: [char; 4] = ['>', '<', 'v', '^'];
    for r in 0..rows {
        for c in 0..cols {
            if D.contains(&grid[r][c]) {
                grid[r][c] = '.';
            }
        }
    }

    dfs2(&grid, &Pos { row: rows - 1, col: cols - 2}, 0, 1).to_string()
}

#[derive(PartialEq)]
struct State {
    pos: Pos,
    steps: u32,
    visited: Vec<Vec<bool>>,
}
impl State {
    fn move_to(&self, r: usize, c: usize) -> State {
        let mut visited = self.visited.clone();
        visited[r][c] = true;
        State {
            pos: Pos {
                row: r,
                col: c,
            },
            steps: self.steps + 1,
            visited,
        }
    }
}

fn dfs2(grid: &[Vec<char>], final_pos: &Pos, r: usize, c: usize) -> u32 {
    let mut max_steps = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    let mut previous: Vec<Vec<Vec<(Pos, u32)>>> = vec![vec![vec![]; cols]; rows];
    let mut visited = vec![vec![false; cols]; rows];
    visited[r][c] = true;
    let mut to_visit: Vec<State> = vec![
        State { pos: Pos { row: r, col: c }, steps: 0, visited }
    ];
    while let Some(state) = to_visit.pop() {
        if state.pos == *final_pos {
            max_steps = max_steps.max(state.steps);
            continue;
        }
        'd: for (cond, (r, c)) in get_dirs(state.pos.row, state.pos.col, rows, cols) {
            if cond && grid[r][c] != '#' && !state.visited[r][c] {
                for p in previous[r][c].iter_mut() {
                    if p.0 == state.pos {
                        if state.steps + 1 > p.1 {
                            p.1 = state.steps + 1;
                            to_visit.push(state.move_to(r, c));
                        }
                        continue 'd;
                    }
                }
                previous[r][c].push((Pos::new(state.pos.row, state.pos.col), state.steps + 1));
                to_visit.push(state.move_to(r, c));
            }
        }
    }
    max_steps
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
        assert_eq!("154", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day23.txt");
        assert_eq!("", part2(input));
    }
}
