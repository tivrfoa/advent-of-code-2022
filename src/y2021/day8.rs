use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
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
        for word in right_words {
            match word.len() {
                2 | 3 | 4 | 7 => qt_uniq += 1,
                _ => (),
            }
        }
    }

    qt_uniq.to_string()
}

fn part2(input: String) -> String {
    let mut left: Vec<Vec<&str>> = vec![];
    let mut right: Vec<Vec<&str>> = vec![];
    for line in input.lines() {
        let tmp = line.split_once(" | ").unwrap();
        left.push(tmp.0.split_ascii_whitespace().collect());
        right.push(tmp.1.split_ascii_whitespace().collect());
    }

    let mut sum = 0;

    for (left_words, right_words) in left.iter().zip(right.iter()) {
        let mut map: Vec<Vec<char>> = vec![vec![]; 10];
        for word in left_words {
            match word.len() {
                2 => map[1] = word.chars().collect(),
                3 => map[7] = word.chars().collect(),
                4 => map[4] = word.chars().collect(),
                7 => map[8] = word.chars().collect(),
                _ => (),
            }
        }

        let cf = map[1].clone();
        let bd = map[4].minus(&map[1]);

        for word in left_words {
            match word.len() {
                6 => {
                    if word.intersect(&bd).len() == 1 {
                        map[0] = word.chars().collect();
                    } else if word.intersect(&cf).len() == 1 {
                        map[6] = word.chars().collect();
                    } else {
                        map[9] = word.chars().collect();
                    }
                }
                5 => {
                    if word.intersect(&cf).len() == 2 {
                        map[3] = word.chars().collect();
                    } else if word.intersect(&bd).len() == 2 {
                        map[5] = word.chars().collect();
                    } else {
                        map[2] = word.chars().collect();
                    }
                }
                _ => (),
            }
        }

        for m in &mut map { m.sort(); }

        let mut n = 0;

        for rw in right_words {
            let mut rw: Vec<char> = rw.chars().collect();
            rw.sort();
            for (i, m) in map.iter().enumerate() {
                if *m == rw {
                    n = n * 10 + i;
                    break;
                }
            }
        }

        sum += n;
    }

    sum.to_string()
}

trait Minus {
    fn minus(&self, b: &[char]) -> Vec<char>;
}
impl Minus for Vec<char> {
    fn minus(&self, b: &[char]) -> Vec<char> {
        self.iter()
            .filter(|c| !b.contains(c))
            .map(|c| *c)
            .collect()
    }
}

trait Intersect {
    fn intersect(&self, b: &[char]) -> Vec<char>;
}
impl Intersect for &str {
    fn intersect(&self, b: &[char]) -> Vec<char> {
        self.chars().filter(|c| b.contains(c)).collect()
    }
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
        assert_eq!("375", part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2021/day8-sample.txt");
        assert_eq!("61229", part2(input));
    }

    #[test]
    fn part2_input() {
       let input = util::read_file("inputs/2021/day8.txt");
       assert_eq!("1019355", part2(input));
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
