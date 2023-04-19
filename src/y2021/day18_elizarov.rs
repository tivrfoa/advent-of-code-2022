/*

Here I'll try to implement Roman Elizarov Kotlin solution in Rust
https://github.com/elizarov/AdventOfCode2021/blob/main/src/Day18.kt

*/

use crate::util;

use std::cell::Cell;
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

#[derive(Debug)]
enum SNum {
    Reg {x: Cell<u32> },
    Pair(Box<SNum>, Box<SNum>),
}

use SNum::*;

//impl SNum::Reg {
//    fn set_x(&mut self
//}

impl SNum {
    fn parse(s: &str) -> Box<SNum> {
        Self::parse_helper(s, &mut 0)
    }

    fn parse_helper(s: &str, i: &mut usize) -> Box<SNum> {
        if s.char_at(*i) == '[' {
            *i += 1;
            let l = Self::parse_helper(s, i);
            assert!(s.char_at(*i) == ',');
            *i += 1;
            let r = Self::parse_helper(s, i);
            assert!(s.char_at(*i) == ']');
            *i += 1;
            return Box::new(Pair(l, r));
        }
        let start = *i;
        while s.char_at(*i).is_digit(10) {
            *i += 1;
        }
        Self::new_boxed_reg(s[start..*i].parse::<u32>().unwrap())
    }

    fn new_reg(x: u32) -> SNum {
        Reg {
            x: Cell::new(x),
        }
    }

    fn new_boxed_reg(x: u32) -> Box<SNum> {
        Box::new(Reg {
            x: Cell::new(x)
        })
    }

    /// This method returns a &mut reference to the
    /// parent of the pair, if found
    fn find_pair(&mut self, n: u32) -> Option<(&mut SNum, char)> {
        if n == 1 {
            match self {
                Pair(l, r) => {
                    if l.is_pair() {
                        return Some((self, 'l'));
                    }
                    if r.is_pair() {
                        return Some((self, 'r'));
                    }
                }
                _ => return None,
            }
        }
        if let Pair(l, r) = self {
            let p = l.find_pair(n - 1);
            if p.is_some() { return p; }
            let p = r.find_pair(n - 1);
            if p.is_some() { return p; }
        }
        None
    }

    fn get_x(&self) -> u32 {
        match self {
            Reg { x } => x.get(),
            _ => panic!("self not reg: {:?}", self),
        }
    }

    fn get_old_values(&self) -> (u32, u32) {
        match self {
            Pair(l, r) => (l.get_x(), r.get_x()),
            _ => panic!("self not Pair: {:?}", self),
        }
    }

    fn traverse<'a>(&'a self, keep: &'a SNum) -> Vec<&'a SNum> {
        match keep {
            Reg { x } => vec![keep],
            Pair(l, r) => {
                if self as *const _ == keep as *const _ {
                    return vec![self];
                }
                let mut ret = l.traverse(keep);
                ret.append(&mut r.traverse(keep));
                return ret;
            }
        }
    }

    fn explode(&mut self) -> bool {
        let mut n = self.find_pair(4);
        // dbg!(&n);
        if let Some((Pair(l, r), side)) = n {
            match side {
                'l' => {
                    let (old_left, old_right) = l.get_old_values();
                    *l = Self::new_boxed_reg(0);
                    drop(n);

                    let list = self.traverse(&l);
                    let i = list
                        .iter()
                        .position(|&p| p as *const _ == (&**l) as *const _)
                        .unwrap();

                }
                'r' => {
                    *r = Self::new_boxed_reg(0);
                }
                _ => panic!("{side}"),
            }
            true
        } else {
            false
        }
    }

    /// This method is called from the outer pair
    fn split(&mut self) -> bool {
        match self {
            _ => (),
            Pair(l, r) => {
                match &**l {
                    Reg { x } => {
                        let x = x.get();
                        if x >= 10 {
                            *l = Box::new(Pair(
                                Self::new_boxed_reg(x / 2),
                                Self::new_boxed_reg(
                                    x / 2 + if x % 2 == 0 { 0 } else { 1 }
                                ),
                            ));
                            return true;
                        }
                    }
                    _ => {
                        if l.split() {
                            return true;
                        }
                    },
                }
                match &**r {
                    Reg { x } => {
                        let x = x.get();
                        if x >= 10 {
                            *r = Box::new(Pair(
                                Self::new_boxed_reg(x / 2),
                                Self::new_boxed_reg(
                                    x / 2 + if x % 2 == 0 { 0 } else { 1 }
                                ),
                            ));
                            return true;
                        }
                    }
                    _ => {
                        if r.split() {
                            return true;
                        }
                    },
                }
            }
        }
        false
    }

    fn is_pair(&self) -> bool {
        matches!(*self, Pair(_, _))
    }
}

fn part1(input: String) -> String {
    // cannot call associated function of trait
    // let a = SNum::parse(&input);

    let lines: Vec<&str> = input.lines().collect();
    let mut a = SNum::parse(lines[0]);
    dbg!(&a);
    a.explode();

    let mut b = SNum::parse(lines[1]);

    let mut ab = Pair(a, b);
    dbg!(&ab);
    loop {
        while ab.explode() {};
        if !ab.split() {
            break;
        }
    }
    dbg!(&ab);
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
