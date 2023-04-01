use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;

fn part1(input: String) -> String {
    let mut times = parse(&input);

    for _ in 0..80 {
        let len = times.len();
        for i in 0..len {
            if times[i] == 0 {
                times[i] = 6;
                times.push(8);
            } else {
                times[i] -= 1;
            }
        }
    }

    times.len().to_string()
}

fn part2(input: String) -> String {
    let mut times = parse(&input);

    for _ in 0..256 {
        let len = times.len();
        for i in 0..len {
            if times[i] == 0 {
                times[i] = 6;
                times.push(8);
            } else {
                times[i] -= 1;
            }
        }
    }

    times.len().to_string()
}

fn parse(input: &str) -> Vec<i32> {
    input.split(',').map(|s| s.parse::<i32>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2021/day6-sample.txt");
        assert_eq!("5934", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2021/day6.txt");
        assert_eq!("353274", part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2021/day6-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day6.txt");
        assert_eq!("", part2(input));
    }
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

// pub struct Day6 {}
// 
// impl AOC for Day6 {
//     fn part1(&self, input: Option<String>, args: Vec<String>) -> String {
//         let input = match input {
//             Some(input) => input,
//             None => util::read_file("inputs/2021/day6.txt"),
//         };
//         part1(input)
//     }
// 
//     fn part2(&self, input: Option<String>, args: Vec<String>) -> String {
//         let input = match input {
//             Some(input) => input,
//             None => util::read_file("inputs/2021/day6.txt"),
//         };
//         part2(input)
//     }
// }
