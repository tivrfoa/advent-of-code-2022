use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;

fn part1(input: String) -> String {

    let mut g = String::new();
    let mut e = String::new();
    let len = input.lines().next().unwrap().len();
    let mut ones = vec![0; len];
    let mut zeros = vec![0; len];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                ones[i] += 1;
            } else {
                zeros[i] += 1;
            }
        }
    }

    for i in 0..len {
        if ones[i] > zeros[i] {
            g.push('1');
            e.push('0');
        } else {
            g.push('0');
            e.push('1');
        }
    }

    (i32::from_str_radix(&g, 2).unwrap() * i32::from_str_radix(&e, 2).unwrap()).to_string()
}

fn part2(input: String) -> String {
    todo!()
}

fn parse(input: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2021/day3-sample.txt");
        assert_eq!("198", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2021/day3.txt");
        assert_eq!("", part1(input));
    }

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/2021/day3-sample.txt");
    //    assert_eq!("", part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/2021/day3.txt");
    //    assert_eq!("", part2(input));
    //}
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

pub struct Day3 {}

impl AOC for Day3 {
    fn part1(&self, input: Option<String>, args: Vec<String>) -> String {
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/2021/day3.txt"),
        };
        part1(input)
    }

    fn part2(&self, input: Option<String>, args: Vec<String>) -> String {
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/2021/day3.txt"),
        };
        part2(input)
    }
}
