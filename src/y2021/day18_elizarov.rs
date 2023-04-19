/*

Here I'll try to implement Roman Elizarov Kotlin solution in Rust
https://github.com/elizarov/AdventOfCode2021/blob/main/src/Day18.kt

*/

use crate::util;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

#[allow(dead_code)]
fn is_digit(s: &str) -> bool {
    s.chars().next().unwrap().is_digit(10)
}

trait StrLetterAt {
    fn char_at(&self, i: usize) -> char;
}

impl StrLetterAt for &str {
    fn char_at(&self, i: usize) -> char {
        self[i..i+1].chars().next().unwrap()
    }
}

trait SNum {
    fn parse(s: &str) -> Box<dyn SNum> where Self: Sized {
        Self::parse_helper(s, &mut 0)
    }

    fn parse_helper(s: &str, i: &mut usize) -> Box<dyn SNum> where Self: Sized {
        if s.char_at(*i) == '[' {
            *i += 1;
            let l = Self::parse_helper(s, i);
            assert!(s.char_at(*i) == ',');
            *i += 1;
            let r = Self::parse_helper(s, i);
            assert!(s.char_at(*i) == ']');
            *i += 1;
            return Box::new(Pair::new(l, r));
        }
        let start = *i;
        while s.char_at(*i).is_digit(10) {
            *i += 1;
        }
        Box::new(Reg {
            x: s[start..*i].parse::<u32>().unwrap()
        })
    }

    // TODO: should return a copy? should consume self?
    fn findPair(&self, n: u32) -> Option<Pair> {
        // if n == 0
        todo!()
    }
}

impl Debug for dyn SNum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result {
        write!(f, 
    }
}

#[derive(Debug)]
struct Reg {
    x: u32
}

impl SNum for Reg {
}

#[derive(Debug)]
struct Pair {
    l: Box<dyn SNum>,
    r: Box<dyn SNum>,
}

impl SNum for Pair {
}

impl Pair {
    fn new(l: Box<dyn SNum>, r: Box<dyn SNum>) -> Self {
        Self {
            l, r
        }
    }
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

fn part1(input: String) -> String {
    // cannot call associated function of trait
    // let a = SNum::parse(&input);
    let a = <Pair as SNum>::parse(&input);
    dbg!(a);
    todo!()
}

fn part2(input: String) -> String {
    todo!()
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
    fn test_split() {
    }

    #[test]
    fn test_explode() {
    }

    #[test]
    fn test_reduce() {
    }

    #[test]
    fn p1s() {
        let input = util::read_file("inputs/2021/day18-sample.txt");
        assert_eq!("4140", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day18.txt");
        assert_eq!("4469", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day18-sample.txt");
        assert_eq!("3993", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day18.txt");
        assert_eq!("4770", part2(input));
    }
}
