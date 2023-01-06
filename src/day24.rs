use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    grid: Vec<Vec<Vec<char>>>,
    pos: (usize, usize),
}

impl State {
    fn new(grid: Vec<Vec<Vec<char>>>, pos: (usize, usize)) -> Self {
        Self { grid, pos }
    }

    fn undo_move_blizzards(&mut self) {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let mut new_grid = vec![vec![vec![]; cols]; rows];

        for (row, row_item) in self.grid.iter().enumerate().take(rows - 1).skip(1) {
            for (col, col_item) in row_item.iter().enumerate().take(cols - 1).skip(1) {
                for c in col_item {
                    match c {
                        '.' => {
                            break;
                        }
                        '<' => {
                            if col == cols - 2 {
                                new_grid[row][1].push('<');
                            } else {
                                new_grid[row][col + 1].push('<');
                            }
                        }
                        '>' => {
                            if col == 1 {
                                new_grid[row][cols - 2].push('>');
                            } else {
                                new_grid[row][col - 1].push('>');
                            }
                        }
                        '^' => {
                            if row == rows - 2 {
                                new_grid[1][col].push('^');
                            } else {
                                new_grid[row + 1][col].push('^');
                            }
                        }
                        'v' => {
                            if row == 1 {
                                new_grid[rows - 2][col].push('v');
                            } else {
                                new_grid[row - 1][col].push('v');
                            }
                        }
                        _ => panic!("{c}"),
                    }
                }
            }
        }

        // fill empty pos with '.'
        for row in new_grid.iter_mut().take(rows - 1).skip(1) {
            for pos in row.iter_mut().take(cols - 1).skip(1) {
                if pos.is_empty() {
                    pos.push('.');
                }
            }
        }

        self.grid = new_grid;
    }

