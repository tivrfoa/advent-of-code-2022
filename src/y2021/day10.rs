use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

fn is_open(c: char) -> bool {
    match c {
        '{' | '[' | '<' | '(' => true,
        _ => false,
    }
}

fn part1(input: String) -> String {
    let p_cost = 3;
    let sq_cost = 57;
    let b_cost = 1197;
    let g_cost = 25137;

    let mut ans = 0;
    'l: for line in input.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            if is_open(c) {
                stack.push(c);
            } else {
                if stack.is_empty() {
                    match c {
                        ')' => {
                            ans += p_cost;
                            continue 'l;
                        }
                        ']' => {
                            ans += sq_cost;
                            continue 'l;
                            continue;
                        }
                        '}' => {
                            ans += b_cost;
                            continue 'l;
                            continue;
                        }
                        '>' => {
                            ans += g_cost;
                            continue 'l;
                            continue;
                        }
                        _ => panic!("invalid close char: {c}"),
                    }
                } else {
                    let last = stack.pop().unwrap();
                    match c {
                        ')' => {
                            if last != '(' {
                                ans += p_cost;
                                continue 'l;
                            }
                        }
                        ']' => {
                            if last != '[' {
                                ans += sq_cost;
                                continue 'l;
                            }
                        }
                        '}' => {
                            if last != '{' {
                                ans += b_cost;
                                continue 'l;
                            }
                        }
                        '>' => {
                            if last != '<' {
                                ans += g_cost;
                                continue 'l;
                            }
                        }
                        _ => panic!("invalid close char: {c}"),
                    }
                }
            }
        }
    }

    ans.to_string()
}

fn part2(input: String) -> String {
    let p_cost = 3;
    let sq_cost = 57;
    let b_cost = 1197;
    let g_cost = 25137;

    let mut incomplete_lines = vec![];
    'l: for line in input.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            if is_open(c) {
                stack.push(c);
            } else {
                if stack.is_empty() {
                    match c {
                        ')' => {
                            continue 'l;
                        }
                        ']' => {
                            continue 'l;
                        }
                        '}' => {
                            continue 'l;
                        }
                        '>' => {
                            continue 'l;
                        }
                        _ => panic!("invalid close char: {c}"),
                    }
                } else {
                    let last = stack.pop().unwrap();
                    match c {
                        ')' => {
                            if last != '(' {
                                continue 'l;
                            }
                        }
                        ']' => {
                            if last != '[' {
                                continue 'l;
                            }
                        }
                        '}' => {
                            if last != '{' {
                                continue 'l;
                            }
                        }
                        '>' => {
                            if last != '<' {
                                continue 'l;
                            }
                        }
                        _ => panic!("invalid close char: {c}"),
                    }
                }
            }
        }
        incomplete_lines.push(line);
    }

    let mut map = HashMap::new();
    map.insert('(', 1);
    map.insert('[', 2);
    map.insert('{', 3);
    map.insert('<', 4);

    let mut scores: Vec<u64> = vec![];
    for line in incomplete_lines {
        let mut stack = vec![];
        for c in line.chars() {
            if is_open(c) {
                stack.push(c);
            } else {
                stack.pop();
            }
        }
        let mut score = 0;
        while let Some(c) = stack.pop() {
            score = score * 5 + map.get(&c).unwrap();
        }
        scores.push(score);
    }
    scores.sort();

    (scores[scores.len() / 2]).to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2021/day10-sample.txt");
        assert_eq!("26397", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2021/day10.txt");
        assert_eq!("392043", part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2021/day10-sample.txt");
        assert_eq!("288957", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day10.txt");
        assert_eq!("1605968119", part2(input));
    }
}
