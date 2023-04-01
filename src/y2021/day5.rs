use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;


#[derive(Debug)]
struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn new(y: usize, x: usize) -> Self {
        Self {
            y,
            x
        }
    }

    fn from_str(s: &str) -> Self {
        let tmp = s.split_once(',').unwrap();
        Self {
            x: tmp.0.parse().unwrap(),
            y: tmp.1.parse().unwrap(),
        }
    }

    fn is_diagonal(&self, other: &Pos) -> bool {
        let min_x = self.x.min(other.x);
        let min_y = self.y.min(other.y);
        let max_x = self.x.max(other.x);
        let max_y = self.y.max(other.y);

        max_x - min_x == max_y - min_y
    }
}

fn draw_lines(grid: &mut Vec<Vec<u32>>, lines_vent: &[(Pos, Pos)]) {
    for lv in lines_vent {
        let min_x = lv.0.x.min(lv.1.x);
        let min_y = lv.0.y.min(lv.1.y);
        let max_x = lv.0.x.max(lv.1.x);
        let max_y = lv.0.y.max(lv.1.y);

        if lv.0.is_diagonal(&lv.1) {
            let left_to_right = lv.0.x <= lv.1.x;
            let top_to_bottom = lv.0.y <= lv.1.y;
            let qt = max_x - min_x + 1;
            let mut x = lv.0.x;
            let mut y = lv.0.y;
            for i in 0..qt {
                grid[y][x] += 1;
                if left_to_right { x += 1; }
                else {
                    if x == 0 { break; }
                    x -= 1;
                }
                if top_to_bottom { y += 1; }
                else {
                    if y == 0 { break; }
                    y -= 1;
                }
            }
        } else {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    grid[y][x] += 1;
                }
            }
        }
    }
}

fn part1(input: String) -> String {
    let mut lines_vent = parse(&input);
    lines_vent = lines_vent.into_iter()
            .filter(|lv| lv.0.x == lv.1.x || lv.0.y == lv.1.y).collect();
    let max_x = lines_vent.iter().map(|lv| lv.0.x.max(lv.1.x)).max().unwrap();
    let max_y = lines_vent.iter().map(|lv| lv.0.y.max(lv.1.y)).max().unwrap();
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
    let mut lines_vent = parse(&input);
    let lines_vent: Vec<(Pos, Pos)> = lines_vent.into_iter()
            .filter(|lv| lv.0.x == lv.1.x || lv.0.y == lv.1.y ||
                    lv.0.is_diagonal(&lv.1)).collect();
    let max_x = lines_vent.iter().map(|lv| lv.0.x.max(lv.1.x)).max().unwrap();
    let max_y = lines_vent.iter().map(|lv| lv.0.y.max(lv.1.y)).max().unwrap();
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

fn parse(input: &str) -> Vec<(Pos, Pos)> {
    let mut lines_vent: Vec<(Pos, Pos)> = vec![];
    for line in input.lines() {
        let tmp = line.split_once(" -> ").unwrap();
        lines_vent.push((Pos::from_str(tmp.0), Pos::from_str(tmp.1)));
    }
    lines_vent
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

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2021/day5-sample.txt");
        assert_eq!("12", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day5.txt");
        assert_eq!("20299", part2(input));
    }
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
