use crate::util;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

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

    let mut lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    while lines.len() > 1 {
        let mut new_line = add_lines(&lines[0], &lines[1]);
        loop {
            let tmp = explode(&new_line);
            if tmp != new_line {
                new_line = tmp;
                continue;
            }
            let tmp = split(&new_line);
            if tmp == new_line {
                break;
            }
            new_line = tmp;
        }

        lines.remove(1);
        lines[0] = new_line;
    }

    dbg!(&lines[0]);
    let ans = reduce(&lines[0]);
    ans.to_string()
}

fn reduce(s: &str) -> u32 {
    let mut ret = s.to_string();
    while ret.chars().filter(|c| *c == ',').count() > 1 {
        ret = reduce_helper(&ret);
    }
    let ans = (&ret[1..ret.len() - 1]).split_once(',').unwrap();
    let left: u32 = ans.0.parse().unwrap();
    let right: u32 = ans.1.parse().unwrap();

    left * 3 + right * 2
}

fn reduce_helper(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        if chars[i] == ']' {
            let mut j = i - 1;
            while chars[j] != '[' {
                j -= 1;
            }
            let left_right = (&s[j+1..i]).split_once(',').unwrap();
            let left: u32 = left_right.0.parse().unwrap();
            let right: u32 = left_right.1.parse().unwrap();
            let v = left * 3 + right * 2;

            let mut ret = String::new();
            ret.push_str(&s[0..j]);
            ret.push_str(&v.to_string());
            ret.push_str(&s[i+1..]);

            return ret;
        }
    }
    panic!("not able to reduce it");
}

fn add_lines(s1: &str, s2: &str) -> String {
    let mut ret = String::new();
    ret.push('[');
    ret.push_str(s1);
    ret.push(',');
    ret.push_str(s2);
    ret.push(']');

    ret
}

fn split(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut ret = String::new();

    for i in 0..s.len() {
        if chars[i].is_digit(10) && chars[i+1].is_digit(10) {
            let mut vl = chars[i].to_digit(10).unwrap() * 10;
            vl += chars[i+1].to_digit(10).unwrap();
            let mut n_digits = 2;
            let mut j = i+2;
            while chars[j].is_digit(10) {
                vl += chars[j].to_digit(10).unwrap();
                n_digits += 1;
                j += 1;
            }
            let l = vl / 2;
            let r = vl / 2 + if vl % 2 == 0 { 0 } else { 1 };

            ret.push_str(&s[0..i]);
            ret.push('[');
            ret.push_str(&l.to_string());
            ret.push(',');
            ret.push_str(&r.to_string());
            ret.push(']');
            ret.push_str(&s[i+n_digits..]);

            break;
        }
    }

    if !ret.is_empty() {
        ret
    } else {
        s.into()
    }
}

fn explode(s: &str) -> String {
    let mut qt = 0;
    let mut ret = String::new();

    for (i, c) in s.chars().enumerate() {
        if c == '[' {
            if qt == 4 {
                let close = find_close(&s[i..]);
                let left_right = (&s[i+1..i+close]).split_once(',').unwrap();
                let left_num: u32 = left_right.0.parse().unwrap();
                let right_num: u32 = left_right.1.parse().unwrap();

                ret.push_str(&s[0..i]);
                ret.push('0');
                ret.push_str(&s[i+close+1..]);

                let chars: Vec<char> = ret.chars().collect();

                // adding left
                // find last number before i, if any
                let mut vl = 0;
                for l in (0..i).rev() {
                    let c = chars[l];
                    if c >= '0' && c <= '9' {
                        vl = c.to_digit(10).unwrap();
                        let mut j = l;
                        while chars[j - 1].is_digit(10) {
                            j -= 1;
                            let d = chars[j].to_digit(10).unwrap() * 10;
                            vl += d;
                        }
                        vl += left_num;

                        let mut tmp = String::new();
                        tmp.push_str(&ret[0..j]);
                        tmp.push_str(&vl.to_string());
                        tmp.push_str(&ret[l+1..]);
                        ret = tmp;
                        break;
                    }
                }

                let start = vl.to_string().len() + i;

                let chars: Vec<char> = ret.chars().collect();

                // adding right
                // find first number after close, if any
                for l in start..ret.len() {
                    let c = chars[l];
                    if c >= '0' && c <= '9' {
                        vl = c.to_digit(10).unwrap();
                        let mut j = l + 1;
                        while chars[j].is_digit(10) {
                            vl *= 10;
                            vl += chars[j].to_digit(10).unwrap();
                            j += 1;
                        }
                        vl += right_num;

                        let mut tmp = String::new();
                        tmp.push_str(&ret[0..l]);
                        tmp.push_str(&vl.to_string());
                        tmp.push_str(&ret[j..]);
                        ret = tmp;
                        break;
                    }
                }

                break;
            }
            qt += 1;
        } else if c == ']' {
            qt -= 1;
        }
    }

    if !ret.is_empty() {
        ret
    } else {
        s.into()
    }
}

fn part2(input: String) -> String {

    let mut best = 0;
    for l in input.lines() {
        for r in input.lines() {
            if l == r { continue; }
            let mut param = String::new();
            param.push_str(l);
            param.push('\n');
            param.push_str(r);

            let tmp: u32 = part1(param).parse().unwrap();
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
    fn test_split() {
        assert_eq!(split("[[[[0,7],4],[15,[0,13]]],[1,1]]"),
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".to_string());
        assert_eq!(split("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"),
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".to_string());
    }

    #[test]
    fn test_explode() {
        assert_eq!(explode("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]".to_string());
        assert_eq!(explode("[7,[6,[5,[4,[3,2]]]]]"), "[7,[6,[5,[7,0]]]]".to_string());
        assert_eq!(explode("[[6,[5,[4,[3,2]]]],1]"), "[[6,[5,[7,0]]],3]".to_string());
        assert_eq!(explode("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_string());
        assert_eq!(explode("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"),
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".to_string());
    }

    #[test]
    fn test_reduce() {
        assert_eq!(reduce("[[1,2],[[3,4],5]]"), 143);
        assert_eq!(reduce("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), 1384);
        assert_eq!(reduce("[[[[1,1],[2,2]],[3,3]],[4,4]]"), 445);
        assert_eq!(reduce("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"), 3488);
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
