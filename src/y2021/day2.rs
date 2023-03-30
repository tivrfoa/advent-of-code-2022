use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;


enum Dir {
    F,
    D,
    U,
}

impl Dir {
    fn new(dir: &str) -> Self {
        match dir {
            "forward" => Dir::F,
            "down" => Dir::D,
            "up" => Dir::U,
            _ => panic!("Invalid dir {dir}"),
        }
    }
}

fn parse_dir_qt(input: &str) -> Vec<(Dir, u32)> {
    let mut ans = vec![];
    for l in input.lines() {
        let tmp = l.split_once(' ').unwrap();
        ans.push((Dir::new(tmp.0), tmp.1.parse::<u32>().unwrap()));
    }
    ans
}

fn part1(input: String) -> String {
    let mut h_pos = 0;
    let mut d_pos = 0;

    let mut data = parse_dir_qt(&input);
    for dir in data {
        match dir.0 {
            Dir::F => {
                h_pos += dir.1;
            },
            Dir::U => {
                d_pos -= dir.1;
            },
            Dir::D => {
                d_pos += dir.1;
            }
        }
    }

    (h_pos * d_pos).to_string()
}

fn part2(input: String) -> String {
    let mut h_pos = 0;
    let mut d_pos = 0;
    let mut aim = 0;

    let mut data = parse_dir_qt(&input);
    for dir in data {
        match dir.0 {
            Dir::F => {
                h_pos += dir.1;
                d_pos += (aim * dir.1);
            },
            Dir::U => {
                aim -= dir.1;
            },
            Dir::D => {
                aim += dir.1;
            }
        }
    }

    (h_pos * d_pos).to_string()
}

fn parse(input: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2021/day2-sample.txt");
        assert_eq!("150", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2021/day2.txt");
        assert_eq!("1746616", part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2021/day2-sample.txt");
        assert_eq!("900", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day2.txt");
        assert_eq!("1741971043", part2(input));
    }
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

pub struct Day2 {}

impl AOC for Day2 {
    fn part1(&self, input: Option<String>, args: Vec<String>) -> String {
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/2021/day2.txt"),
        };
        part1(input)
    }

    fn part2(&self, input: Option<String>, args: Vec<String>) -> String {
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/2021/day2.txt"),
        };
        part2(input)
    }
}
