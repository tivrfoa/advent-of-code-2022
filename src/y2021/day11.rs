use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

fn part1(input: String) -> String {
    let rows = 10;
    let cols = 10;
    let mut grid: Vec<Vec<u32>> = vec![];
    for line in input.lines() {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut ans = 0;
    for _ in 0..100 {
        let mut to_visit = HashSet::new();
        let mut flashed = vec![vec![false; cols]; rows];

        // increase all by one
        for r in 0..10 {
            for c in 0..10 {
                grid[r][c] += 1;
                if grid[r][c] > 9 {
                    to_visit.insert((r, c));
                }
            }
        }

        let mut new_visit = HashSet::new();

        while !to_visit.is_empty() {
            for (r, c) in to_visit {
                if flashed[r][c] {
                    continue;
                }
                flashed[r][c] = true;
                grid[r][c] = 0;
                ans += 1;

                for (cond, (r, c)) in get_dirs(r, c, rows, cols) {
                    if cond && !flashed[r][c] {
                        grid[r][c] += 1;
                        if grid[r][c] > 9 {
                            new_visit.insert((r, c));
                        }
                    }
                }
            }
            to_visit = new_visit;
            new_visit = HashSet::new();
        }
    }

    ans.to_string()
}

fn part2(input: String) -> String {
    let rows = 10;
    let cols = 10;
    let mut grid: Vec<Vec<u32>> = vec![];
    for line in input.lines() {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    for step in 1..=600 {
        let mut to_visit = HashSet::new();
        let mut flashed = vec![vec![false; cols]; rows];

        // increase all by one
        for r in 0..10 {
            for c in 0..10 {
                grid[r][c] += 1;
                if grid[r][c] > 9 {
                    to_visit.insert((r, c));
                }
            }
        }

        let mut new_visit = HashSet::new();
        while !to_visit.is_empty() {
            for (r, c) in to_visit {
                if flashed[r][c] {
                    continue;
                }
                flashed[r][c] = true;
                grid[r][c] = 0;

                for (cond, (r, c)) in get_dirs(r, c, rows, cols) {
                    if cond && !flashed[r][c] {
                        grid[r][c] += 1;
                        if grid[r][c] > 9 {
                            new_visit.insert((r, c));
                        }
                    }
                }
            }
            to_visit = new_visit;
            new_visit = HashSet::new();
        }

        if flashed.iter().filter(|row| row.contains(&false)).count() == 0 {
            return step.to_string();
        }
    }

    panic!("Did not find an answer ...");
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
fn get_dirs(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 8] {
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
    fn part1_sample() {
        let input = util::read_file("inputs/2021/day11-sample.txt");
        assert_eq!("1656", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2021/day11.txt");
        assert_eq!("1713", part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2021/day11-sample.txt");
        assert_eq!("195", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day11.txt");
        assert_eq!("502", part2(input));
    }
}
