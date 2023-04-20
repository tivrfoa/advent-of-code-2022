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

#[derive(Debug, PartialEq)]
enum SNum {
    Reg {x: Cell<u32> },
    Pair(Rc<RefCell<SNum>>, Rc<RefCell<SNum>>),
}

use SNum::*;


impl SNum {
    fn parse(s: &str) -> Rc<RefCell<SNum>> {
        Self::parse_helper(s, &mut 0)
    }

    fn parse_helper(s: &str, i: &mut usize) -> Rc<RefCell<SNum>> {
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
        Rc::new(RefCell::new(Self::new_reg(s[start..*i].parse::<u32>().unwrap())))
    }

    fn new_reg(x: u32) -> SNum {
        Reg {
            x: Cell::new(x),
        }
    }

    fn new_rc_reg(x: u32) -> RcPair {
        Rc::new(RefCell::new(Reg {
            x: Cell::new(x)
        }))
    }

    fn find_pair(snum: RcPair, n: u32) -> Option<RcPair> {
        if n == 0 {
            if snum.borrow().is_pair() {
                return Some(snum.clone());
            } else {
                return None;
            }
        }
        if let Pair(l, r) = &*snum.borrow() {
            let p = SNum::find_pair(l.clone(), n - 1);
            if p.is_some() { return p; }
            let p = SNum::find_pair(r.clone(), n - 1);
            if p.is_some() { return p; }
        }
        None
    }

    fn traverse(curr: RcPair, keep: RcPair) -> Vec<RcPair> {
        match &*curr.borrow() {
            Reg { x } => vec![curr.clone()],
            Pair(l, r) => {
                if Rc::ptr_eq(&curr, &keep) {
                    return vec![curr.clone()];
                }
                let mut ret = SNum::traverse(l.clone(), keep.clone());
                ret.append(&mut SNum::traverse(r.clone(), keep.clone()));
                return ret;
            }
        }
    }

    fn get_old_values(&self) -> (u32, u32) {
        let mut left = 0;
        let mut right = 0;

        if let Pair(l, r) = self {
            if let Reg { x } = &*l.borrow() {
                left = x.get();
            } else {
                panic!("not a reg")
            }
            if let Reg { x } = &*r.borrow() {
                right = x.get();
            } else {
                panic!("not a reg")
            }
        } else {
            panic!("not a pair");
        }

        (left, right)
    }

    fn explode(root: RcPair) -> bool {
        eprintln!("exploding");
        let mut n = SNum::find_pair(root.clone(), 4);
        if let Some(rcPair) = n {
            let list = SNum::traverse(root.clone(), rcPair.clone());
            let i = list
               .iter()
               .position(|p| Rc::ptr_eq(p, &rcPair))
               .unwrap();

            let (old_left, old_right) = rcPair.borrow().get_old_values();

            // update left
            if i > 0 {
                match &*list[i - 1].borrow_mut() {
                    Reg { x } => x.set(x.get() + old_left),
                    Pair(_, _) => panic!("it should have been reg: {:?}", list[i - 1]),
                }
            }

            // update right
            if i + 1 < list.len() {
                match &*list[i + 1].borrow_mut() {
                    Reg { x } => x.set(x.get() + old_right),
                    Pair(_, _) => {
                        panic!("it should have been reg: {:?}", list[i + 1]);
                    }
                }
            }

            // set exploded to zero
            *rcPair.borrow_mut() = Reg {
                x: Cell::new(0),
            };

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
                match &*l.borrow() {
                    Reg { x } => {
                        let x = x.get();
                        if x >= 10 {
                            *l.borrow_mut() = Pair(
                                Self::new_rc_reg(x / 2),
                                Self::new_rc_reg(
                                    x / 2 + if x % 2 == 0 { 0 } else { 1 }
                                ),
                            );
                            return true;
                        }
                    }
                    _ => {
                        if SNum::split(l.clone()) {
                            return true;
                        }
                    },
                }
                match &*r.borrow() {
                    Reg { x } => {
                        let x = x.get();
                        if x >= 10 {
                            *r.borrow_mut() = Pair(
                                Self::new_rc_reg(x / 2),
                                Self::new_rc_reg(
                                    x / 2 + if x % 2 == 0 { 0 } else { 1 }
                                ),
                            );
                            return true;
                        }
                    }
                    _ => {
                        if SNum::split(r.clone()) {
                            return true;
                        }
                    },
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
        eprintln!(">>>>>>>>>>>>>>>>>>>> joining pairs");
        let mut b = SNum::parse(line);
        a = Rc::new(RefCell::new(Pair(a, b)));
        SNum::explode(a.clone());
        loop {
            while SNum::explode(a.clone()) {
                // dbg!(&a);
            }
            
            if !SNum::split(a.clone()) {
                break;
            }
        }
    }
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
