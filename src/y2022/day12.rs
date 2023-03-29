use crate::util;

// GOAL: go from S to E in few steps as possible

/*
RULES

  1. Can go just one square higher
  2. Can go to any square lower or equal

Store for each position in the grid how many steps were used
to get there and don't continue if already visited that place.
*/

use std::collections::VecDeque;

const A: u8 = 'a' as u8;

#[derive(Debug)]
struct State {
    steps: u32,
    position: (usize, usize),
}

impl State {
    fn new(position: (usize, usize), steps: u32) -> Self {
        Self { steps, position }
    }
}

pub fn solve(input: String) -> u32 {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut S: (usize, usize) = (0, 0);
    let mut E: (usize, usize) = (0, 0);

    {
        let mut row = 0;
        let mut found_S = false;
        let mut found_E = false;
        for line in input.lines() {
            if !found_S && let Some(c) = line.find('S') {
				S = (row, c);
				found_S = true;
			}
            if !found_E && let Some(c) = line.find('E') {
				E = (row, c);
				found_E = true;
			}
            // grid.push(line.chars().collect());
            grid.push(line.as_bytes().to_vec());
            row += 1;
        }
    }

    // set S height
    grid[S.0][S.1] = 'a' as u8;

    // set E height
    grid[E.0][E.1] = 'z' as u8;

    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    visited[S.0][S.1] = true;

    let mut queue = VecDeque::from([State::new(S, 0)]);

    while let Some(s) = queue.pop_front() {
        if s.position == E {
            return s.steps;
        }

        let (r, c) = s.position;
        let next_steps = s.steps + 1;

        let dirs = [
            (c > 0, r, if c > 0 { c - 1 } else { 0 }),
            (c < cols - 1, r, c + 1),
            (r > 0, if r > 0 { r - 1 } else { 0 }, c),
            (r < rows - 1, r + 1, c),
        ];

        for (cond, next_row, next_col) in dirs {
            if cond && !visited[next_row][next_col] && grid[next_row][next_col] <= grid[r][c] + 1 {
                visited[next_row][next_col] = true;
                queue.push_back(State::new((next_row, next_col), next_steps));
            }
        }
    }

    panic!("Did not get to E: {:?}", E);
}

// I think we just need to make E the start and return whenever we find an 'a'
pub fn solve_part2(input: String) -> u32 {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut S: (usize, usize) = (0, 0);
    let mut E: (usize, usize) = (0, 0);

    {
        let mut row = 0;
        let mut found_S = false;
        let mut found_E = false;
        for line in input.lines() {
            if !found_S && let Some(c) = line.find('S') {
				S = (row, c);
				found_S = true;
			}
            if !found_E && let Some(c) = line.find('E') {
				E = (row, c);
				found_E = true;
			}
            // grid.push(line.chars().collect());
            grid.push(line.as_bytes().to_vec());
            row += 1;
        }
    }

    // set S height
    grid[S.0][S.1] = 'a' as u8; // S probably does not matter for part 2

    // set E height
    grid[E.0][E.1] = 'z' as u8;

    let rows = grid.len();
    let cols = grid[0].len();
    let get_neighbours = |r: usize, c: usize| -> Vec<(usize, usize)> {
        let mut nn = vec![];

        if r < rows - 1 {
            nn.push((r + 1, c));
        }
        if r > 0 {
            nn.push((r - 1, c));
        }
        if c > 0 {
            nn.push((r, c - 1));
        }
        if c < cols - 1 {
            nn.push((r, c + 1));
        }

        nn
    };

    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    visited[E.0][E.1] = true;
    let mut queue = VecDeque::new();
    queue.push_back(State::new(E, 0));

    while let Some(s) = queue.pop_front() {
        let (r, c) = s.position;
        if grid[r][c] == A {
            return s.steps;
        }

        let next_steps = s.steps + 1;

        for (to_row, to_col) in get_neighbours(r, c) {
            if grid[to_row][to_col] + 1 >= grid[r][c] && !visited[to_row][to_col] {
                visited[to_row][to_col] = true;
                queue.push_back(State::new((to_row, to_col), next_steps));
            }
        }
    }

    panic!("Did not get to E: {:?}", E);
}

fn dbg(grid: &Vec<Vec<u8>>) {
    for i in 0..grid.len() {
        println!("{:?}", grid[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2022/day12-sample.txt");
        assert_eq!(31, solve(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2022/day12.txt");
        assert_eq!(408, solve(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2022/day12-sample.txt");
        assert_eq!(29, solve_part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2022/day12.txt");
        assert_eq!(399, solve_part2(input));
    }
}
