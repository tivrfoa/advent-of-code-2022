use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
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
            new_str.push_str(&curr[i..i + 1]);
            new_str.push_str(map.get(&curr[i..i + 2]).unwrap());
        }
        new_str.push_str(&curr[curr.len() - 1..]);
        curr = new_str;
        // println!("After step {}: {}", i, curr);
    }

    let mut qt_map: HashMap<char, u32> = HashMap::new();
    for c in curr.chars() {
        qt_map.entry(c).and_modify(|qt| *qt += 1).or_insert(1);
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

/*

For part2, I copied the solution from here:

2021 advent of code - day 14 solutions
https://www.youtube.com/watch?v=TIL1JwLtIzw
*/
fn part2(input: String) -> String {
    let polymer_template = input.lines().next().unwrap();
    let mut map: HashMap<&str, char> = HashMap::new();

    for line in input.lines().skip(2) {
        let tmp = line.split_once(" -> ").unwrap();
        map.insert(tmp.0, tmp.1.chars().next().unwrap());
    }
    dbg!(&map);

    let mut pair_qt: HashMap<String, u64> = HashMap::new();
    for i in 0..polymer_template.len() - 1 {
        pair_qt
            .entry(polymer_template[i..=i + 1].into())
            .and_modify(|qt| *qt += 1)
            .or_insert(1);
    }
    dbg!(&pair_qt);

    let mut char_qt: HashMap<char, u64> = HashMap::new();

    for _i in 1..=40 {
        let mut new_pair_qt: HashMap<String, u64> = HashMap::with_capacity(pair_qt.len());
        char_qt = HashMap::new();
        for (k, v) in pair_qt {
            let pattern = *map.get(&k[..]).unwrap();
            let mut chars = k.chars();
            let l = chars.next().unwrap();
            let r = chars.next().unwrap();
            let pair_key = format!("{}{}", l, pattern);
            new_pair_qt.entry(pair_key).and_modify(|qt| *qt += v).or_insert(v);
            let pair_key = format!("{}{}", pattern, r);
            new_pair_qt.entry(pair_key).and_modify(|qt| *qt += v).or_insert(v);

            char_qt.entry(l).and_modify(|qt| *qt += v).or_insert(v);
            char_qt.entry(pattern).and_modify(|qt| *qt += v).or_insert(v);
        }
        pair_qt = new_pair_qt;
    }

    let last_char = polymer_template.chars().last().unwrap();
    char_qt.entry(last_char).and_modify(|qt| *qt += 1);

    let mut lc = u64::MAX;
    let mut mc = 0;

    for (_, v) in char_qt {
        if v < lc { lc = v; }
        if v > mc { mc = v; }
    }

    (mc - lc).to_string()
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
fn get_dirs_with_diagonals(
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
) -> [(bool, (usize, usize)); 8] {
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
        assert_eq!("2188189693529", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day14.txt");
        assert_eq!("5208377027195", part2(input));
    }
}
