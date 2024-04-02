use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}
impl Pos {
    fn new(row: usize, col: usize) -> Pos {
        Pos { row, col }
    }
    fn from(p: (usize, usize)) -> Pos {
        Pos { row: p.0, col: p.1 }
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

// Translation from hyper-neutrino
// https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day23p2.py
pub fn part2(input: &str) -> String {
    let mut grid = input.to_char_grid();
    let rows = grid.len();
    let cols = grid[0].len();
    let start = Pos::from((0usize, grid[0].iter().position(|&c| c == '.').unwrap()));
    let end = Pos::from((rows - 1, grid[rows - 1].iter().position(|&c| c == '.').unwrap()));
    let mut points: Vec<Pos> = vec![start, end];

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == '#' {
                continue;
            }

            let mut neighbors = 0;
            for (cond, (r, c)) in get_dirs(r, c, rows, cols) {
                if cond && grid[r][c] != '#' {
                    neighbors += 1;
                }
            }
            if neighbors >= 3 {
                points.push(Pos::new(r, c));
            }
        }
    }

    let mut graph: HashMap<Pos, HashMap<Pos, i32>> = HashMap::new();
    for pos in &points {
        graph.insert(*pos, HashMap::new());
        let mut stack = vec![(0, pos.row, pos.col)];
        let mut seen = HashSet::new();
        seen.insert((pos.row, pos.col));

        while let Some((n, r, c)) = stack.pop() {
            let p = Pos::new(r, c);
            if n != 0 && points.contains(&p) {
                let tmp = graph.get_mut(pos).unwrap().insert(p, n);
                continue;
            }

            for (cond, (r, c)) in get_dirs(r, c, rows, cols) {
                if cond && grid[r][c] != '#' && !seen.contains(&(r, c)) {
                    stack.push((n + 1, r, c));
                    seen.insert((r, c));
                }
            }
        }
    }

    fn dfs(end: Pos, graph: &HashMap<Pos, HashMap<Pos, i32>>, seen: &mut HashSet<(usize, usize)>, pt: Pos) -> i32 {
        if pt == end {
            return 0;
        }

        let mut m = i32::MIN;
        seen.insert((pt.row, pt.col));
        for (k, v) in &graph[&pt] {
            if !seen.contains(&(k.row, k.col)) {
                m = m.max(dfs(end, graph, seen, *k) + v);
            }
        }
        seen.remove(&(pt.row, pt.col));
        m
    };

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    dfs(end, &graph, &mut seen, start).to_string()
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
        assert_eq!("6710", part2(input));
    }
}
