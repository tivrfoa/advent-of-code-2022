use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
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
            for (r, c) in to_visit.drain() {
                if flashed[r][c] {
                    continue;
                }
                flashed[r][c] = true;
                grid[r][c] = 0;
                ans += 1;

                // left
                if c > 0 && !flashed[r][c - 1] {
                    grid[r][c - 1] += 1;
                    if grid[r][c - 1] > 9 {
                        new_visit.insert((r, c - 1));
                    }
                }

                // right
                if c < cols - 1 && !flashed[r][c + 1] {
                    grid[r][c + 1] += 1;
                    if grid[r][c + 1] > 9 {
                        new_visit.insert((r, c + 1));
                    }
                }

                // top
                if r > 0 && !flashed[r - 1][c] {
                    grid[r - 1][c] += 1;
                    if grid[r - 1][c] > 9 {
                        new_visit.insert((r - 1, c));
                    }
                }

                // bottom
                if r < rows - 1 && !flashed[r + 1][c] {
                    grid[r + 1][c] += 1;
                    if grid[r + 1][c] > 9 {
                        new_visit.insert((r + 1, c));
                    }
                }

                // top left
                if r > 0 && c > 0 && !flashed[r - 1][c - 1] {
                    grid[r - 1][c - 1] += 1;
                    if grid[r - 1][c - 1] > 9 {
                        new_visit.insert((r - 1, c - 1));
                    }
                }

                // top right
                if r > 0 && c < cols - 1 && !flashed[r - 1][c + 1] {
                    grid[r - 1][c + 1] += 1;
                    if grid[r - 1][c + 1] > 9 {
                        new_visit.insert((r - 1, c + 1));
                    }
                }

                // bottom left
                if r < rows - 1 && c > 0 && !flashed[r + 1][c - 1] {
                    grid[r + 1][c - 1] += 1;
                    if grid[r + 1][c - 1] > 9 {
                        new_visit.insert((r + 1, c - 1));
                    }
                }

                // bottom right
                if r < rows - 1 && c < cols - 1 && !flashed[r + 1][c + 1] {
                    grid[r + 1][c + 1] += 1;
                    if grid[r + 1][c + 1] > 9 {
                        new_visit.insert((r + 1, c + 1));
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
            for (r, c) in to_visit.drain() {
                if flashed[r][c] {
                    continue;
                }
                flashed[r][c] = true;
                grid[r][c] = 0;

                // left
                if c > 0 && !flashed[r][c - 1] {
                    grid[r][c - 1] += 1;
                    if grid[r][c - 1] > 9 {
                        new_visit.insert((r, c - 1));
                    }
                }

                // right
                if c < cols - 1 && !flashed[r][c + 1] {
                    grid[r][c + 1] += 1;
                    if grid[r][c + 1] > 9 {
                        new_visit.insert((r, c + 1));
                    }
                }

                // top
                if r > 0 && !flashed[r - 1][c] {
                    grid[r - 1][c] += 1;
                    if grid[r - 1][c] > 9 {
                        new_visit.insert((r - 1, c));
                    }
                }

                // bottom
                if r < rows - 1 && !flashed[r + 1][c] {
                    grid[r + 1][c] += 1;
                    if grid[r + 1][c] > 9 {
                        new_visit.insert((r + 1, c));
                    }
                }

                // top left
                if r > 0 && c > 0 && !flashed[r - 1][c - 1] {
                    grid[r - 1][c - 1] += 1;
                    if grid[r - 1][c - 1] > 9 {
                        new_visit.insert((r - 1, c - 1));
                    }
                }

                // top right
                if r > 0 && c < cols - 1 && !flashed[r - 1][c + 1] {
                    grid[r - 1][c + 1] += 1;
                    if grid[r - 1][c + 1] > 9 {
                        new_visit.insert((r - 1, c + 1));
                    }
                }

                // bottom left
                if r < rows - 1 && c > 0 && !flashed[r + 1][c - 1] {
                    grid[r + 1][c - 1] += 1;
                    if grid[r + 1][c - 1] > 9 {
                        new_visit.insert((r + 1, c - 1));
                    }
                }

                // bottom right
                if r < rows - 1 && c < cols - 1 && !flashed[r + 1][c + 1] {
                    grid[r + 1][c + 1] += 1;
                    if grid[r + 1][c + 1] > 9 {
                        new_visit.insert((r + 1, c + 1));
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
