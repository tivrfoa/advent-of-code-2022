use crate::util;

use util::*;

use std::cell::Cell;
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;

fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut g = vec![];
    for line in input.lines() {
        let mut l = vec![];
        for c in line.chars() {
            let n: u8 = c as u8 - b'0';
            l.push(n);
        }
        g.push(l);
    }
    g
}

const L: u8 = 0;
const R: u8 = 1;
const N: u8 = 2;
const S: u8 = 3;

fn dir_to_char(dir: u8) -> char {
    match dir {
        L => '<',
        R => '>',
        N => '^',
        S => 'v',
        _ => panic!("{dir}"),
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: u32,
    r: usize,
    c: usize,
    dir: u8,
    blocks: u8,
    path: Vec<(usize, usize, char)>,
}

impl State {
    fn new(cost: u32, r: usize, c: usize, dir: u8, blocks: u8, path: Vec<(usize, usize, char)>) -> Self { Self { cost, r, c, dir, blocks, path } }

    fn go_to(&self, grid: &[Vec<u8>], dr: i32, dc: i32, dir: u8) -> Option<Self> {
        if (self.dir == N && dir == S)
            || (self.dir == S && dir == N)
            || (self.dir == L && dir == R)
            || (self.dir == R && dir == L)
        {
            return None;
        }
        let rows = grid.len();
        let cols = grid.len();
        if (dr < 0 && self.r == 0)
            || (dr > 0 && self.r + 1 == rows)
            || (dc < 0 && self.c == 0)
            || (dc > 0 && self.c + 1 == cols)
        {
            return None;
        }

        let r = (self.r as i32 + dr) as usize;
        let c = (self.c as i32 + dc) as usize;
        let mut path = self.path.clone();
        path.push((r, c, dir_to_char(dir)));

        Some(Self {
            r,
            c,
            cost: self.cost + grid[r][c] as u32,
            dir,
            blocks: if self.dir == dir { self.blocks + 1 } else { 1 },
            path,
        })
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(input: &str) -> String {
    const MIN: u8 = 4;
    const MAX: u8 = 10;
    let grid = parse(input);
    let (rows, cols) = (grid.len(), grid[0].len());
    let dest = (rows - 1, cols - 1);
    let mut visited: HashSet<(usize, usize, u8, u8)> = HashSet::new();
    let mut pq = BinaryHeap::new();
    pq.push(Reverse(State::new(0, 0, 0, R, 0, vec![(0, 0, '>')])));
    pq.push(Reverse(State::new(0, 0, 0, S, 0, vec![(0, 0, 'v')])));

    while let Some(state) = pq.pop() {
        let state = state.0;
        let (r, c, dir, cost, blocks) = (state.r, state.c, state.dir, state.cost, state.blocks);
        if (r, c) == dest && blocks >= MIN {
            print_path(state.path, rows, cols);
            return cost.to_string();
        }
        if !visited.insert((r, c, dir, blocks)) {
            continue;
        }

        const DIRS: [(i32, i32, u8); 4] = [(-1, 0, N), (1, 0, S), (0, 1, R), (0, -1, L)];

        for (dr, dc, new_dir) in DIRS {
            // turn
            if blocks >= MIN && dir != new_dir {
                if let Some(s) = state.go_to(&grid, dr, dc, new_dir) {
                    pq.push(Reverse(s));
                }
            }

            // go straight
            if blocks < MAX && dir == new_dir {
                if let Some(s) = state.go_to(&grid, dr, dc, new_dir) {
                    pq.push(Reverse(s));
                }
            }
        }
    }

    "0".into()
}

fn print_path(s_path: Vec<(usize, usize, char)>, rows: usize, cols: usize) {
    let mut path = vec![vec!['.'; cols]; rows];
    for (row, col, c) in s_path {
        path[row][col] = c;
    }
    for row in path {
        println!("{:?}", row);
    }
}
pub fn part1(input: &str) -> String {
    "todo".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2023/day17-sample.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day17.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day17-sample.txt");
        assert_eq!("94", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day17.txt");
        assert_eq!("1055", part2(input));
    }
}
