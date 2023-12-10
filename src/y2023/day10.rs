use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

// Start at S
// L, F, J, ., 7, |, -

const PIPES: [char; 6] = ['|', '-', 'L', 'J', '7', 'F'];

fn solve(grid: &[Vec<char>], start_row: usize, start_col: usize) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut r = start_row;
    let mut c = start_col;
    let mut from = (r, c);
    let mut steps = 0;

    loop {
        if !visited.insert((r, c)) {
            return None;
        }
        steps += 1;
        match grid[r][c] {
            '|' => {
                // north
                if r > 0 && (r - 1, c) != from {
                    from = (r, c);
                    r = r - 1;
                    let d = grid[r][c];
                    if d != '|' && d != '7' && d != 'F' {
                        return None;
                    }
                    // south
                } else if r + 1 < rows && (r + 1, c) != from {
                    from = (r, c);
                    r = r + 1;
                    let d = grid[r][c];
                    if d != '|' && d != 'L' && d != 'J' {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            '-' => {
                // west
                if c > 0 && (r, c - 1) != from {
                    from = (r, c);
                    c = c - 1;
                    let d = grid[r][c];
                    if d != '-' && d != 'L' && d != 'F' {
                        return None;
                    }
                    // east
                } else if c + 1 < cols && (r, c + 1) != from {
                    from = (r, c);
                    c = c + 1;
                    let d = grid[r][c];
                    if d != '-' && d != 'J' && d != '7' {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            'L' => {
                // east
                if c + 1 < cols && (r, c + 1) != from {
                    from = (r, c);
                    c += 1;
                    let d = grid[r][c];
                    if d != '-' && d != 'J' && d != '7' {
                        return None;
                    }
                    // north
                } else if r > 0 && (r - 1, c) != from {
                    from = (r, c);
                    r -= 1;
                    let d = grid[r][c];
                    if d != '|' && d != '7' && d != 'F' {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            'J' => {
                // north
                if r > 0 && (r - 1, c) != from {
                    from = (r, c);
                    r -= 1;
                    let d = grid[r][c];
                    if d != '|' && d != '7' && d != 'F' {
                        return None;
                    }
                    // west
                } else if c > 0 && (r, c - 1) != from {
                    from = (r, c);
                    c -= 1;
                    let d = grid[r][c];
                    if d != '-' && d != 'L' && d != 'F' {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            '7' => {
                // south
                if r + 1 < rows && (r + 1, c) != from {
                    from = (r, c);
                    r += 1;
                    let d = grid[r][c];
                    if d != '|' && d != 'L' && d != 'J' {
                        return None;
                    }
                    // west
                } else if c > 0 && (r, c - 1) != from {
                    from = (r, c);
                    c -= 1;
                    let d = grid[r][c];
                    if d != '-' && d != 'L' && d != 'F' {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            'F' => {
                // south
                if r + 1 < rows && (r + 1, c) != from {
                    from = (r, c);
                    r += 1;
                    let d = grid[r][c];
                    if d != '|' && d != 'L' && d != 'J' {
                        return None;
                    }
                    // east
                } else if c + 1 < cols && (r, c + 1) != from {
                    from = (r, c);
                    c += 1;
                    let d = grid[r][c];
                    if d != '-' && d != 'J' && d != '7' {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            _ => panic!("{}", grid[r][c]),
        }

        if r == start_row && c == start_col {
            // found the loop!
            return Some((steps + 1) / 2);
        }
    }
}

pub fn part1(input: &str) -> String {
    // find the loop
    // then find the farthest

    let mut grid = input.to_char_grid();
    let rows = grid.len();
    let cols = grid[0].len();
    let start = {
        let mut S = (0, 0);
        'r:
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 'S' {
                    S = (r, c);
                    break 'r;
                }
            }
        }
        S
    };

    // finding the loop: try each of the possible pipes and check if it's a loop
    for pipe in PIPES {
        grid[start.0][start.1] = pipe;
        if let Some(farthest) = solve(&grid, start.0, start.1) {
            return farthest.to_string();
        }
    }

    "failed".into()
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
        let input = include_str!("../../inputs/2023/day10-sample.txt");
        assert_eq!("4", part1(input));
    }

    #[test]
    fn sample2() {
        let input = include_str!("../../inputs/2023/day10-sample2.txt");
        assert_eq!("8", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day10.txt");
        assert_eq!("6979", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day10-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day10.txt");
        assert_eq!("", part2(input));
    }
}
