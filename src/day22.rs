use crate::util;
use crate::aoc;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;

pub struct Day22 {}

impl AOC for Day22 {
    fn part1(&self, input: String, args: Vec<String>) -> String {
        part1(input)
    }

    fn part2(&self, input: String, args: Vec<String>) -> String {
        todo!()
    }
}

pub fn part1(input: String) -> String {
    let (rows, actions) = parse(input);
    //dbg!(&tmp);
    //dbg!(&actions[0]);
    //dbg!(&actions[actions.len() - 1]);

    let num_rows = rows.len();
    let mut cr = 0;
    let mut cc = rows[0].first_col;
    let mut cf = Facing::R;

    'la: for a in actions {
        let first_col = rows[cr].first_col;
        let last_col = rows[cr].last_col;
        let row = &rows[cr].row;

        match a {
            Action::R | Action::L => {
                cf = cf.rotate(&a);
            }
            Action::Move(qt) => {
                // println!("move {qt}, r = {cr}, c = {cc}, face = {:?}", &cf);
                for _ in 0..qt {
                    match cf {
                        Facing::R => {
                            if cc + 1 > last_col {
                                // wrap around if first col is not a wall
                                if row[first_col] == '#' {
                                    continue 'la;
                                } else {
                                    cc = first_col;
                                }
                            } else if row[cc + 1] == '#' {
                                continue 'la;
                            } else {
                                cc += 1;
                            }
                        }
                        Facing::L => {
                            if cc == 0 || cc - 1 < first_col {
                                // wrap around if last col is not a wall
                                if row[last_col] == '#' {
                                    continue 'la;
                                } else {
                                    cc = last_col;
                                }
                            } else if row[cc - 1] == '#' {
                                continue 'la;
                            } else {
                                cc -= 1;
                            }
                        }
                        Facing::D => {
                            if cr + 1 == num_rows
                                || rows[cr + 1].first_col > cc
                                || rows[cr + 1].last_col < cc
                            {
                                // wrap around if next row/col is not a wall
                                let mut next_row = 0;
                                while rows[next_row].first_col > cc || rows[next_row].last_col < cc
                                {
                                    next_row += 1;
                                }
                                if rows[next_row].row[cc] == '#' || next_row == cr {
                                    continue 'la;
                                } else {
                                    cr = next_row;
                                }
                            } else if rows[cr + 1].row[cc] == '#' {
                                continue 'la;
                            } else {
                                cr += 1;
                            }
                        }
                        Facing::U => {
                            if cr == 0 || rows[cr - 1].first_col > cc || rows[cr - 1].last_col < cc
                            {
                                // wrap around if next row/col is not a wall
                                let mut next_row = num_rows - 1;
                                while rows[next_row].first_col > cc || rows[next_row].last_col < cc
                                {
                                    next_row -= 1;
                                }
                                if rows[next_row].row[cc] == '#' || next_row == cr {
                                    continue 'la;
                                } else {
                                    cr = next_row;
                                }
                            } else if rows[cr - 1].row[cc] == '#' {
                                continue 'la;
                            } else {
                                cr -= 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("r = {cr}, c = {cc}, face = {:?}", &cf);
    let ans = 1000 * (cr + 1) + 4 * (cc + 1) + cf as usize;

    ans.to_string()
}

pub fn part2(input: String) -> String {
    todo!()
}

#[derive(Debug)]
struct Row {
    first_col: usize,
    last_col: usize,
    row: Vec<char>,
}

impl Row {
    fn new(first_col: usize, row: Vec<char>) -> Self {
        Self {
            first_col,
            last_col: row.len() - 1,
            row,
        }
    }
}

#[derive(Debug)]
enum Facing {
    R,
    D,
    L,
    U,
}

impl Facing {
    fn rotate(&self, turn: &Action) -> Facing {
        use Facing::*;
        match (self, turn) {
            (R, Action::R) => D,
            (R, Action::L) => U,
            (L, Action::L) => D,
            (L, Action::R) => U,
            (D, Action::L) => R,
            (D, Action::R) => L,
            (U, Action::L) => L,
            (U, Action::R) => R,
            _ => panic!("{self:?}, {turn:?}"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    Move(usize),
    R,
    L,
}

fn parse_actions(line: &str) -> Vec<Action> {
    let mut actions = vec![];
    let mut num = 0;

    for c in line.chars() {
        if c == 'R' || c == 'L' {
            if num > 0 {
                actions.push(Action::Move(num));
                num = 0;
            }
            if c == 'R' {
                actions.push(Action::R);
            } else {
                actions.push(Action::L);
            }
        } else {
            num = num * 10 + c.to_digit(10).unwrap() as usize;
        }
    }
    if num > 0 {
        actions.push(Action::Move(num));
    }

    actions
}

fn parse(input: String) -> (Vec<Row>, Vec<Action>) {
    let mut rows: Vec<Row> = vec![];
    let mut actions: Vec<Action> = vec![];

    let mut handle_actions = false;

    for line in input.lines() {
        if handle_actions {
            actions = parse_actions(line);

            break;
        }
        let row: Vec<char> = line.chars().collect();
        if row.is_empty() {
            handle_actions = true;
            continue;
        }
        // find pos of first non-empty space
        let first_col = row.iter().position(|c| *c != ' ').unwrap_or(0);
        rows.push(Row::new(first_col, row));
    }

    (rows, actions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/day22-sample.txt");
        assert_eq!("6032", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/day22.txt");
        assert_eq!("95358", part1(input));
    }

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day22-sample.txt");
    //    assert_eq!("", part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day22.txt");
    //    assert_eq!("", part2(input));
    //}
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}
