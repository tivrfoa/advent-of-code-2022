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

    let mut sum: u32 = 0;

    for (left_words, right_words) in left.iter().zip(right.iter()) {
        let mut map: Vec<Vec<char>> = vec![vec![]; 10];
        for word in left_words {
            match word.len() {
                2 if map[1].is_empty() => map[1] = word.chars().collect(),
                3 if map[7].is_empty() => map[7] = word.chars().collect(),
                4 if map[4].is_empty() => map[4] = word.chars().collect(),
                7 if map[8].is_empty() => map[8] = word.chars().collect(),
                _ => (),
            }
        }

        // comparing letters 1 and 7 we will know
        // which is letter a
        let a: char = *map[7]
            .iter()
            .filter(|c| !map[1].contains(c))
            .next()
            .unwrap();

        // we can get letters b and d comparing 1 and 4,
        // although we don't know the order yet
        let bd: Vec<char> = map[4]
            .iter()
            .filter(|c| !map[1].contains(c))
            .map(|c| *c)
            .collect();

        // e g = 8 - 4 - a
        let eg: Vec<char> = map[8]
            .iter()
            .filter(|c| **c != a && !map[4].contains(c))
            .map(|c| *c)
            .collect();

        map[2] = left_words
            .iter()
            .filter(|w| w.len() == 5 && w.contains(eg[0]) && w.contains(eg[1]))
            .next()
            .unwrap()
            .chars()
            .collect();

        let b: char = *bd.iter()
            .filter(|c| !map[2].contains(c))
            .next()
            .unwrap();
        let d: char = *bd.iter().filter(|&c| *c != b).next().unwrap();

        map[5] = left_words
            .iter()
            .filter(|w| w.len() == 5 && w.contains(b) && w.contains(d))
            .next()
            .unwrap()
            .chars()
            .collect();

        // c = 4 - 5
        let c: char = *map[4]
            .iter()
            .filter(|c| !map[5].contains(c))
            .next()
            .unwrap();

        let f: char = *map[1].iter().filter(|&c0| *c0 != c).next().unwrap();

        // find 0, 6 and 9
        for word in left_words {
            if word.len() == 6 {
                if word.contains(eg[0]) && word.contains(eg[1]) {
                    if word.contains(c) {
                        map[0] = word.chars().collect();
                    } else {
                        map[6] = word.chars().collect();
                    }
                } else {
                    map[9] = word.chars().collect();
                }
            }
        }

        // e = 6 - 9
        let e: char = *map[6]
            .iter()
            .filter(|c| !map[9].contains(c))
            .next()
            .unwrap();

        let g: char = eg.into_iter().filter(|&c| c != e).next().unwrap();


        let mut word = String::new();
        for right_word in right_words {
            match right_word.len() {
                2 => {
                    word.push('1');
                }
                3 => {
                    word.push('7');
                }
                4 => {
                    word.push('4');
                }
                5 => {
                    if word.contains(b) {
                        word.push('5');
                    } else if word.contains(f) {
                        word.push('3');
                    } else {
                        word.push('2');
                    }
                }
                6 => {
                    if !word.contains(d) {
                        word.push('0');
                    } else if word.contains(e) {
                        word.push('6');
                    } else {
                        word.push('9');
                    }
                }
                7 => {
                    word.push('8');
                }
                l @ _ => panic!("invalid length {l}"),
            }
        }
        dbg!(&word);
        sum += word.parse::<u32>().unwrap();
    }

    sum.to_string()
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

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/2021/day8.txt");
    //    assert_eq!("", part2(input));
    //}
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
