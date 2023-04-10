use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

fn part1(input: String) -> String {
    let polymer_template = input.lines().next().unwrap();
    let mut map: HashMap<&str, &str> = HashMap::new();

    for line in input.lines().skip(2) {
        let tmp = line.split_once(" -> ").unwrap();
        map.insert(tmp.0, tmp.1);
    }

    let mut curr: String = polymer_template.into();

    // println!("Template: {}", curr);
    for i in 1..=10 {
        let mut new_str = String::with_capacity(curr.len() * 2);
        for i in 0..curr.len() - 1 {
            new_str.push_str(&curr[i..i+1]);
            new_str.push_str(map.get(&curr[i..i+2]).unwrap());
        }
        new_str.push_str(&curr[curr.len()-1..]);
        curr = new_str;
        // println!("After step {}: {}", i, curr);
    }

    let mut qt_map: HashMap<char, u32> = HashMap::new();
    for c in curr.chars() {
        if let Some(qt) = qt_map.get_mut(&c) {
            *qt += 1;
        } else {
            qt_map.insert(c, 1);
        }
    }

    let mut lc = u32::MAX;
    let mut mc = 0;
    for (_, v) in qt_map {
        if v < lc {
            lc = v;
        }
        if v > mc {
            mc = v;
        }
    }

    (mc - lc).to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = util::read_file("inputs/2021/day14-sample.txt");
        assert_eq!("1588", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day14.txt");
        assert_eq!("2874", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day14-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day14.txt");
        assert_eq!("", part2(input));
    }
}
