use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

fn is_open(c: char) -> bool {
    match c {
        '{' | '[' | '<' | '(' => true,
        _ => false,
    }
}

fn part1(input: String) -> String {
    let cost_map = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    let open_close_map = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    let mut ans = 0;
    'l: for line in input.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            if is_open(c) {
                stack.push(c);
            } else {
                if stack.is_empty() {
                    ans += cost_map.get(&c).unwrap();
                    continue 'l;
                } else {
                    let last = stack.pop().unwrap();
                    if *open_close_map.get(&last).unwrap() != c {
                        ans += cost_map.get(&c).unwrap();
                        continue 'l;
                    }
                }
            }
        }
    }

    ans.to_string()
}

fn part2(input: String) -> String {
    let open_close_map = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    let mut incomplete_lines = vec![];
    'l: for line in input.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            if is_open(c) {
                stack.push(c);
            } else {
                if !stack.is_empty() {
                    let last = stack.pop().unwrap();
                    if *open_close_map.get(&last).unwrap() != c {
                        continue 'l;
                    }
                }
            }
        }
        incomplete_lines.push(line);
    }

    let mut map = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    let mut scores: Vec<u64> = vec![];
    for line in incomplete_lines {
        let mut stack = vec![];
        for c in line.chars() {
            if is_open(c) {
                stack.push(c);
            } else {
                stack.pop();
            }
        }
        let mut score = 0;
        while let Some(c) = stack.pop() {
            score = score * 5 + map.get(&c).unwrap();
        }
        scores.push(score);
    }
    scores.sort();

    (scores[scores.len() / 2]).to_string()
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
        let input = util::read_file("inputs/2021/day10-sample.txt");
        assert_eq!("26397", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2021/day10.txt");
        assert_eq!("392043", part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2021/day10-sample.txt");
        assert_eq!("288957", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day10.txt");
        assert_eq!("1605968119", part2(input));
    }
}
