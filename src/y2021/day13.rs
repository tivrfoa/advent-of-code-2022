use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

fn part1(input: String) -> String {
    let mut lines = input.lines();
    let mut tmp_pos: Vec<(usize, usize)> = vec![];
    let mut fold_instructions: Vec<(char, usize)> = vec![];
    let mut max_x = 0;
    let mut max_y = 0;

    let mut parse_fold = false;
    for line in lines {
        if line.is_empty() {
            parse_fold = true;
            continue;
        }

        if parse_fold {
            let tmp: Vec<_> = line.split_ascii_whitespace().collect();
            let (l, v) = tmp[2].split_once('=').unwrap();
            let l = l.chars().next().unwrap();
            let v = v.parse::<usize>().unwrap();
            fold_instructions.push((l, v));
        } else {
            let tmp = line.split_once(',').unwrap();
            let x: usize = tmp.0.parse().unwrap();
            let y: usize = tmp.1.parse().unwrap();
            tmp_pos.push((y, x));

            if x > max_x { max_x = x; }
            if y > max_y { max_y = y; }
        }
    }

    let rows = max_y + 1;
    let cols = max_x + 1;

    let mut g: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
    for (r, c) in tmp_pos {
        g[r][c] = '#';
    }

    let (l, v) = fold_instructions[0];

    if l == 'y' {
        let len = g.len() - v - 1;

        for (r1, r2) in (v - len..v).zip((v + 1..g.len()).rev()) {
            for c in 0..g[0].len() {
                if g[r2][c] == '#' {
                    g[r1][c] = '#'
                }
            }
        }

        g.split_off(v);
    } else {
        let len = g[0].len() - v - 1;

        for (c1, c2) in (v - len..v).zip((v + 1..g[0].len()).rev()) {
            for r in 0..g.len() {
                if g[r][c2] == '#' {
                    g[r][c1] = '#'
                }
            }
        }

        for r in g.iter_mut() {
            r.split_off(v);
        }
    }

    let mut ans = 0;
    for r in g {
        for v in r {
            if v == '#' {
                ans += 1;
            }
        }
    }

    ans.to_string()
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
        let input = util::read_file("inputs/2021/day13-sample.txt");
        assert_eq!("17", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day13.txt");
        assert_eq!("716", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day13-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day13.txt");
        assert_eq!("", part2(input));
    }
}
