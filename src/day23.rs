use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;

const DIRS: [char; 4] = ['N', 'S', 'W', 'E'];

fn get_new_pos(
    grid: &[Vec<char>],
    mut dir: usize,
    row: usize,
    col: usize,
) -> Option<(usize, usize)> {
    assert!(row > 0);

    // check if it's alone
    if grid[row - 1][col - 1] == '.'
        && grid[row - 1][col] == '.'
        && grid[row - 1][col + 1] == '.'
        && grid[row + 1][col - 1] == '.'
        && grid[row + 1][col] == '.'
        && grid[row + 1][col + 1] == '.'
        && grid[row][col - 1] == '.'
        && grid[row][col + 1] == '.'
    {
        return None;
    }

    for _ in 0..4 {
        match DIRS[dir] {
            'N' => {
                if grid[row - 1][col - 1] == '.'
                    && grid[row - 1][col] == '.'
                    && grid[row - 1][col + 1] == '.'
                {
                    return Some((row - 1, col));
                }
            }
            'S' => {
                if grid[row + 1][col - 1] == '.'
                    && grid[row + 1][col] == '.'
                    && grid[row + 1][col + 1] == '.'
                {
                    return Some((row + 1, col));
                }
            }
            'W' => {
                if grid[row - 1][col - 1] == '.'
                    && grid[row][col - 1] == '.'
                    && grid[row + 1][col - 1] == '.'
                {
                    return Some((row, col - 1));
                }
            }
            'E' => {
                if grid[row - 1][col + 1] == '.'
                    && grid[row][col + 1] == '.'
                    && grid[row + 1][col + 1] == '.'
                {
                    return Some((row, col + 1));
                }
            }
            _ => panic!("{dir}"),
        }
        dir = (dir + 1) % 4;
    }

    None
}

fn part1(input: String) -> String {
    let mut grid = parse(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dir = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            print!("{}", grid[r][c]);
        }
        println!();
    }

    for _ in 0..10 {
        let mut moves: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
        for r in 1..rows {
            for c in 1..cols {
                if grid[r][c] == '#' {
                    if let Some(pos) = get_new_pos(&grid, dir, r, c) {
                        match moves.get_mut(&pos) {
                            Some(v) => {
                                v.push((r, c));
                            }
                            None => {
                                moves.insert(pos, vec![(r, c)]);
                            }
                        }
                    }
                }
            }
        }

        for (key, value) in moves.into_iter() {
            if value.len() > 1 {
                continue;
            }
            let (oldr, oldc) = value[0];
            let (newr, newc) = key;
            grid[oldr][oldc] = '.';
            grid[newr][newc] = '#';
        }

        dir = (dir + 1) % DIRS.len();
    }

    let mut left_most = 0;
    for r in &grid {
        if let Some(col) = r.iter().position(|c| *c == '#') {
            if left_most == 0 || col < left_most {
                left_most = col;
            }
        }
    }
    let mut right_most = 0;
    for r in &grid {
        if let Some(col) = r.iter().rposition(|c| *c == '#') {
            if col > right_most {
                right_most = col;
            }
        }
    }
    let mut top_most = 0;
    for (r, row) in grid.iter().enumerate() {
        if let Some(_) = row.iter().position(|c| *c == '#') {
            top_most = r;
            break;
        }
    }
    let mut bottom_most = 0;
    for r in (0..grid.len()).rev() {
        if let Some(_) = grid[r].iter().position(|c| *c == '#') {
            bottom_most = r;
            break;
        }
    }

    let mut num_empty_tiles = 0;

    dbg!(top_most, bottom_most, left_most, right_most);
    for r in top_most..=bottom_most {
        for c in left_most..=right_most {
            if grid[r][c] == '.' {
                num_empty_tiles += 1;
            }
        }
    }

    dbg!(grid.len());
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            print!("{}", grid[r][c]);
        }
        println!();
    }

    num_empty_tiles.to_string()
}

fn part2(input: String) -> String {
    todo!()
}

fn parse(input: String) -> Vec<Vec<char>> {
    let mut tmp_grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        tmp_grid.push(line.chars().collect::<Vec<char>>());
    }

    let cols = tmp_grid[0].len() + 20;
    let rows = tmp_grid.len() + 20;
    let mut grid = vec![vec!['.'; cols]; rows];
    for r in 10..rows - 10 {
        for c in 10..cols - 10 {
            grid[r][c] = tmp_grid[r - 10][c - 10];
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/day23-sample.txt");
        assert_eq!("110", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/day23.txt");
        assert_eq!("3882", part1(input));
    }

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day23-sample.txt");
    //    assert_eq!("", part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day23.txt");
    //    assert_eq!("", part2(input));
    //}
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

pub struct Day23 {}

impl AOC for Day23 {
    fn part1(&self, input: Option<String>, args: Vec<String>) -> String {
        let input = match input {
            Some(input) => input,
            // None => util::read_file("inputs/day23.txt"),
            None => util::read_file("inputs/day23-sample.txt"),
        };
        part1(input)
    }

    fn part2(&self, input: Option<String>, args: Vec<String>) -> String {
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/day23.txt"),
        };
        part2(input)
    }
}
