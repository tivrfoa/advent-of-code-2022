use crate::util;

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    map_idx: usize,
    pos: (usize, usize),
}

impl State {
    fn move_to(&self, r: usize, c: usize) -> Self {
        Self {
            map_idx: self.map_idx,
            pos: (r, c),
        }
    }

    fn move_blizzards(&mut self, len: usize) {
        self.map_idx = (self.map_idx + 1) % len;
    }
}

const LEFT:  u8 = 0b1;
const RIGHT: u8 = 0b10;
const UP:    u8 = 0b100;
const DOWN:  u8 = 0b1000;

fn lcm(a: usize, b: usize) -> usize {
    (a / gcd(a, b)) * b
}

fn gcd(mut u: usize, mut v: usize) -> usize {
    if u == 0 { return v; }
    if v == 0 { return u; }

    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();

    loop {
        v >>= v.trailing_zeros();

        #[allow(clippy::manual_swap)]
        if u > v {
            // mem::swap(&mut u, &mut v);
            let temp = u;
            u = v;
            v = temp;
        }

        v -= u; // here v >= u

        if v == 0 { break; }
    }

    u << shift
}

fn generate_maps(mut grid: Vec<Vec<u8>>) -> Vec<Vec<Vec<u8>>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let len = lcm(rows - 2, cols - 2);
    let mut maps = Vec::with_capacity(len);

    maps.push(grid.clone());
    for _ in 1..len {
        grid = move_blizzards(grid);
        maps.push(grid.clone());
    }

    maps
}

fn move_blizzards(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut new_grid = vec![vec![0; cols]; rows];

    for (row, row_item) in grid.iter().enumerate().take(rows - 1).skip(1) {
        for (col, item) in row_item.iter().enumerate().take(cols - 1).skip(1) {
            if item & LEFT >= 1 {
                let col = if col == 1 { cols - 2 } else { col - 1 };
                new_grid[row][col] |= LEFT;
            }
            if item & RIGHT >= 1 {
                let col = if col == cols - 2 { 1 } else { col + 1 };
                new_grid[row][col] |= RIGHT;
            }
            if item & UP >= 1 {
                let row = if row == 1 { rows - 2 } else { row - 1 };
                new_grid[row][col] |= UP;
            }
            if item & DOWN >= 1 {
                let row = if row == rows - 2 { 1 } else { row + 1 };
                new_grid[row][col] |= DOWN;
            }
        }
    }

    new_grid
}

fn position_contain_blizzard(grid: &[Vec<u8>], r: usize, c: usize) -> bool {
    if grid[r][c] & LEFT != 0 ||
        grid[r][c] & RIGHT != 0 ||
        grid[r][c] & UP != 0 ||
        grid[r][c] & DOWN != 0 {
            true
    } else {
        false
    }
}

fn draw(grid: &[Vec<u8>]) {
    let rows = grid.len();
    let cols = grid[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if r == 0 || r == rows - 1 || c == 0 || c == cols - 1 {
                print!("#");
            } else {
                let l = match grid[r][c] {
                    LEFT => '<',
                    RIGHT => '>',
                    UP => '^',
                    DOWN => 'v',
                    0 => '.',
                    _ => '*',
                };
                print!("{l}");
            }
        }
        println!();
    }
}

type Pos = (usize, usize);

fn solve_with_min_heap(
    maps: &[Vec<Vec<u8>>],
    mut state: State,
    last_pos: Pos,
    minutes: u16,
) -> (u16, usize) {
    let rows = maps[0].len();
    let cols = maps[0][0].len();
    let mut visited: HashSet<State> = HashSet::new();
    let mut states: BinaryHeap<Reverse<(u16, State)>> = BinaryHeap::new();
    states.push(Reverse((minutes, state)));

    while let Some(Reverse((minutes, mut state))) = states.pop() {
        if state.pos == last_pos {
            return (minutes, state.map_idx);
        }

        // move blizzards, then check where we can go
        state.move_blizzards(maps.len());

        let moves: [(bool, (i32, i32)); 5] = [
            (state.pos.1 < cols - 2, (0, 1)),
            (state.pos.1 > 1, (0, -1)),
            (state.pos.0 > 1, (-1, 0)),
            (state.pos.0 < rows - 2, (1, 0)),
            (true, (0, 0)),
        ];

        for m in &moves {
            if m.0 {
                let r = (state.pos.0 as i32 + m.1.0) as usize;
                let c = (state.pos.1 as i32 + m.1.1) as usize;
                if !position_contain_blizzard(&maps[state.map_idx], r, c) {
                    let new_state = state.move_to(r, c);
                    if !visited.contains(&new_state) {
                        visited.insert(new_state.clone());
                        states.push(Reverse((minutes + 1, new_state)));
                    }
                }
            }
        }
    }

    (u16::MAX, usize::MAX)
}

