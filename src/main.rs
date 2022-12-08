use std::env;
use std::fs;

const ZERO: u8 = '0' as u8;

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
	let visible_rows = rows * 2;
	let visible_cols = cols * 2;
	visible_trees = visible_rows + visible_cols - 4; // - 4 to avoid double counting

	let mut visible_grid: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
	for row in 0..rows {
		visible_grid[row][0] = true;
		visible_grid[row][cols - 1] = true;
	}
	for col in 0..cols {
		visible_grid[0][col] = true;
		visible_grid[rows - 1][col] = true;
	}

	for r in 1..rows - 1 {
		for c in 1..cols - 1 {
			// left
			if visible_grid[r][c - 1] && grid[r][c - 1] < grid[r][c] {
				visible_grid[r][c] = true;
				visible_trees += 1;
				continue;
			}

			// right
			if visible_grid[r][c + 1] && grid[r][c + 1] < grid[r][c] {
				visible_grid[r][c] = true;
				visible_trees += 1;
				continue;
			}

			// top
			if visible_grid[r - 1][c] && grid[r - 1][c] < grid[r][c] {
				visible_grid[r][c] = true;
				visible_trees += 1;
				continue;
			}

			// bottom
			if visible_grid[r + 1][c] && grid[r + 1][c] < grid[r][c] {
				visible_grid[r][c] = true;
				visible_trees += 1;
				continue;
			}
		}
	}

    println!("ans: {}", visible_trees);
}

fn get_file_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
