use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;


#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }

    fn from_str(s: &str) -> Self {
        let tmp = s.split_once(',').unwrap();
        Self {
            x: tmp.0.parse().unwrap(),
            y: tmp.1.parse().unwrap(),
        }
    }
}

fn draw_lines(grid: &mut Vec<Vec<u32>>, lines_vent: &[(Pos, Pos)]) {
    for lv in lines_vent {
        let min_x = lv.0.x.min(lv.1.x);
        let min_y = lv.0.y.min(lv.1.y);
        let max_x = lv.0.x.max(lv.1.x);
        let max_y = lv.0.y.max(lv.1.y);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                grid[y][x] += 1;
            }
        }
    }
}

fn part1(input: String) -> String {
    let mut lines_vent: Vec<(Pos, Pos)> = vec![];
    for line in input.lines() {
        let tmp = line.split_once(" -> ").unwrap();
        lines_vent.push((Pos::from_str(tmp.0), Pos::from_str(tmp.1)));
    }

    lines_vent = lines_vent.into_iter()
            .filter(|lv| lv.0.x == lv.1.x || lv.0.y == lv.1.y).collect();
    let max_x = lines_vent.iter().map(|lv| lv.0.x.max(lv.1.x)).max().unwrap();
    let max_y = lines_vent.iter().map(|lv| lv.0.y.max(lv.1.y)).max().unwrap();
    // dbg!(max_x, max_y);
    let mut grid = vec![vec![0; max_x + 1]; max_y + 1];
    draw_lines(&mut grid, &lines_vent);
    let mut ans = 0;
    for r in grid {
        for v in r {
            if v > 1 {
                ans += 1;
            }
        }
    }

    ans.to_string()
}

fn part2(input: String) -> String {
    "".into()
}

fn parse(input: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2021/day5-sample.txt");
        assert_eq!("5", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2021/day5.txt");
        assert_eq!("5608", part1(input));
    }

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/2021/day5-sample.txt");
    //    assert_eq!("", part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/2021/day5.txt");
    //    assert_eq!("", part2(input));
    //}
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

// pub struct Day5 {}
// 
// impl AOC for Day5 {
//     fn part1(&self, input: Option<String>, args: Vec<String>) -> String {
//         let input = match input {
//             Some(input) => input,
//             None => util::read_file("inputs/2021/day5.txt"),
//         };
//         part1(input)
//     }
// 
//     fn part2(&self, input: Option<String>, args: Vec<String>) -> String {
//         let input = match input {
//             Some(input) => input,
//             None => util::read_file("inputs/2021/day5.txt"),
//         };
//         part2(input)
//     }
// }
