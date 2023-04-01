use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn part2(input: String) -> String {
    let mut tmp = parse(&input);
    let mut times = [0u64; 9];
    for t in tmp {
        times[t as usize] += 1;
    }
    dbg!(&times);

    for _ in 0..256 {
        let new_fishes = times[0];
        let mut prev = times[8];
        for i in (0..=7).rev() {
            let tmp = times[i];
            times[i] = prev;
            prev = tmp;
        }
        times[6] += new_fishes;
        times[8] = new_fishes;
    }

    times.iter().sum::<u64>().to_string()
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
        assert_eq!("26984457539", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day6.txt");
        assert_eq!("1609314870967", part2(input));
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
