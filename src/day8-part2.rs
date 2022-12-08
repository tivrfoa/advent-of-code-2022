use std::env;
use std::fs;

const ZERO: u8 = '0' as u8;

fn main() {
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

    let mut max = 4;

    // It doesn't need to consider borders, because it would multiply by 0
    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            // left
            let qt_left = {
                let mut qt = 0;
                for j in (0..c).rev() {
                    qt += 1;
                    if grid[r][j] >= grid[r][c] {
                        break;
                    }
                }
                qt
            };

            // right
            let qt_right = {
                let mut qt = 0;
                for j in c + 1..cols {
                    qt += 1;
                    if grid[r][j] >= grid[r][c] {
                        break;
                    }
                }
                qt
            };

            // bottom
            let qt_bottom = {
                let mut qt = 0;
                for j in r + 1..rows {
                    qt += 1;
                    if grid[j][c] >= grid[r][c] {
                        break;
                    }
                }
                qt
            };

            // top
            let qt_top = {
                let mut qt = 0;
                for j in (0..r).rev() {
                    qt += 1;
                    if grid[j][c] >= grid[r][c] {
                        break;
                    }
                }
                qt
            };

            let scenic_score = qt_left * qt_right * qt_bottom * qt_top;
            if scenic_score > max {
                max = scenic_score;
            }
        }
    }

    println!("ans: {}", max);
}

fn get_file_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
