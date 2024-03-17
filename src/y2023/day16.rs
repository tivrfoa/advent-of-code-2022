use crate::util;
use util::*;

const LEFT: usize = 0;
const RIGHT: usize = 1;
const UP: usize = 2;
const DOWN: usize = 3;

struct Beam {
    dir: usize,
    r: usize,
    c: usize,
}

impl Beam {
    fn new(dir: usize, r: usize, c: usize) -> Self {
        Self {
            dir, r, c,
        }
    }
}

fn solve(grid: &[Vec<char>], beam: Beam) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited: Vec<Vec<[bool; 4]>> = vec![vec![[false; 4]; cols]; rows];
    let mut beams: Vec<Beam> = vec![beam];

    while let Some(Beam { dir: d, r, c }) = beams.pop() {
        if visited[r][c][d] {
            continue;
        }
        visited[r][c][d] = true;

        match grid[r][c] {
            '.' => {
                match d {
                    RIGHT => {
                        if c + 1 < cols {
                            beams.push(Beam::new(d, r, c + 1));
                        }
                    }
                    LEFT => {
                        if c > 0 {
                            beams.push(Beam::new(d, r, c - 1));
                        }
                    }
                    UP => {
                        if r > 0 {
                            beams.push(Beam::new(d, r - 1, c));
                        }
                    }
                    DOWN => {
                        if r + 1 < rows {
                            beams.push(Beam::new(d, r + 1, c));
                        }
                    }
                    _ => panic!("Invalid dir: {d}"),
                }
            }
            '-' => {
                match d {
                    RIGHT => {
                        if c + 1 < cols {
                            beams.push(Beam::new(d, r, c + 1));
                        }
                    }
                    LEFT => {
                        if c > 0 {
                            beams.push(Beam::new(d, r, c - 1));
                        }
                    }
                    UP | DOWN => {
                        // split left
                        if c > 0 {
                            beams.push(Beam::new(LEFT, r, c - 1));
                        }
                        // split right
                        if c + 1 < cols {
                            beams.push(Beam::new(RIGHT, r, c + 1));
                        }
                    }
                    _ => panic!("Invalid dir: {d}"),
                }
            }
            '|' => {
                match d {
                    LEFT | RIGHT => {
                        // split up
                        if r > 0 {
                            beams.push(Beam::new(UP, r - 1, c));
                        }
                        // split down
                        if r + 1 < rows {
                            beams.push(Beam::new(DOWN, r + 1, c));
                        }
                    }
                    DOWN => {
                        if r + 1 < rows {
                            beams.push(Beam::new(d, r + 1, c));
                        }
                    }
                    UP => {
                        if r > 0 {
                            beams.push(Beam::new(d, r - 1, c));
                        }
                    }
                    _ => panic!("Invalid dir: {d}"),
                }
            }
            '\\' => {
                match d {
                    RIGHT => {
                        // goes down
                        if r + 1 < rows {
                            beams.push(Beam::new(DOWN, r + 1, c));
                        }
                    }
                    LEFT => {
                        // goes up
                        if r > 0 {
                            beams.push(Beam::new(UP, r - 1, c));
                        }
                    }
                    UP => {
                        // goes left
                        if c > 0 {
                            beams.push(Beam::new(LEFT, r, c - 1));
                        }
                    }
                    DOWN => {
                        // goes right
                        if c + 1 < cols {
                            beams.push(Beam::new(RIGHT, r, c + 1));
                        }
                    }
                    _ => panic!("Invalid dir: {d}"),
                }
            }
            '/' => {
                match d {
                    RIGHT => {
                        // goes up
                        if r > 0 {
                            beams.push(Beam::new(UP, r - 1, c));
                        }
                    }
                    LEFT => {
                        // goes down
                        if r + 1 < rows {
                            beams.push(Beam::new(DOWN, r + 1, c));
                        }
                    }
                    UP => {
                        // goes right
                        if c + 1 < cols {
                            beams.push(Beam::new(RIGHT, r, c + 1));
                        }
                    }
                    DOWN => {
                        // goes left
                        if c > 0 {
                            beams.push(Beam::new(LEFT, r, c - 1));
                        }
                    }
                    _ => panic!("Invalid dir: {d}"),
                }
            }
            _ => panic!("Invalid char: {}", grid[r][c]),
        }
    }

    let mut ans = 0;
    for row in visited {
        for cell in row {
            for i in 0..4 {
                if cell[i] {
                    ans += 1;
                    break;
                }
            }
        }
    }

    ans
}

pub fn part1(input: &str) -> String {
    let grid = input.to_char_grid();
    solve(&grid, Beam::new(RIGHT, 0, 0)).to_string()
}

pub fn part2(input: &str) -> String {
    let grid = input.to_char_grid();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut max = 0;

    // top to bottom and bottom to top
    for c in 0..cols {
        max = max.max(solve(&grid, Beam::new(DOWN, 0, c)));
        max = max.max(solve(&grid, Beam::new(UP, rows - 1, c)));
    }

    // right and left
    for r in 0..rows {
        max = max.max(solve(&grid, Beam::new(RIGHT, r, 0)));
        max = max.max(solve(&grid, Beam::new(LEFT, r, cols - 1)));
    }

    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2023/day16-sample.txt");
        assert_eq!("46", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day16.txt");
        assert_eq!("8389", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day16-sample.txt");
        assert_eq!("51", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day16.txt");
        assert_eq!("8564", part2(input));
    }
}
