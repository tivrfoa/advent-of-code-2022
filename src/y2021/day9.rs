use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

fn part1(input: String) -> String {
    let mut ans = 0;
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut grid = vec![vec![i32::MAX; cols + 2]; rows + 2];

    for (r, line) in input.lines().enumerate() {
        for (c, n) in line.chars().enumerate() {
            grid[r+1][c+1] = n.to_digit(10).unwrap() as i32;
        }
    }

    for r in 1..=rows {
        for c in 1..=cols {
            // left
            if grid[r][c-1] <= grid[r][c] { continue; }

            // right
            if grid[r][c+1] <= grid[r][c] { continue; }

            // up
            if grid[r - 1][c] <= grid[r][c] { continue; }

            // down
            if grid[r + 1][c] <= grid[r][c] { continue; }

            ans += grid[r][c] + 1;
        }
    }

    ans.to_string()
}

fn part2(input: String) -> String {
    "".into()
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
        assert_eq!("", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day9.txt");
        assert_eq!("", part2(input));
    }
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
