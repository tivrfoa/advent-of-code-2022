use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    grid: Vec<Vec<Vec<char>>>,
    minutes: u32,
    pos: (usize, usize),
}

impl State {
    fn new(grid: Vec<Vec<Vec<char>>>, minutes: u32, pos: (usize, usize)) -> Self {
        Self {
            grid,
            minutes,
            pos,
        }
    }

    fn move_blizzards(&mut self) {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let mut new_grid = vec![vec![vec![]; cols]; rows];

        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                for c in &self.grid[row][col] {
                    match c {
                        '.' => {
                            break;
                        }
                        '<' => {
                            if col == 1 {
                                new_grid[row][cols - 2].push('<');
                            } else {
                                new_grid[row][col - 1].push('<');
                            }
                        }
                        '>' => {
                            if col == cols - 2 {
                                new_grid[row][1].push('>');
                            } else {
                                new_grid[row][col + 1].push('>');
                            }
                        }
                        '^' => {
                            if row == 1 {
                                new_grid[rows - 2][col].push('^');
                            } else {
                                new_grid[row - 1][col].push('^');
                            }
                        }
                        'v' => {
                            if row == rows - 2 {
                                new_grid[1][col].push('v');
                            } else {
                                new_grid[row + 1][col].push('v');
                            }
                        }
                        _ => panic!("{c}"),
                    }
                }
            }
        }

        // fill empty pos with '.'
        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                if new_grid[row][col].is_empty() {
                    new_grid[row][col].push('.');
                }
            }
        }

        self.grid = new_grid;
    }

    fn draw(&self) {
        for r in 1..self.grid.len() - 1 {
            for c in 1..self.grid[0].len() - 1 {
                if self.grid[r][c].len() > 1 {
                    print!("{}", self.grid[r][c].len());
                } else {
                    print!("{}", self.grid[r][c][0]);
                }
            }
            println!();
        }
    }

    fn move_right(&self) -> Option<Self> {
        let (r, c) = self.pos;
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        if c < cols - 2 && !self.position_contain_blizzard(r, c + 1) {
            let new_state = Self::new(self.grid.clone(), self.minutes + 1, (r, c + 1));
            return Some(new_state);
        }
        None
    }

    fn move_left(&self) -> Option<Self> {
        let (r, c) = self.pos;
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        if c > 1 && !self.position_contain_blizzard(r, c - 1) {
            let new_state = Self::new(self.grid.clone(), self.minutes + 1, (r, c - 1));
            return Some(new_state);
        }
        None
    }

    fn move_up(&self) -> Option<Self> {
        let (r, c) = self.pos;
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        if r > 1 && !self.position_contain_blizzard(r - 1, c) {
            let new_state = Self::new(self.grid.clone(), self.minutes + 1, (r - 1, c));
            return Some(new_state);
        }
        None
    }

    fn move_down(&self) -> Option<Self> {
        let (r, c) = self.pos;
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        if r < rows - 2 && !self.position_contain_blizzard(r + 1, c) {
            let new_state = Self::new(self.grid.clone(), self.minutes + 1, (r + 1, c));
            return Some(new_state);
        }
        None
    }

    fn position_contain_blizzard(&self, r: usize, c: usize) -> bool {
        for p in &self.grid[r][c] {
            match p {
                '<' | '>' |'^' | 'v' => return true,
                _ => continue,
            }
        }
        false
    }
}

/*

Maybe encode directions in chars if it uses too much memory ...

4: blizard in all directions

<>: A
<^: B
<v: C
>^: D
>v: E
^: F
v
<>^: A


*/

fn draw(grid: &[Vec<Vec<char>>]) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c].len() > 1 {
                print!("{}", grid[r][c].len());
            } else {
                print!("{}", grid[r][c][0]);
            }
        }
        println!();
    }
}

/*

The last column does not contain up and down, so there's no danger of
meeting a blizzard when exiting.
*/

fn part1(input: String) -> String {
    // let mut min_minutes = u32::MAX;
    let mut min_minutes = 1000;
    let grid = parse(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let initial_pos = (0, 1);
    let last_pos = (grid.len() - 2, grid[0].len() - 2); // row, col
    dbg!(&last_pos);

    let mut initial_state = State::new(grid, 0, (0, 1));
    initial_state.move_blizzards();
    initial_state.minutes += 1;
    initial_state.pos = (1, 1);

    let mut visited: HashSet<State> = HashSet::new();
    visited.insert(initial_state.clone());

    let mut states: VecDeque<State> = VecDeque::new();
    states.push_front(initial_state);

    while let Some(mut state) = states.pop_front() {
        // println!("min {}", state.minutes);
        if states.len() > 20000 {
            dbg!(state.minutes);
            state.draw();
            return "".into();
        }
        //dbg!(state.minutes);
        //state.draw();
        if state.pos == last_pos {
            if state.minutes < min_minutes {
                println!("best min is now: {}", state.minutes);
                min_minutes = state.minutes;
            }
            continue;
        }

        if state.minutes == min_minutes {
            continue;
        }

        // move blizzards, then check where we can go
        state.move_blizzards();

        if let Some(s) = state.move_right() {
            if !visited.contains(&s) {
                visited.insert(s.clone());
                states.push_front(s);
            }
        }

        if let Some(s) = state.move_left() {
            if !visited.contains(&s) {
                visited.insert(s.clone());
                states.push_front(s);
            }
        }

        if let Some(s) = state.move_up() {
            if !visited.contains(&s) {
                visited.insert(s.clone());
                states.push_front(s);
            }
        }

        if let Some(s) = state.move_down() {
            if !visited.contains(&s) {
                visited.insert(s.clone());
                states.push_front(s);
            }
        }

        // wait
        if !state.position_contain_blizzard(state.pos.0, state.pos.1) {
            state.minutes += 1;
            if !visited.contains(&state) {
                visited.insert(state.clone());
                states.push_front(state);
            }
        }
    }

    (min_minutes + 1).to_string()
}

fn part2(input: String) -> String {
    todo!()
}

fn parse(input: String) -> Vec<Vec<Vec<char>>> {
    let mut grid: Vec<Vec<Vec<char>>> = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(vec![c]);
        }
        grid.push(row);
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/day24-sample2.txt");
        assert_eq!("18", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/day24.txt");
        assert_eq!("", part1(input));
    }

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day24-sample.txt");
    //    assert_eq!("", part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day24.txt");
    //    assert_eq!("", part2(input));
    //}
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

pub struct Day24 {}

impl AOC for Day24 {
    fn part1(&self, input: Option<String>, args: Vec<String>) -> String {
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/day24.txt"),
            //None => util::read_file("inputs/day24-sample.txt"),
            //None => util::read_file("inputs/day24-sample2.txt"),
        };
        part1(input)
    }

    fn part2(&self, input: Option<String>, args: Vec<String>) -> String {
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/day24.txt"),
        };
        part2(input)
    }
}
