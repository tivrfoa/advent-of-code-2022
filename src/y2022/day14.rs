use crate::util;

#[derive(Debug, Clone)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn draw_lines(grid: &mut [Vec<char>], lines: Vec<Vec<Pos>>) {
    for positions in lines {
        for i in 0..positions.len() - 1 {
            let mut start_x = positions[i].col;
            let mut start_y = positions[i].row;
            let mut end_x = positions[i + 1].col;
            let mut end_y = positions[i + 1].row;

            if start_x > end_x {
                std::mem::swap(&mut start_x, &mut end_x);
            }

            if start_y > end_y {
                std::mem::swap(&mut start_y, &mut end_y);
            }

            for y in start_y..=end_y {
                for x in start_x..=end_x {
                    grid[y][x] = '#';
                }
            }
        }
    }
}

/// Abyss means outside of the grid
pub fn solve(input: String) -> usize {
    let mut lines: Vec<Vec<Pos>> = vec![];
    let (mut min_x, mut max_x) = (usize::MAX, 0);
    let (mut min_y, mut max_y) = (usize::MAX, 0);

    for line in input.lines() {
        let mut l = vec![];
        for xy in line.split(" -> ") {
            // println!("{xy:?}");
            let (x, y) = xy.split_once(',').unwrap();
            let x = (*x).parse::<usize>().unwrap();
            let y = (*y).parse::<usize>().unwrap();
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }

            l.push(Pos::new(y, x));
        }
        lines.push(l);
    }

    let cols = max_x - min_x + 1;
    let rows = max_y + 1;
    // apply reduction on x to avoid grid greater than necessary
    for line in &mut lines {
        for pos in line {
            pos.col -= min_x;
        }
    }

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
    let sand_source: Pos = Pos::new(0, 500 - min_x);
    grid[sand_source.row][sand_source.col] = '+';
    draw_lines(&mut grid, lines);

    let mut units_of_sand_come_to_rest = 0;

    let mut curr_row = sand_source.row;
    let mut curr_col = sand_source.col;
    loop {
        // try down
        while curr_row + 1 < rows && grid[curr_row + 1][curr_col] == '.' {
            curr_row += 1;
        }

        if curr_row + 1 == rows {
            break;
        }

        // try down left
        if curr_col == 0 {
            break;
        }

        if grid[curr_row + 1][curr_col - 1] == '.' {
            curr_row += 1;
            curr_col -= 1;
            continue;
        }

        // try down right
        if curr_col + 1 == cols {
            break;
        }

        if grid[curr_row + 1][curr_col + 1] == '.' {
            curr_row += 1;
            curr_col += 1;
            continue;
        }

        // if if it reached, it can rest and we reset curr positions to source
        grid[curr_row][curr_col] = 's';
        curr_row = sand_source.row;
        curr_col = sand_source.col;
        units_of_sand_come_to_rest += 1;
    }

    units_of_sand_come_to_rest
}

pub fn solve_part2(input: String) -> usize {
    let mut lines: Vec<Vec<Pos>> = vec![];
    let (mut min_x, mut max_x) = (usize::MAX, 0);
    let (mut min_y, mut max_y) = (usize::MAX, 0);

    for line in input.lines() {
        let mut l = vec![];
        for xy in line.split(" -> ") {
            let (x, y) = xy.split_once(',').unwrap();
            let x = (*x).parse::<usize>().unwrap();
            let y = (*y).parse::<usize>().unwrap();
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }

            l.push(Pos::new(y, x));
        }
        lines.push(l);
    }

    let cols = max_x - min_x + 1 + max_y * 2;
    let rows = max_y + 1 + 2;
    // apply reduction on x to avoid grid greater than necessary
    for line in &mut lines {
        for pos in line {
            pos.col = pos.col - min_x + max_y;
        }
    }

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
    let sand_source: Pos = Pos::new(0, 500 - min_x + max_y);
    draw_lines(&mut grid, lines);

    grid[rows - 1].fill('#');

    dbg(&grid);

    println!("--------------------------------------");
    println!("--------------------------------------");

    let mut units_of_sand_come_to_rest = 0;
    let mut curr_row = sand_source.row;
    let mut curr_col = sand_source.col;

    loop {
        // try down
        while curr_row + 1 < rows && grid[curr_row + 1][curr_col] == '.' {
            curr_row += 1;
        }

        if grid[curr_row + 1][curr_col - 1] == '.' {
            curr_row += 1;
            curr_col -= 1;
            continue;
        }

        if grid[curr_row + 1][curr_col + 1] == '.' {
            curr_row += 1;
            curr_col += 1;
            continue;
        }

        if curr_row == sand_source.row && curr_col == sand_source.col {
            units_of_sand_come_to_rest += 1;
            break;
        }

        // if if it reached, it can rest and we reset curr positions to source
        grid[curr_row][curr_col] = 's';
        curr_row = sand_source.row;
        curr_col = sand_source.col;
        units_of_sand_come_to_rest += 1;
    }

    dbg(&grid);
    units_of_sand_come_to_rest
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
        let input = util::read_file("inputs/2022/day14-sample.txt");
        assert_eq!(24, solve(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2022/day14.txt");
        assert_eq!(683, solve(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2022/day14-sample.txt");
        assert_eq!(93, solve_part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2022/day14.txt");
        assert_eq!(28821, solve_part2(input));
    }
}