    fn move_blizzards(&mut self) {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let mut new_grid = vec![vec![vec![]; cols]; rows];

        for (row, row_item) in self.grid.iter().enumerate().take(rows - 1).skip(1) {
            for (col, col_item) in row_item.iter().enumerate().take(cols - 1).skip(1) {
                for c in col_item {
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
        for row in new_grid.iter_mut().take(rows - 1).skip(1) {
            for pos in row.iter_mut().take(cols - 1).skip(1) {
                if pos.is_empty() {
                    pos.push('.');
                }
            }
        }

        self.grid = new_grid;
    }

    fn draw(&self) {
        for r in 1..self.grid.len() - 1 {
            for c in 1..self.grid[0].len() - 1 {
                /*if self.pos.0 == r && self.pos.1 == c {
                    print!("E");
                } else */
                if self.grid[r][c].len() > 1 {
                    print!("{}", self.grid[r][c].len());
                } else {
                    print!("{}", self.grid[r][c][0]);
                }
            }
            println!();
        }
    }

    fn position_contain_blizzard(&self, r: usize, c: usize) -> bool {
        for p in &self.grid[r][c] {
            match p {
                '<' | '>' | '^' | 'v' => return true,
                _ => continue,
            }
        }
        false
    }

    fn get_key(&self) -> Vec<u16> {
        const fn l_to_n(l: char) -> usize {
            match l {
                '<' => 1,
                '>' => 2,
                '^' => 3,
                'v' => 4,
                _ => 10,
            }
        }
        let mut key = vec![];
        for (r, row) in self.grid.iter().enumerate() {
            for (c, p) in row.iter().enumerate() {
                for l in p {
                    match l {
                        '<' | '>' | '^' | 'v' => {
                            key.push((l_to_n(*l) * 10_000 + (r + 1) * 100 + c) as u16);
                        }
                        _ => continue,
                    }
                }
            }
        }
        key.push((self.pos.0 * 100 + self.pos.1) as u16);
        key
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
            } else if grid[r][c].len() == 1 {
                print!("{}", grid[r][c][0]);
            } else {
                print!("#");
            }
        }
        println!();
    }
}

type Pos = (usize, usize);

fn dfs(
    visited: &mut HashMap<Vec<u16>, u32>,
    last_pos: Pos,
    minutes: u32,
    state: &mut State,
    ans: &mut u32,
    rows: usize,
    cols: usize,
    final_grid: &mut Vec<Vec<Vec<char>>>,
) {
    let key = state.get_key();
    match visited.get(&key) {
        Some(m) => {
            if *m <= minutes {
                return;
            }
            visited.insert(key, minutes);
        }
        None => {
            visited.insert(key, minutes);
        }
    }

    if state.pos == last_pos {
        if minutes < *ans {
            println!("best min is now: {}", minutes);
            *ans = minutes;
            *final_grid = state.grid.clone();
            // println!("======== FINAL GRID =======");
            // state.draw();
            //draw(&final_grid);
            //dbg!(final_grid);
            // println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        }
        return;
    }

    if minutes >= *ans {
        // can only get worse, so return
        return;
    }

    // move blizzards, then check where we can go
    state.move_blizzards();

    // let mut you_moved = false;

    // right
    if state.pos.1 < cols - 2 && !state.position_contain_blizzard(state.pos.0, state.pos.1 + 1) {
        // you_moved = true;
        state.pos = (state.pos.0, state.pos.1 + 1);
        dfs(
            visited,
            last_pos,
            minutes + 1,
            state,
            ans,
            rows,
            cols,
            final_grid,
        );
        state.pos = (state.pos.0, state.pos.1 - 1);
    }

    // left
    if state.pos.1 > 1 && !state.position_contain_blizzard(state.pos.0, state.pos.1 - 1) {
        // you_moved = true;
        state.pos = (state.pos.0, state.pos.1 - 1);
        dfs(
            visited,
            last_pos,
            minutes + 1,
            state,
            ans,
            rows,
            cols,
            final_grid,
        );
        state.pos = (state.pos.0, state.pos.1 + 1);
    }

    // up
    if state.pos.0 > 1 && !state.position_contain_blizzard(state.pos.0 - 1, state.pos.1) {
        // you_moved = true;
        state.pos = (state.pos.0 - 1, state.pos.1);
        dfs(
            visited,
            last_pos,
            minutes + 1,
            state,
            ans,
            rows,
            cols,
            final_grid,
        );
        state.pos = (state.pos.0 + 1, state.pos.1);
    }

    // down
    if state.pos.0 < rows - 2 && !state.position_contain_blizzard(state.pos.0 + 1, state.pos.1) {
        // you_moved = true;
        state.pos = (state.pos.0 + 1, state.pos.1);
        dfs(
            visited,
            last_pos,
            minutes + 1,
            state,
            ans,
            rows,
            cols,
            final_grid,
        );
        state.pos = (state.pos.0 - 1, state.pos.1);
    }

    // wait
    if !state.position_contain_blizzard(state.pos.0, state.pos.1) {
        // you_moved = true;
        dfs(
            visited,
            last_pos,
            minutes + 1,
            state,
            ans,
            rows,
            cols,
            final_grid,
        );
    }

    // if !you_moved {
    //     println!("You got killed! ({}, {}), minutes = {minutes}", state.pos.0, state.pos.1);
    //     state.draw();
    // }

    state.undo_move_blizzards();
}

const MAX_MINUTES: u32 = 270;

fn part1(input: String) -> String {
    let mut min_minutes = MAX_MINUTES; //
    let grid = parse(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let initial_pos = (0, 1);
    let last_pos = (grid.len() - 2, grid[0].len() - 2); // row, col
                                                        // dbg!(&last_pos);

    let mut initial_state = State::new(grid, initial_pos);
    initial_state.move_blizzards();
    // Enter grid
    initial_state.pos = (1, 1);

    let mut final_grid: Vec<Vec<Vec<char>>> = vec![];
    let mut visited: HashMap<Vec<u16>, u32> = HashMap::new();

    dfs(
        &mut visited,
        last_pos,
        1,
        &mut initial_state,
        &mut min_minutes,
        rows,
        cols,
        &mut final_grid,
    );

    (min_minutes + 1).to_string()
}

/// initial and final position "inside" the grid
fn solve(
    grid: Vec<Vec<Vec<char>>>,
    initial_pos: Pos,
    final_pos: Pos,
) -> (u32, Vec<Vec<Vec<char>>>) {
    println!("Trying to get from {:?} to {:?}", initial_pos, final_pos);
    let mut min_minutes = MAX_MINUTES;
    let mut final_grid: Vec<Vec<Vec<char>>> = vec![];

    let mut minutes = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    let mut initial_state = State::new(grid, initial_pos);

    println!("=========== INITIAL STATE =============");

    while minutes < 15 {
        initial_state.draw();
        minutes += 1;
        initial_state.move_blizzards();

        while initial_state.position_contain_blizzard(initial_state.pos.0, initial_state.pos.1) {
            println!("It can't enter the grid. minutes: {minutes}");
            // initial_state.draw();
            minutes += 1;
            initial_state.move_blizzards();
        }

        let mut visited: HashMap<Vec<u16>, u32> = HashMap::new();
        // println!("------grid before dfs-------");
        // initial_state.draw();
        dfs(
            &mut visited,
            final_pos,
            minutes,
            &mut initial_state,
            &mut min_minutes,
            rows,
            cols,
            &mut final_grid,
        );

        if !final_grid.is_empty() {
            break;
        }

        println!("It didn't find a solution. Let's try again.");
    }

    if final_grid.is_empty() {
        panic!("Mission Failed");
    }

    println!("--->> debugging final grid");
    draw(&final_grid);

    // before return, we need to move blizzards in the final grid one more time
    initial_state.grid = final_grid;
    initial_state.move_blizzards();

    (min_minutes + 1, initial_state.grid)
}

fn part2(input: String) -> String {
    let mut sum = 0;
    let grid = parse(input);
    let initial_pos = (1, 1);
    // row, col
    let last_pos = (grid.len() - 2, grid[0].len() - 2);

    // go
    let (minutes, grid) = solve(grid, initial_pos, last_pos);
    sum += minutes;

    // go back
    let (minutes, grid) = solve(grid, last_pos, (1, 1));
    sum += minutes;

    // go again
    let (minutes, _) = solve(grid, initial_pos, last_pos);
    sum += minutes;

    sum.to_string()
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
        assert_eq!("240", part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/day24-sample2.txt");
        assert_eq!("54", part2(input));
    }

    #[test]
    fn part2_input() {
       let input = util::read_file("inputs/day24.txt");
       assert_eq!("717", part2(input));
    }
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
        println!(
            "sample answer: {}",
            part1(util::read_file("inputs/day24-sample2.txt"))
        );
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/day24.txt"),
        };
        part1(input)
    }

    fn part2(&self, input: Option<String>, args: Vec<String>) -> String {
        println!(
            "sample answer: {}",
            part2(util::read_file("inputs/day24-sample2.txt"))
        );
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/day24.txt"),
        };
        part2(input)
    }
}
