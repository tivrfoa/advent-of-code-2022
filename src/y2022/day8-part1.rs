use std::env;
use std::fs;

const ZERO: u8 = '0' as u8;

const NOT_VISIBLE: i8 = -1;
const VISIBLE: i8 = 1;
const NOT_VISITED: i8 = 0;

fn main() {
    let mut visible_trees = 0;
    let contents = get_file_contents();
    let mut grid: Vec<Vec<u8>> = vec![];

    for line in contents.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c as u8 - ZERO);
        }
        grid.push(row);
    }

    let rows = grid.len();
    let cols = grid[0].len();
    println!("{} x {} grid", rows, cols);

    let mut visible_grid: Vec<Vec<i8>> = vec![vec![0; cols]; rows];
    for row in 0..rows {
        visible_grid[row][0] = 1;
        visible_grid[row][cols - 1] = 1;
        visible_trees += 2;
    }
    for col in 1..cols - 1 {
        visible_grid[0][col] = 1;
        visible_grid[rows - 1][col] = 1;
        visible_trees += 2;
    }

    println!("vt {}", visible_trees);

    // for x in 0..rows {
    // 	println!("{:?}", visible_grid[x]);
    // }

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            // left
            let is_visible_from_left = {
                let mut is_visible = true;
                for j in (0..c).rev() {
                    if grid[r][j] >= grid[r][c] {
                        is_visible = false;
                        break;
                    }
                }
                is_visible
            };

            // right
            let is_visible_from_right = {
                let mut is_visible = true;
                for j in c + 1..cols {
                    if grid[r][j] >= grid[r][c] {
                        is_visible = false;
                        break;
                    }
                }
                is_visible
            };

            // bottom
            let is_visible_from_bottom = {
                let mut is_visible = true;
                for j in r + 1..rows {
                    if grid[j][c] >= grid[r][c] {
                        is_visible = false;
                        break;
                    }
                }
                is_visible
            };

            // top
            let is_visible_from_top = {
                let mut is_visible = true;
                for j in (0..r).rev() {
                    if grid[j][c] >= grid[r][c] {
                        is_visible = false;
                        break;
                    }
                }
                is_visible
            };

            if is_visible_from_left
                || is_visible_from_right
                || is_visible_from_top
                || is_visible_from_bottom
            {
                visible_grid[r][c] = VISIBLE;
                visible_trees += 1;
            } else {
                visible_grid[r][c] = NOT_VISIBLE;
            }
        }
    }

    for x in 0..rows {
        println!("{:?}", visible_grid[x]);
    }
    println!("ans: {}", visible_trees);
}

fn get_file_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
