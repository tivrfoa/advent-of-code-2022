use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

#[derive(Debug)]
struct CubeFace {
    grid: Vec<Vec<char>>,
}

impl CubeFace {
    fn parse_cube_faces(mut rows: Vec<Row>) -> [Self; 6] {
        let mut cube_faces = [
            CubeFace::default(),
            CubeFace::default(),
            CubeFace::default(),
            CubeFace::default(),
            CubeFace::default(),
            CubeFace::default(),
        ];

        // 2 and 3
        for i in 0..50 {
            cube_faces[2].grid.push(rows[i].row.split_off(100));
            cube_faces[1].grid.push(rows[i].row.split_off(50));
        }

        // 1
        for i in 50..100 {
            cube_faces[0].grid.push(rows[i].row.split_off(50));
        }

        // 4 and 5
        for i in 100..150 {
            cube_faces[4].grid.push(rows[i].row.split_off(50));
            cube_faces[3].grid.push(rows[i].row.clone());
        }

        // 6
        for i in 150..200 {
            cube_faces[5].grid.push(rows[i].row.clone());
        }

        cube_faces
    }

    fn default() -> Self {
        Self {
            grid: vec![],
        }
    }
}

/*

0 move up    -> 1 bottom (0 top left  -> 1 bottom left)
0 move right -> 2 bottom (0 top right -> 2 bottom left)
0 move down  -> 4 top    (0 bot left  -> 4 top    left)



*/
fn handle_wrap

pub fn part2(input: String) -> String {
    let (rows, actions) = parse(input);

    // I'll hard code the cube faces :(
    let cube: [CubeFace; 6] = CubeFace::parse_cube_faces(rows.clone());
    /*for c in cube {
        for r in c.grid {
            for c in r {
                print!("{c}");
            }
            println!();
        }
    }*/

    let mut cf = 0;
    let mut cr = 0;
    let mut cc = 0;
    let mut dir = Dir::R;

    'la: for a in actions {
        let first_col = 0;
        let last_col  = 50;
        let row = &cube[cf].grid[cr];

        match a {
            Action::R | Action::L => {
                dir = dir.rotate(&a);
            }
            Action::Move(qt) => {
                // println!("move {qt}, r = {cr}, c = {cc}, face = {:?}", &dir);
                for _ in 0..qt {
                    match dir {
                        Dir::R => {
                            if cc + 1 > last_col {
                                handle_wrap(&cube, cf, &dir);
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
                        Dir::L => {
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
                        Dir::D => {
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
                        Dir::U => {
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

    println!("r = {cr}, c = {cc}, face = {:?}", &dir);
    let ans = 1000 * (cr + 1) + 4 * (cc + 1) + dir as usize;

    ans.to_string()
}

fn wrap_cube_face() {
}

#[derive(Clone, Debug)]
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
enum Dir {
    R,
    D,
    L,
    U,
}

impl Dir {
    fn rotate(&self, turn: &Action) -> Dir {
        use Dir::*;
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
    fn part2_sample() {
        let input = util::read_file("inputs/2022/day22-sample.txt");
        assert_eq!("", part2(input));
    }

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/2022/day22.txt");
    //    assert_eq!("", part2(input));
    //}
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}
