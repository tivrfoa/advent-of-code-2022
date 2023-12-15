use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn tilt_north(grid: &mut Vec<Vec<char>>) {
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
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();

    for y in (0..rows - 1).rev() {
        for x in 0..cols {
            if grid[y][x] == 'O' {
                for row in y+1..rows {
                    if grid[row][x] != '.' {
                        break;
                    }
                    grid[row][x] = 'O';
                    grid[row - 1][x] = '.';
                }
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();

    for x in 1..cols {
        for y in 0..rows {
            if grid[y][x] == 'O' {
                for col in (0..x).rev() {
                    if grid[y][col] != '.' {
                        break;
                    }
                    grid[y][col] = 'O';
                    grid[y][col + 1] = '.';
                }
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();

    for x in (0..cols - 1).rev() {
        for y in 0..rows {
            if grid[y][x] == 'O' {
                for col in x+1..cols {
                    if grid[y][col] != '.' {
                        break;
                    }
                    grid[y][col] = 'O';
                    grid[y][col - 1] = '.';
                }
            }
        }
    }
}

fn get_north_load(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total = 0;
    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == 'O' {
                total += rows - y;
            }
        }
    }
    total
}

// let load_single_rounded_rock = rows - rock.row;
pub fn part1(input: &str) -> String {
    let mut grid = input.to_char_grid();
    tilt_north(&mut grid);
    get_north_load(&grid).to_string()
}

pub fn part2(input: &str) -> String {
    let mut grid = input.to_char_grid();
    let mut north_loads: HashSet<usize> = HashSet::new();

    for i in 0..1000 {
    // for i in 0..3 {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        // dbg_grid(&grid);

        let load = get_north_load(&grid);
        if !north_loads.insert(load) {
            println!("Same load at {i} - load = {load}");
        }
    }

    // 1_000_000_000 % cycle_length = pos in cycle length that correponds
    // to the north load

    "0".to_string()
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
