/*

Here I'll try to implement Roman Elizarov Kotlin solution in Rust
https://github.com/elizarov/AdventOfCode2021/blob/main/src/Day18.kt

*/

use crate::util::{self, dbg};

use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;
use std::rc::Rc;

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

type RcPair = Rc<RefCell<SNum>>;

#[derive(Debug)]
enum SNum {
    Reg {x: u32 },
    Pair(RcPair, RcPair),
}

use SNum::*;


impl SNum {
    fn parse(s: &str) -> RcPair {
        Self::parse_helper(s, &mut 0)
    }

    fn parse_helper(s: &str, i: &mut usize) -> RcPair {
        if s.char_at(*i) == '[' {
            *i += 1;
            let l = Self::parse_helper(s, i);
            assert!(s.char_at(*i) == ',');
            *i += 1;
            let r = Self::parse_helper(s, i);
            assert!(s.char_at(*i) == ']');
            *i += 1;
            return Rc::new(RefCell::new(Pair(l, r)));
        }
        let start = *i;
        while s.char_at(*i).is_digit(10) {
            *i += 1;
        }
        Self::new_rc_reg(s[start..*i].parse::<u32>().unwrap())
    }

    fn new_rc_reg(x: u32) -> RcPair {
        Rc::new(RefCell::new(Reg {
            x,
        }))
    }

    fn find_pair(snum: &RcPair, n: u32) -> Option<RcPair> {
        if n == 0 {
            if snum.borrow().is_pair() {
                return Some(snum.clone());
            } else {
                return None;
            }
        }
        if let Pair(l, r) = &*snum.borrow() {
            if let Some(p) = SNum::find_pair(&l, n - 1) {
                return Some(p);
            }
            if let Some(p) = SNum::find_pair(&r, n - 1) {
                return Some(p);
            }
        }
        None
    }

    fn traverse(curr: &RcPair, keep: &RcPair) -> Vec<RcPair> {
        match &*curr.borrow() {
            Reg { x } => vec![curr.clone()],
            Pair(l, r) => {
                if Rc::ptr_eq(curr, keep) {
                    return vec![curr.clone()];
                }
                let mut ret = SNum::traverse(&l, keep);
                ret.append(&mut SNum::traverse(&r, keep));
                return ret;
            }
        }
    }

    fn get_old_values(&self) -> (u32, u32) {
        let mut left = 0;
        let mut right = 0;

        if let Pair(l, r) = self {
            if let Reg { x } = &*l.borrow() {
                left = *x;
            } else {
                panic!("not a reg")
            }
            if let Reg { x } = &*r.borrow() {
                right = *x;
            } else {
                panic!("not a reg")
            }
        } else {
            panic!("not a pair");
        }

        (left, right)
    }

    fn magnitude(self) -> u32 {
        match self {
            Reg { x } => x,
            Pair(l, r) => 3 * Rc::try_unwrap(l).unwrap().into_inner().magnitude() +
                2 * Rc::try_unwrap(r).unwrap().into_inner().magnitude(),
        }
    }

    fn explode(root: RcPair) -> bool {
        eprintln!("exploding");
        let n = SNum::find_pair(&root, 4);
        if let Some(rcPair) = n {
            let list = SNum::traverse(&root, &rcPair);
            let i = list
               .iter()
               .position(|p| Rc::ptr_eq(p, &rcPair))
               .unwrap();

            let (old_left, old_right) = rcPair.borrow().get_old_values();

            // update left
            if i > 0 {
                match &mut *list[i - 1].borrow_mut() {
                    Reg { x } => *x += old_left,
                    Pair(_, _) => panic!("it should have been reg: {:?}", list[i - 1]),
                }
            }

            // update right
            if i + 1 < list.len() {
                match &mut *list[i + 1].borrow_mut() {
                    Reg { x } => *x += old_right,
                    Pair(_, _) => panic!("it should have been reg: {:?}", list[i + 1]),
                }
            }

            // set exploded to zero
            *rcPair.borrow_mut() = Reg {
                x: 0,
            };

            true
        } else {
            false
        }
    }

    fn split_x(pair: RcPair) -> bool {
        let mut new_pair = None;
        match &*pair.borrow() {
            Reg { x } => {
                let x = *x;
                if x >= 10 {
                    new_pair = Some(Pair(
                        Self::new_rc_reg(x / 2),
                        Self::new_rc_reg(
                            x / 2 + if x % 2 == 0 { 0 } else { 1 }
                        ),
                    ));
                }
            }
            _ => {
                if SNum::split(pair.clone()) {
                    return true;
                }
            },
        }

        if let Some(p) = new_pair {
            *pair.borrow_mut() = p;
            true
        } else {
            false
        }
    }

    /// This method is called from the outer pair (root)
    fn split(root: RcPair) -> bool {
        eprintln!("spliting");
        match &*root.borrow() {
            Pair(l, r) => {
                if SNum::split_x(l.clone()) || SNum::split_x(r.clone()) {
                    return true;
                }
            }
            _ => (),
        }
        false
    }

    fn is_pair(&self) -> bool {
        matches!(*self, Pair(_, _))
    }
}

fn part1(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut a = SNum::parse(lines[0]);

    for line in lines.iter().skip(1) {
        let b = SNum::parse(line);
        a = Rc::new(RefCell::new(Pair(a, b)));
        loop {
            while SNum::explode(a.clone()) {}
            if !SNum::split(a.clone()) {
                break;
            }
        }
    }

    Rc::try_unwrap(a).unwrap().into_inner().magnitude().to_string()
}

fn part2(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut best = 0;
    for a in input.lines() {
        for b in input.lines() {
            if a == b { continue; }
            let mut a = SNum::parse(a);
            let b = SNum::parse(b);
            a = Rc::new(RefCell::new(Pair(a, b)));
            loop {
                while SNum::explode(a.clone()) {}
                if !SNum::split(a.clone()) {
                    break;
                }
            }
            let tmp = Rc::try_unwrap(a).unwrap().into_inner().magnitude();
            best = best.max(tmp);
        }
    }

    best.to_string()
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
