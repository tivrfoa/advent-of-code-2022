use crate::util;

use std::collections::HashMap;

const SHAPES: [char; 5] = ['-', '+', 'L', 'I', 'S'];

fn can_move_left(grid: &[Vec<char>], row: usize, left_edge: usize) -> bool {
    if left_edge > 0 && grid[row][left_edge - 1] == '.' {
        true
    } else {
        false
    }
}

fn can_move_right(grid: &[Vec<char>], row: usize, left_edge: usize, width: usize) -> bool {
    if left_edge + width < COLS && grid[row][left_edge + width] == '.' {
        true
    } else {
        false
    }
}

fn can_fall(grid: &[Vec<char>], row: usize, left_edge: usize, width: usize) -> bool {
    for c in left_edge..left_edge + width {
        if grid[row + 1][c] == '#' {
            return false;
        }
    }
    true
}

const LINES: usize = 10_000;
const COLS: usize = 7;

pub fn solve(input: String) -> usize {
    let input = input.trim();
    let moves: Vec<char> = input.chars().collect();
    println!("{:?}", moves);
    let mut curr_shape = 0;
    let mut tallest_rock_row: usize = LINES;
    // The tall, vertical chamber is exactly seven units wide.
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; COLS]; LINES + 1];
    grid[LINES] = vec!['#'; COLS];
    let mut curr_move = 0;

    for _ in 0..2022 {
        // for _ in 0..20 {
        // Each rock appears so that its left edge is two units away from the
        // left wall and its bottom edge is three units above the highest rock
        // in the room (or the floor, if there isn't one).
        let mut row = tallest_rock_row - 4;
        let mut left_edge = 2;

        loop {
            // loop until rock comes to rest

            match SHAPES[curr_shape] {
                '-' => {
                    // move
                    if moves[curr_move] == '<' {
                        if can_move_left(&grid, row, left_edge) {
                            left_edge -= 1;
                        }
                    } else {
                        if can_move_right(&grid, row, left_edge, 4) {
                            left_edge += 1;
                        }
                    }

                    // fall
                    if can_fall(&grid, row, left_edge, 4) {
                        row += 1;
                    } else {
                        // rock comes to rest
                        for c in left_edge..left_edge + 4 {
                            grid[row][c] = '#';
                        }

                        // check if we need to update tallest rock
                        if row < tallest_rock_row {
                            tallest_rock_row = row;
                        }
                        break;
                    }
                }
                '+' => {
                    // move
                    if moves[curr_move] == '<' {
                        if can_move_left(&grid, row - 2, left_edge + 1)
                            && can_move_left(&grid, row - 1, left_edge)
                            && can_move_left(&grid, row, left_edge + 1)
                        {
                            left_edge -= 1;
                        }
                    } else {
                        if can_move_right(&grid, row - 2, left_edge + 1, 1)
                            && can_move_right(&grid, row - 1, left_edge, 3)
                            && can_move_right(&grid, row, left_edge + 1, 1)
                        {
                            left_edge += 1;
                        }
                    }

                    // fall
                    if can_fall(&grid, row, left_edge + 1, 1)
                        && can_fall(&grid, row - 1, left_edge, 1)
                        && can_fall(&grid, row - 1, left_edge + 2, 1)
                    {
                        row += 1;
                    } else {
                        // rock comes to rest
                        grid[row - 2][left_edge + 1] = '#';
                        grid[row - 1][left_edge] = '#';
                        grid[row - 1][left_edge + 1] = '#';
                        grid[row - 1][left_edge + 2] = '#';
                        grid[row][left_edge + 1] = '#';

                        // check if we need to update tallest rock
                        if row - 2 < tallest_rock_row {
                            tallest_rock_row = row - 2;
                        }
                        break;
                    }
                }
                'L' => {
                    // reverse L : 3 x 3

                    // move
                    if moves[curr_move] == '<' {
                        if can_move_left(&grid, row - 2, left_edge + 2)
                            && can_move_left(&grid, row - 1, left_edge + 2)
                            && can_move_left(&grid, row, left_edge)
                        {
                            left_edge -= 1;
                        }
                    } else {
                        if can_move_right(&grid, row - 2, left_edge + 2, 1)
                            && can_move_right(&grid, row - 1, left_edge + 2, 1)
                            && can_move_right(&grid, row, left_edge, 3)
                        {
                            left_edge += 1;
                        }
                    }

                    // fall
                    if can_fall(&grid, row, left_edge, 3) {
                        row += 1;
                    } else {
                        // rock comes to rest
                        grid[row - 2][left_edge + 2] = '#';
                        grid[row - 1][left_edge + 2] = '#';
                        grid[row][left_edge] = '#';
                        grid[row][left_edge + 1] = '#';
                        grid[row][left_edge + 2] = '#';

                        // check if we need to update tallest rock
                        if row - 2 < tallest_rock_row {
                            tallest_rock_row = row - 2;
                        }
                        break;
                    }
                }
                'I' => {
                    let width: usize = 1;
                    let height: usize = 4;

                    // move
                    if moves[curr_move] == '<' {
                        if (row - height + 1..=row)
                            .map(|i| can_move_left(&grid, i, left_edge))
                            .find(|b| !*b)
                            .is_none()
                        {
                            left_edge -= 1;
                        }
                    } else {
                        if (row - height + 1..=row)
                            .map(|i| can_move_right(&grid, i, left_edge, width))
                            .find(|b| !*b)
                            .is_none()
                        {
                            left_edge += 1;
                        }
                    }

                    // fall
                    if can_fall(&grid, row, left_edge, width) {
                        row += 1;
                    } else {
                        // rock comes to rest
                        for r in row - height + 1..=row {
                            for c in left_edge..left_edge + width {
                                grid[r][c] = '#';
                            }
                        }

                        // check if we need to update tallest rock
                        if row - 3 < tallest_rock_row {
                            tallest_rock_row = row - 3;
                        }
                        break;
                    }
                }
                'S' => {
                    let width: usize = 2;
                    let height: usize = 2;

                    // move
                    if moves[curr_move] == '<' {
                        if (row - height + 1..=row)
                            .map(|i| can_move_left(&grid, i, left_edge))
                            .find(|b| !*b)
                            .is_none()
                        {
                            left_edge -= 1;
                        }
                    } else {
                        if (row - height + 1..=row)
                            .map(|i| can_move_right(&grid, i, left_edge, width))
                            .find(|b| !*b)
                            .is_none()
                        {
                            left_edge += 1;
                        }
                    }

                    // fall
                    if can_fall(&grid, row, left_edge, width) {
                        row += 1;
                    } else {
                        // rock comes to rest
                        for r in row - height + 1..=row {
                            for c in left_edge..left_edge + width {
                                grid[r][c] = '#';
                            }
                        }

                        // check if we need to update tallest rock
                        if row - 1 < tallest_rock_row {
                            tallest_rock_row = row - 1;
                        }
                        break;
                    }
                }
                _ => panic!(
                    "Invalid shape: {}, curr_shape {}",
                    SHAPES[curr_shape], curr_shape
                ),
            }

            curr_move = (curr_move + 1) % moves.len();
        }

        // draw(&grid, tallest_rock_row);
        curr_move = (curr_move + 1) % moves.len();
        curr_shape = (curr_shape + 1) % SHAPES.len();
    }

    LINES - tallest_rock_row
}

pub fn solve_part2(input: String, max: i64) -> i64 {
    0
}

#[allow(dead_code)]
fn draw(grid: &[Vec<char>], start_row: usize) {
    println!("Tallest rock row: {}", start_row);
    for r in start_row..=LINES {
        for c in 0..COLS {
            print!("{}", grid[r][c]);
        }
        println!();
    }
    println!("\n-----------------------\n");
}

#[allow(dead_code)]
fn dbg(grid: &Vec<Vec<char>>) {
    for item in grid {
        println!("{:?}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/day17-sample.txt");
        assert_eq!(3068, solve(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/day17.txt");
        assert_eq!(3106, solve(input));
    }

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day17-sample.txt");
    //    assert_eq!(56000011, solve_part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day17.txt");
    //    assert_eq!(12817603219131, solve_part2(input));
    //}
}
