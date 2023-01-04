use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;

/*

The last column does not contain up and down, so there's no danger of
meeting a blizzard when exiting.
*/

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    grid: Vec<Vec<Vec<char>>>,
    pos: (usize, usize),
}

impl State {
    fn new(grid: Vec<Vec<Vec<char>>>, pos: (usize, usize)) -> Self {
        Self {
            grid,
            pos,
        }
    }

    fn undo_move_blizzards(&mut self) {
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
        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                if new_grid[row][col].is_empty() {
                    new_grid[row][col].push('.');
                }
            }
        }

        self.grid = new_grid;
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
                if self.pos.0 == r && self.pos.1 == c {
                    print!("E");
                } else if self.grid[r][c].len() > 1 {
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
            let new_state = Self::new(self.grid.clone(), (r, c + 1));
            return Some(new_state);
        }
        None
    }

    fn move_left(&self) -> Option<Self> {
        let (r, c) = self.pos;
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        if c > 1 && !self.position_contain_blizzard(r, c - 1) {
            let new_state = Self::new(self.grid.clone(), (r, c - 1));
            return Some(new_state);
        }
        None
    }

    fn move_up(&self) -> Option<Self> {
        let (r, c) = self.pos;
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        if r > 1 && !self.position_contain_blizzard(r - 1, c) {
            let new_state = Self::new(self.grid.clone(), (r - 1, c));
            return Some(new_state);
        }
        None
    }

    fn move_down(&self) -> Option<Self> {
        let (r, c) = self.pos;
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        if r < rows - 2 && !self.position_contain_blizzard(r + 1, c) {
            let new_state = Self::new(self.grid.clone(), (r + 1, c));
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

type Pos = (usize, usize);

fn dfs(visited: &mut HashMap<State, u32>, last_pos: Pos,
        minutes: u32, state: &mut State, ans: &mut u32,
        rows: usize, cols: usize) {
    match visited.get(&state) {
        Some(m) => {
            if *m <= minutes {
                return;
            }
            visited.insert(state.clone(), minutes);
        },
        None => {
            visited.insert(state.clone(), minutes);
        }
    }

    if state.pos == last_pos {
        if minutes < *ans {
            println!("best min is now: {}", minutes);
            *ans = minutes;
        }
        return;
    }

    if minutes == *ans {
        // can only get worse, so return
        return;
    }

    // move blizzards, then check where we can go
    state.move_blizzards();

    // right
    if state.pos.1 < cols - 2 && !state.position_contain_blizzard(state.pos.0, state.pos.1 + 1) {
        state.pos = (state.pos.0, state.pos.1 + 1);
        dfs(visited, last_pos, minutes + 1, state, ans, rows, cols);
        state.pos = (state.pos.0, state.pos.1 - 1);
    }

    // left
    if state.pos.1 > 1 && !state.position_contain_blizzard(state.pos.0, state.pos.1 - 1) {
        state.pos = (state.pos.0, state.pos.1 - 1);
        dfs(visited, last_pos, minutes + 1, state, ans, rows, cols);
        state.pos = (state.pos.0, state.pos.1 + 1);
    }

    // up
    if state.pos.0 > 1 && !state.position_contain_blizzard(state.pos.0 - 1, state.pos.1) {
        state.pos = (state.pos.0 - 1, state.pos.1);
        dfs(visited, last_pos, minutes + 1, state, ans, rows, cols);
        state.pos = (state.pos.0 + 1, state.pos.1);
    }

    // down
    if state.pos.0 < rows - 2 && !state.position_contain_blizzard(state.pos.0 + 1, state.pos.1) {
        state.pos = (state.pos.0 + 1, state.pos.1);
        dfs(visited, last_pos, minutes + 1, state, ans, rows, cols);
        state.pos = (state.pos.0 - 1, state.pos.1);
    }

    // wait
    if !state.position_contain_blizzard(state.pos.0, state.pos.1) {
        dfs(visited, last_pos, minutes + 1, state, ans, rows, cols);
    }

    state.undo_move_blizzards();
}

fn part1(input: String) -> String {
    // let mut min_minutes = u32::MAX;
    //let mut min_minutes = 4000; // found 3952
    //let mut min_minutes = 3000; // found 2956
    //let mut min_minutes = 2000; // found 1963
    //let mut min_minutes = 1000; // found 933
    //let mut min_minutes = 500; // not found in a reasonable time
    //let mut min_minutes = 700; //  found 669
    //let mut min_minutes = 600; //  587
    let mut min_minutes = 550; //  
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

    let mut visited: HashMap<State, u32> = HashMap::new();

    dfs(&mut visited, last_pos, 1, &mut initial_state, &mut min_minutes, rows, cols);

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
        println!("sample answer: {}", part1(util::read_file("inputs/day24-sample2.txt")));
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/day24.txt"),
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
