use crate::util;

// GOAL: go from S to E in few steps as possible

/*
RULES

  1. Can go just one square higher
  2. Can go to any square lower or equal

This is a DP problem

Store for each position in the grid how many steps were used
to get there and don't continue if already visited that place
with fewer steps.
*/

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct State {
    steps: u32,
    position: (usize, usize),
}

impl State {
    fn new(position: (usize, usize), steps: u32) -> Self {
        Self { steps, position }
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

    let mut dp: Vec<Vec<u32>> = vec![vec![u32::MAX; cols]; rows];
    let mut min_heap = BinaryHeap::new();
    min_heap.push(State::new(S, 0));

    while let Some(s) = min_heap.pop() {
        if s.position == E {
            return s.steps;
        }

        let (r, c) = s.position;
        let next_steps = s.steps + 1;

        // left
        if c > 0 && grid[r][c - 1] <= grid[r][c] + 1 && next_steps < dp[r][c - 1] {
            dp[r][c - 1] = next_steps;
            min_heap.push(State::new((r, c - 1), next_steps));
        }

        // right
        if c < cols - 1 && grid[r][c + 1] <= grid[r][c] + 1 && next_steps < dp[r][c + 1] {
            dp[r][c + 1] = next_steps;
            min_heap.push(State::new((r, c + 1), next_steps));
        }

        // up
        if r > 0 && grid[r - 1][c] <= grid[r][c] + 1 && next_steps < dp[r - 1][c] {
            dp[r - 1][c] = next_steps;
            min_heap.push(State::new((r - 1, c), next_steps));
        }

        // down
        if r < rows - 1 && grid[r + 1][c] <= grid[r][c] + 1 && next_steps < dp[r + 1][c] {
            dp[r + 1][c] = next_steps;
            min_heap.push(State::new((r + 1, c), next_steps));
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
        let input = util::read_file("inputs/day12-sample.txt");
        assert_eq!(31, solve(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/day12.txt");
        assert_eq!(408, solve(input));
    }

    #[test]
    #[ignore]
    fn part2_sample() {
        let input = util::read_file("inputs/day12-sample.txt");
        assert_eq!(123, solve(input));
    }

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day12.txt");
    //    assert_eq!(18085004878, solve(input));
    //}
}
