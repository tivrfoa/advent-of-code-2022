use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

fn part1(input: String) -> String {
    let mut ans = 0;
    let mut grid = parse_grid_extra(&input, u32::MAX);
    let rows = grid.len() - 2;
    let cols = grid[0].len() - 2;

    for r in 1..=rows {
        for c in 1..=cols {
            // left
            if grid[r][c - 1] <= grid[r][c] {
                continue;
            }

            // right
            if grid[r][c + 1] <= grid[r][c] {
                continue;
            }

            // up
            if grid[r - 1][c] <= grid[r][c] {
                continue;
            }

            // down
            if grid[r + 1][c] <= grid[r][c] {
                continue;
            }

            ans += grid[r][c] + 1;
        }
    }

    ans.to_string()
}

fn visit_borders(visited: &mut Vec<Vec<bool>>) {
    let rows = visited.len();
    let cols = visited[0].len();
    // top, bottom
    for c in 0..cols {
        visited[0][c] = true;
        visited[rows - 1][c] = true;
    }
    // left, right
    for r in 0..rows {
        visited[r][0] = true;
        visited[r][cols - 1] = true;
    }
}

fn visit(grid: &[Vec<u32>], visited: &mut Vec<Vec<bool>>, row: usize, col: usize) -> u32 {
    if visited[row][col] || grid[row][col] == 9 {
        return 0;
    }

    visited[row][col] = true;
    let mut sum = 1;

    // left
    sum += visit(grid, visited, row, col - 1);
    // right
    sum += visit(grid, visited, row, col + 1);
    // top
    sum += visit(grid, visited, row - 1, col);
    // bottom
    sum += visit(grid, visited, row + 1, col);

    sum
}

fn part2(input: String) -> String {
    let mut grid = parse_grid_extra(&input, u32::MAX);
    let rows = grid.len() - 2;
    let cols = grid[0].len() - 2;
    let mut visited = vec![vec![false; cols + 2]; rows + 2];
    visit_borders(&mut visited);

    let mut basis = vec![];
    for r in 1..=rows {
        for c in 1..=cols {
            let size = visit(&grid, &mut visited, r, c);
            if size > 0 {
                basis.push(size);
            }
        }
    }
    basis.sort_by(|a, b| b.cmp(a));

    (basis[0] * basis[1] * basis[2]).to_string()
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
fn parse_grid_extra<T: std::str::FromStr + From<u32> + std::clone::Clone>(
    input: &str,
    default: T,
) -> Vec<Vec<T>>
where
    <T as std::str::FromStr>::Err: Debug,
{
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut grid = vec![vec![default; cols + 2]; rows + 2];

    for (r, line) in input.lines().enumerate() {
        for (c, n) in line.chars().enumerate() {
            grid[r + 1][c + 1] = T::from(n.to_digit(10).unwrap());
        }
    }

    grid
}

#[allow(dead_code)]
fn parse_grid_i32(input: &str) -> Vec<Vec<i32>> {
    parse_grid_i32_extra(input, 0)
}

#[allow(dead_code)]
fn parse_grid_i32_extra(input: &str, default: i32) -> Vec<Vec<i32>> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut grid = vec![vec![default; cols + 2]; rows + 2];

    for (r, line) in input.lines().enumerate() {
        for (c, n) in line.chars().enumerate() {
            grid[r + 1][c + 1] = n.to_digit(10).unwrap() as i32;
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2021/day9-sample.txt");
        assert_eq!("15", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2021/day9.txt");
        assert_eq!("468", part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2021/day9-sample.txt");
        assert_eq!("1134", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day9.txt");
        assert_eq!("1280496", part2(input));
    }
}
