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

type Pos = (usize, usize);

pub fn part2(input: &str) -> String {
    // lp: mark positions that belong to the main loop
    // for every other position, try to make it go outside without
    // passing through loop positions (lp)

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
        if let Some(lp) = solve_p2(&grid, start.0, start.1) {
            //for (r, c) in &lp {
            //    grid[*r][*c] = 'T';
            //}
            let qt = count_enclosed(&mut grid, lp).to_string();
            dbg_grid(&grid);
            return qt;
        }
    }

    "failed".into()
}

fn count_enclosed(grid: &mut Vec<Vec<char>>, lp: Vec<Pos>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut enclosed: HashSet<Pos> = HashSet::new();
    let mut escaped: HashSet<Pos> = HashSet::new();
    let mut qt = 0;

    for r in 0..rows {
        for c in 0..cols {
            if lp.contains(&(r, c)) { continue; }
            let mut visited: HashSet<Pos> = HashSet::new();
            if !can_go_outside(grid, &lp, r, c, &mut visited, &mut enclosed, &mut escaped) {
                qt += 1;
                enclosed.insert((r, c));
                // grid[r][c] = 'I';
            } else {
                // grid[r][c] = 'O';
            }
        }
    }

    for (r, c) in enclosed {
        grid[r][c] = 'I';
    }

    for (r, c) in escaped {
        grid[r][c] = 'O';
    }

    qt
}

fn can_go_outside(grid: &[Vec<char>], lp: &[Pos], r: usize, c: usize,
        visited: &mut HashSet<Pos>,
        enclosed: &mut HashSet<Pos>,
        escaped: &mut HashSet<Pos>) -> bool {
    if escaped.contains(&(r, c)) { return true; }
    if enclosed.contains(&(r, c)) { return false; }
    if lp.contains(&(r, c)) { return false; }
    let rows = grid.len();
    let cols = grid[0].len();
    if r == 0 || r + 1 == rows || c == 0 || c + 1 == cols {
        escaped.insert((r, c));
        return true;
    }
    if !visited.insert((r, c)) { return false; }

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy != 0 || dx != 0 {
                let mut r2 = (r as i32 + dy) as usize;
                let mut c2 = (c as i32 + dx) as usize;
                if lp.contains(&(r2, c2)) {
                    // check if it can squeeze ...
                    // JL, 7L, ||, 7F, JF, =, ...
                    // basically need to check if it can go to its neighbors
                    // this implementation supposes squeeze does NOT make
                    // turns ...

                    if dy != 0 && dx != 0 {
                        continue;
                    }

                    // vertical squeeze
                    if dx == 0 {
                        loop {
                            let d = grid[r2][c2];
                            if d == '7' || d == '|' || d == 'J' {
                                let right = grid[r2][c2 + 1];
                                // if right == '.' {
                                if !lp.contains(&(r2, c2+1)) {
                                    if can_go_outside(grid, lp, r2, c2 + 1, visited, enclosed, escaped) {
                                        escaped.insert((r, c));
                                        return true;
                                    } else {
                                        break;
                                    }
                                }
                                if right == '|' || right == 'L' || right == 'F' {
                                    if (dy == 1 && r2 + 1 == rows) || r2 == 0 {
                                        escaped.insert((r, c));
                                        return true;
                                    }
                                    r2 = (r2 as i32 + dy) as usize;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    // horizontal squeeze
                    if dy == 0 {
                        loop {
                            let d = grid[r2][c2];
                            if d == '-' || d == 'L' || d == 'J' {
                                if !lp.contains(&(r2 + 1, c2)) {
                                    if can_go_outside(grid, lp, r2 + 1, c2, visited, enclosed, escaped) {
                                        escaped.insert((r, c));
                                        return true;
                                    } else {
                                        break;
                                    }
                                }
                                let below = grid[r2 + 1][c2];
                                if below == '-' || below == '7' || below == 'F' {
                                    if (dx == 1 && c2 + 1 == cols) || c2 == 0 {
                                        escaped.insert((r, c));
                                        return true;
                                    }
                                    c2 = (c2 as i32 + dx) as usize;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                } else {
                    if can_go_outside(grid, lp,
                            r2,
                            c2,
                            visited,
                            enclosed,
                            escaped) {
                        escaped.insert((r, c));
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn solve_p2(grid: &[Vec<char>], start_row: usize, start_col: usize) -> Option<Vec<(usize, usize)>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut r = start_row;
    let mut c = start_col;
    let mut from = (r, c);
    let mut lp = vec![];

    loop {
        if !visited.insert((r, c)) {
            return None;
        }
        lp.push((r, c));

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
            return Some(lp);
        }
    }
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
    fn p200() {
        let input = include_str!("../../inputs/2023/day10-sample-p2-00.txt");
        assert_eq!("8", part2(input));
    }

    #[test]
    fn p202() {
        let input = include_str!("../../inputs/2023/day10-sample-p2-02.txt");
        assert_eq!("4", part2(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day10-sample-p2-01.txt");
        assert_eq!("10", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day10.txt");
        assert_eq!("", part2(input));
    }
}
