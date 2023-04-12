use crate::util;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

fn part1(input: String) -> String {
    let mut grid: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let len = grid.len();

    let mut dp: Vec<Vec<u32>> = vec![vec![0; len + 1]; len + 1];
    for i in 0..len {
        dp[i][len] = u32::MAX;
        dp[len][i] = u32::MAX;
    }
    dp[len][len - 1] = 0;
    dp[len - 1][len] = 0;

    // do bottom up approach
    for r in (0..len).rev() {
        for c in (0..len).rev() {
            dp[r][c] = dp[r + 1][c].min(dp[r][c + 1]) + grid[r][c];
        }
    }

    dp[0][0] -= grid[0][0];

    dp[0][0].to_string()
}

fn increase_grid(grid: &mut Vec<Vec<u32>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    // replicate 4 for times to the right, then do the same for rows
    for i in 1..=4 {
        for r in 0..rows {
            for c in 0..cols {
                let v = grid[r][c] + i;
                grid[r].push(if v > 9 { v % 9 } else { v });
            }
        }
    }

    let cols = cols * 5;

    for i in 1..=4 {
        for r in 0..rows {
            let mut row = vec![];
            for c in 0..cols {
                let v = grid[r][c] + i;
                row.push(if v > 9 { v % 9 } else { v });
            }
            grid.push(row);
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    visited: Vec<Vec<bool>>,
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(input: String) -> String {
    let mut grid: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect(),
        );
    }

    increase_grid(&mut grid);

    let len = grid.len();
    let rows = len;
    let cols = len;

    let mut memo: Vec<Vec<u32>> = vec![vec![u32::MAX; len]; len];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; len]; len];
    visited[0][0] = true;

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let start = State {
        visited,
        cost: 0,
        position: (0, 0),
    };
    heap.push(start);
    let mut min = u32::MAX;

    while let Some(state) = heap.pop() {
        let (r, c) = state.position;
        if r == rows - 1 && c == cols - 1 {
            if memo[r][c] < min {
                min = memo[r][c];
            }
            continue;
        }

        for (cond, (row, col)) in get_dirs(r, c, rows, cols) {
            if cond && !state.visited[row][col] && state.cost + grid[row][col] < memo[row][col]  {
                memo[row][col] = state.cost + grid[row][col];
                let mut new_state = state.clone();
                new_state.visited[row][col] = true;
                new_state.cost = memo[row][col];
                new_state.position = (row, col);
                heap.push(new_state);
            }
        }
    }

    dbg!(memo[len - 1][len - 1]);

    min.to_string()
}

#[allow(dead_code)]
fn dbg_grid<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

#[allow(dead_code)]
fn in_to_nums<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: Debug,
{
    input.split(',').map(|n| n.parse::<T>().unwrap()).collect()
}

#[allow(dead_code)]
fn split_str_to_nums<T: std::str::FromStr>(input: &str, separator: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: Debug,
{
    input
        .split(separator)
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}

#[allow(dead_code)]
fn vec_max<T: std::str::FromStr + std::cmp::Ord + Copy>(vec: &[T]) -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    *vec.iter().max().unwrap()
}

#[allow(dead_code)]
fn vec_min<T: std::str::FromStr + std::cmp::Ord + Copy>(vec: &[T]) -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    *vec.iter().min().unwrap()
}

#[allow(dead_code)]
fn str_to_char_tuple(s: &str) -> (char, char) {
    (
        s[0..1].chars().next().unwrap(),
        s[1..2].chars().next().unwrap(),
    )
}

#[allow(dead_code)]
trait MapAddOrInsert<K, V> {
    fn add_or_insert(&mut self, k: K, v: V);
}

#[allow(dead_code)]
impl<K: Eq + Hash, V: std::ops::AddAssign + Copy> MapAddOrInsert<K, V> for HashMap<K, V> {
    fn add_or_insert(&mut self, k: K, v: V) {
        self.entry(k).and_modify(|qt| *qt += v).or_insert(v);
    }
}

#[allow(dead_code)]
fn get_dirs(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 4] {
    [
        // left
        (c > 0, (r, if c > 0 { c - 1 } else { 0 })),
        // right
        (c < cols - 1, (r, c + 1)),
        // top
        (r > 0, (if r > 0 { r - 1 } else { 0 }, c)),
        // bottom
        (r < rows - 1, (r + 1, c)),
    ]
}

#[allow(dead_code)]
fn get_dirs_with_diagonals(
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
) -> [(bool, (usize, usize)); 8] {
    [
        // left
        (c > 0, (r, if c > 0 { c - 1 } else { 0 })),
        // right
        (c < cols - 1, (r, c + 1)),
        // top
        (r > 0, (if r > 0 { r - 1 } else { 0 }, c)),
        // bottom
        (r < rows - 1, (r + 1, c)),
        // top left
        (
            r > 0 && c > 0,
            (if r > 0 { r - 1 } else { 0 }, if c > 0 { c - 1 } else { 0 }),
        ),
        // top right
        (
            r > 0 && c < cols - 1,
            (if r > 0 { r - 1 } else { 0 }, c + 1),
        ),
        // bottom left
        (
            r < rows - 1 && c > 0,
            (r + 1, if c > 0 { c - 1 } else { 0 }),
        ),
        // bottom right
        (r < rows - 1 && c < cols - 1, (r + 1, c + 1)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = util::read_file("inputs/2021/day15-sample.txt");
        assert_eq!("40", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day15.txt");
        assert_eq!("373", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day15-sample.txt");
        assert_eq!("315", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day15.txt");
        assert_eq!("2868", part2(input));
    }
}