fn part1(input: String) -> String {
    let grid = parse(input);
    let last_pos = (grid.len() - 2, grid[0].len() - 2); // row, col
    let maps = generate_maps(grid);

    let initial_state = State {
        map_idx: 1,
        pos: (1, 1),
    };

    let (min_minutes, _) = solve_with_min_heap(
        &maps,
        initial_state,
        last_pos,
        1,
    );

    (min_minutes + 1).to_string()
}

/// initial and final position "inside" the grid
fn solve(
    maps: &[Vec<Vec<u8>>],
    map_idx: usize,
    initial_pos: Pos,
    final_pos: Pos,
) -> (u16, usize) {
    println!("Trying to get from {:?} to {:?}", initial_pos, final_pos);
    let mut min_minutes = u16::MAX;
    let mut final_map_idx = usize::MAX;
    let mut minutes = 0;
    let mut initial_state = State {
        map_idx,
        pos: initial_pos,
    };

    println!("=========== INITIAL STATE =============");
    draw(&maps[initial_state.map_idx]);

    while minutes < 15 {
        minutes += 1;
        initial_state.move_blizzards(maps.len());

        while position_contain_blizzard(&maps[initial_state.map_idx], initial_state.pos.0, initial_state.pos.1) {
            println!("It can't enter the grid. minutes: {minutes}");
            draw(&maps[initial_state.map_idx]);
            minutes += 1;
            initial_state.move_blizzards(maps.len());
        }

        // println!("------grid before solve_with_min_heap-------");
        //draw(&maps[map_idx]);
        let (tmp_t, tmp_idx) = solve_with_min_heap(
            maps,
            initial_state.clone(),
            final_pos,
            minutes,
        );
        min_minutes = tmp_t;
        final_map_idx = tmp_idx;

        if min_minutes != u16::MAX {
            break;
        }

        println!("It didn't find a solution. Let's try again.");
    }

    if min_minutes == u16::MAX {
        panic!("Mission Failed");
    }

    println!("--->> debugging final grid");
    final_map_idx = (final_map_idx + 1) % maps.len();
    draw(&maps[final_map_idx]);

    // before return, we need to move blizzards in the final grid one more time
    (min_minutes + 1, final_map_idx)
}

fn part2(input: String) -> String {
    let mut sum = 0;
    let grid = parse(input);
    let initial_pos = (1, 1);
    let last_pos = (grid.len() - 2, grid[0].len() - 2);
    let maps = generate_maps(grid);

    // go
    let (minutes, map_idx) = solve(&maps, 0, initial_pos, last_pos);
    sum += minutes;

    // go back
    let (minutes, map_idx) = solve(&maps, map_idx, last_pos, (1, 1));
    sum += minutes;

    // go again
    let (minutes, _) = solve(&maps, map_idx, initial_pos, last_pos);
    sum += minutes;

    sum.to_string()
}

fn parse(input: String) -> Vec<Vec<u8>> {
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            match c {
                '<' => row.push(LEFT),
                '>' => row.push(RIGHT),
                '^' => row.push(UP),
                'v' => row.push(DOWN),
                _ => row.push(0),
            }
        }
        grid.push(row);
    }
    println!("parsed grid:");
    draw(&grid);
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2022/day24-sample2.txt");
        assert_eq!("18", part1(input));
    }

    #[test]
    fn part1_input() {
       let input = util::read_file("inputs/2022/day24.txt");
       assert_eq!("240", part1(input));
    }

    #[test]
    fn part2_sample() {
       let input = util::read_file("inputs/2022/day24-sample2.txt");
       assert_eq!("54", part2(input));
    }

    #[test]
    fn part2_input() {
      let input = util::read_file("inputs/2022/day24.txt");
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
            part1(util::read_file("inputs/2022/day24-sample2.txt"))
        );
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/2022/day24.txt"),
        };
        part1(input)
    }

    fn part2(&self, input: Option<String>, args: Vec<String>) -> String {
        println!(
            "sample answer: {}",
            part2(util::read_file("inputs/2022/day24-sample2.txt"))
        );
        let input = match input {
            Some(input) => input,
            None => util::read_file("inputs/2022/day24.txt"),
        };
        part2(input)
    }
}
