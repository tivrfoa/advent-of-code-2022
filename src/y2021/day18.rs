use crate::util;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;


#[derive(Debug, PartialEq)]
enum Pair {
    PairPair(Box<Pair>, Box<Pair>),
    NumberPair(u32, Box<Pair>),
    PairNumber(Box<Pair>, u32),
    NumberNumber(u32, u32),
    Number(u32),
    Null,
}

fn c_to32(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

/// first index is open [ [
fn find_close(s: &str) -> usize {
    let mut qt = 1;

    for (i, c) in s.chars().enumerate().skip(1) {
        if c == '[' {
            qt += 1;
        } else if c == ']' {
            qt -= 1;
            if qt == 0 {
                return i;
            }
        } else {
            continue;
        }
    }
    panic!("did not find close ]");
}

impl Pair {
    
    fn parse(s: &str) -> Pair {
        assert!(&s[0..1] == "[");
        Pair::parse_helper(&s[1..])
    }

    fn parse_helper(s: &str) -> Pair {
        let mut left = Null;
        let mut right = Null;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        let mut i = 0;
        while i < len {
            if chars[i] == ',' {
                i += 1;
                continue;
            }
            if chars[i] >= '0' && chars[i] <= '9' {
                if left == Null {
                    left = Number(c_to32(chars[i]));
                    i += 1;
                    continue;
                } else {
                    right = Number(c_to32(chars[i]));
                    break;
                }
            }

            if chars[i] == '[' {
                if left == Null {
                    let close = find_close(&s[i..]);
                    left = Pair::parse(&s[i..i+close]);
                    i += close + 1;
                    continue;
                } else {
                    let close = find_close(&s[i..]);
                    right = Pair::parse(&s[i..i+close]);
                    break;
                }
            }

            if chars[i] == ']' {
                panic!("is it a valid state??");
            }

            panic!("wtf! {}", chars[i]);
        }

        match (&left, &right) {
            (Number(l), Number(r)) => {
                NumberNumber(*l, *r)
            }
            (Number(l), NumberNumber(_, _)) => {
                NumberPair(*l, Box::new(right))
            }
            (NumberNumber(_, _), Number(r)) => {
                PairNumber(Box::new(left), *r)
            }
            (NumberNumber(_, _), NumberNumber(_, _)) => {
                PairPair(Box::new(left), Box::new(right))
            }
            (PairPair(_, _), PairPair(_, _)) => {
                PairPair(Box::new(left), Box::new(right))
            }
            (PairPair(_, _), Number(r)) => {
                PairNumber(Box::new(left), *r)
            }
            (Number(l), PairPair(_, _)) => {
                NumberPair(*l, Box::new(right))
            }
            (Number(l), NumberPair(_, _)) => {
                NumberPair(*l, Box::new(right))
            }
            (NumberPair(_, _), Number(r)) => {
                PairNumber(Box::new(left), *r)
            }
            (NumberPair(_, _), NumberPair(_, _)) => {
                PairPair(Box::new(left), Box::new(right))
            }
            (PairNumber(_, _), PairNumber(_, _)) => {
                PairPair(Box::new(left), Box::new(right))
            }
            (PairNumber(_, _), NumberPair(_, _)) => {
                PairPair(Box::new(left), Box::new(right))
            }
            (NumberPair(_, _), PairNumber(_, _)) => {
                PairPair(Box::new(left), Box::new(right))
            }
            (PairPair(_, _), NumberPair(_, _)) => PairPair(Box::new(left), Box::new(right)),
            (PairPair(_, _), PairNumber(_, _)) => PairPair(Box::new(left), Box::new(right)),
            (PairPair(_, _), NumberNumber(_, _)) => PairPair(Box::new(left), Box::new(right)),
            (NumberPair(_, _), PairPair(_, _)) => PairPair(Box::new(left), Box::new(right)),
            (NumberPair(_, _), NumberNumber(_, _)) => PairPair(Box::new(left), Box::new(right)),
            (PairNumber(_, _), PairPair(_, _)) => PairPair(Box::new(left), Box::new(right)),
            (PairNumber(_, _), NumberNumber(_, _)) => PairPair(Box::new(left), Box::new(right)),
            (PairNumber(_, _), Number(r)) => PairNumber(Box::new(left), *r),
            (NumberNumber(_, _), PairPair(_, _)) => PairPair(Box::new(left), Box::new(right)),
            (NumberNumber(_, _), NumberPair(_, _)) => PairPair(Box::new(left), Box::new(right)),
            (NumberNumber(_, _), PairNumber(_, _)) => PairPair(Box::new(left), Box::new(right)),
            (Number(l), PairNumber(_, _)) => NumberPair(*l, Box::new(right)),
            (_, _) => panic!("{:?} {:?}", left, right),
        }
    }
}

use Pair::*;

fn part1(input: String) -> String {

    let np = NumberPair(1, Box::new(NumberNumber(1, 2)));
    dbg!(np);

    dbg!(Pair::parse("[4,5]")); // ok
    dbg!(Pair::parse("[3,[4,5]]")); // ok
    dbg!(Pair::parse("[[3,4],5]")); // ok
    dbg!(Pair::parse("[[3,4],[4,5]]")); // ok
    dbg!(Pair::parse("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]")); // ok
    dbg!(Pair::parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]")); // ok


    "1".into()
}

fn part2(input: String) -> String {
    "".into()
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

#[allow(dead_code)]
fn str_to_char_tuple(s: &str) -> (char, char) {
    (s[0..1].chars().next().unwrap(), s[1..2].chars().next().unwrap())
}

#[allow(dead_code)]
trait MapAddOrInsert<K, V> {
    fn add_or_insert(&mut self, k: K, v: V);
}

#[allow(dead_code)]
impl<K: Eq + Hash, V: std::ops::AddAssign + Copy> MapAddOrInsert<K, V> for HashMap<K, V> {
    fn add_or_insert(&mut self, k: K, v: V) {
        self.entry(k).and_modify(|qt| *qt += v).or_insert(v);
    }
}

#[allow(dead_code)]
fn get_dirs(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 4] {
    [
        // left
        (c > 0, (r, if c > 0 { c - 1 } else { 0 })),
        // right
        (c < cols - 1, (r, c + 1)),
        // top
        (r > 0, (if r > 0 { r - 1 } else { 0 }, c)),
        // bottom
        (r < rows - 1, (r + 1, c)),
    ]
}

#[allow(dead_code)]
fn get_dirs_with_diagonals(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 8] {
    [
        // left
        (c > 0, (r, if c > 0 { c - 1 } else { 0 })),
        // right
        (c < cols - 1, (r, c + 1)),
        // top
        (r > 0, (if r > 0 { r - 1 } else { 0 }, c)),
        // bottom
        (r < rows - 1, (r + 1, c)),
        // top left
        (
            r > 0 && c > 0,
            (if r > 0 { r - 1 } else { 0 }, if c > 0 { c - 1 } else { 0 }),
        ),
        // top right
        (
            r > 0 && c < cols - 1,
            (if r > 0 { r - 1 } else { 0 }, c + 1),
        ),
        // bottom left
        (
            r < rows - 1 && c > 0,
            (r + 1, if c > 0 { c - 1 } else { 0 }),
        ),
        // bottom right
        (r < rows - 1 && c < cols - 1, (r + 1, c + 1)),
    ]
}

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = util::read_file("inputs/2021/day18-sample.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day18.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day18-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day18.txt");
        assert_eq!("", part2(input));
    }
}
