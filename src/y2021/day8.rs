use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

fn part1(input: String) -> String {
    let mut left: Vec<Vec<&str>> = vec![];
    let mut right: Vec<Vec<&str>> = vec![];
    for line in input.lines() {
        let tmp = line.split_once(" | ").unwrap();
        left.push(tmp.0.split_ascii_whitespace().collect());
        right.push(tmp.1.split_ascii_whitespace().collect());
    }

    let mut qt_uniq = 0;

    for right_words in &right {
        // let map: Vec<Vec<char>> = vec![vec![]; 10];
        for word in right_words {
            match word.len() {
                2 | 3 | 4 | 7 => qt_uniq += 1,
                _ => (),
            }
        }
    }

    // dbg!(&left, &right);

    qt_uniq.to_string()
}

fn part2(input: String) -> String {
    "".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2021/day8-sample.txt");
        assert_eq!("26", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2021/day8.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2021/day8-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day8.txt");
        assert_eq!("", part2(input));
    }
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
// pub struct Day8 {}
// 
// impl AOC for Day8 {
//     fn part1(&self, input: Option<String>, args: Vec<String>) -> String {
//         let input = match input {
//             Some(input) => input,
//             None => util::read_file("inputs/2021/day8.txt"),
//         };
//         part1(input)
//     }
// 
//     fn part2(&self, input: Option<String>, args: Vec<String>) -> String {
//         let input = match input {
//             Some(input) => input,
//             None => util::read_file("inputs/2021/day8.txt"),
//         };
//         part2(input)
//     }
// }
