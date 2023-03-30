use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;

fn part1(input: String) -> String {

    let mut ans: u32 = 0;
    let nums = util::input_as_vec_i32(&input);
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            ans += 1;
        }
    }
    ans.to_string()
}

fn three_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];

    for i in 2..nums.len() {
        ans.push(nums[i-2] + nums[i-1] + nums[i]);
    }

    ans
}

fn part2(input: String) -> String {

    let mut ans: u32 = 0;
    let nums = util::input_as_vec_i32(&input);
    let nums = three_sum(nums);
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            ans += 1;
        }
    }
    ans.to_string()
}

fn parse(input: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2021/day1-sample.txt");
        assert_eq!("7", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2021/day1.txt");
        assert_eq!("1316", part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2021/day1-sample.txt");
        assert_eq!("5", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day1.txt");
        assert_eq!("1344", part2(input));
    }
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

pub struct Day1 {}

impl AOC for Day1 {
    fn part1(&self, input: Option<String>, args: Vec<String>) -> String {
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/2021/day1.txt"),
        };
        part1(input)
    }

    fn part2(&self, input: Option<String>, args: Vec<String>) -> String {
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/2021/day1.txt"),
        };
        part2(input)
    }
}
