// Solution copied from UncleScientist
// https://www.youtube.com/watch?v=J9REs45c49k

use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;

/*


- "SNAFU works the same way, except it uses powers of five instead of ten."
- "So, because ten (in normal numbers) is two fives and no ones, in SNAFU it
   is written 20. Since eight (in normal numbers) is two fives minus two ones,
   it is written 2=."


*/

fn to_i64(nstr: &str) -> i64 {
    let mut result = 0;

    for c in nstr.chars() {
        match c {
            '0'..='2' => result = result * 5 + (c as u8 - b'0') as i64,
            '-' => result = result * 5 - 1,
            '=' => result = result * 5 - 2,
            _ => panic!("Invalid char {c}"),
        }
    }

    result
}

fn to_snafu(mut num: i64) -> String {
    let mut result = String::new();

    while num > 0 {
        let n = num % 5;

        match n {
            0..=2 => {
                result.push((n as u8 + b'0') as char);
                num /= 5;
            },
            3 => {
                result.push('=');
                num = (num + 2) / 5;
            }
            4 => {
                result.push('-');
                num = (num + 1) / 5;
            }
            _ => panic!("Invalid n {n}"),
        }
    }

    result.chars().rev().collect()
}

fn part1(input: String) -> String {
    let snafu_nums = parse(&input);
    dbg!(&snafu_nums);
    let nums: Vec<i64> = snafu_nums.into_iter().map(to_i64).collect();
    dbg!(&nums);
    let sum = nums.into_iter().sum();
    dbg!(&sum);

    to_snafu(sum)
}

fn part2(input: String) -> String {
    todo!()
}

fn parse(input: &String) -> Vec<&str> {
    input.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/day25-sample.txt");
        assert_eq!("2=-1=0", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/day25.txt");
        assert_eq!("2-0-0=1-0=2====20=-2", part1(input));
    }

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day25-sample.txt");
    //    assert_eq!("", part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day25.txt");
    //    assert_eq!("", part2(input));
    //}
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

pub struct Day25 {}

impl AOC for Day25 {
    fn part1(&self, input: Option<String>, args: Vec<String>) -> String {
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/day25.txt"),
        };
        part1(input)
    }

    fn part2(&self, input: Option<String>, args: Vec<String>) -> String {
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/day25.txt"),
        };
        part2(input)
    }
}
