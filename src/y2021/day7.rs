use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

fn part1(input: String) -> String {
    let nums: Vec<i32> = input.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
    let mut min = i32::MAX;
    let max = *nums.iter().max().unwrap();
    for t in 1..=max {
        let mut curr = 0;
        for n in &nums {
            curr += (n - t).abs();
        }
        min = min.min(curr);
    }

    min.to_string()
}

fn part2(input: String) -> String {
    let nums: Vec<i32> = input.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
    let mut min = i32::MAX;
    let max = *nums.iter().max().unwrap();
    for t in 1..=max {
        let mut curr = 0;
        for n in &nums {
            let diff = (n - t).abs() as usize;
            curr += util::sum_of_consecutive_numbers(0, diff as u32) as i32;
        }
        min = min.min(curr);
    }

    min.to_string()
}

fn parse(input: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2021/day7-sample.txt");
        assert_eq!("37", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2021/day7.txt");
        assert_eq!("355989", part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2021/day7-sample.txt");
        assert_eq!("168", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day7.txt");
        assert_eq!("102245489", part2(input));
    }
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

// pub struct Day7 {}
// 
// impl AOC for Day7 {
//     fn part1(&self, input: Option<String>, args: Vec<String>) -> String {
//         let input = match input {
//             Some(input) => input,
//             None => util::read_file("inputs/2021/day7.txt"),
//         };
//         part1(input)
//     }
// 
//     fn part2(&self, input: Option<String>, args: Vec<String>) -> String {
//         let input = match input {
//             Some(input) => input,
//             None => util::read_file("inputs/2021/day7.txt"),
//         };
//         part2(input)
//     }
// }
