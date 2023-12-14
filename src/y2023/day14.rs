use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn tilt(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();

    for y in 1..rows {
        for x in 0..cols {
            if grid[y][x] == 'O' {
                for row in (0..y).rev() {
                    if grid[row][x] != '.' {
                        break;
                    }
                    grid[row][x] = 'O';
                    grid[row + 1][x] = '.';
                }
            }
        }
    }

    grid
}

// let load_single_rounded_rock = rows - rock.row;
pub fn part1(input: &str) -> String {
    let mut total = 0;

    let grid = input.to_char_grid();
    dbg_grid(&grid);
    let grid = tilt(grid);
    dbg_grid(&grid);
    let rows = grid.len();
    let cols = grid[0].len();

    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == 'O' {
                total += rows - y;
            }
        }
    }

    total.to_string()
}

pub fn part2(input: &str) -> String {
    "".into()
}

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
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
        let input = include_str!("../../inputs/2023/day14-sample.txt");
        assert_eq!("136", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day14.txt");
        assert_eq!("103333", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day14-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day14.txt");
        assert_eq!("", part2(input));
    }
}
