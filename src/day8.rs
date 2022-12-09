use crate::util;

const ZERO: u8 = '0' as u8;

fn get_qt_col<I: Iterator<Item = usize>>(grid: &Vec<Vec<u8>>, iter: I, r: usize, c: usize) -> u32 {
    let mut qt = 0;
    for j in iter {
        qt += 1;
        if grid[r][j] >= grid[r][c] {
            break;
        }
    }
    qt
}

fn get_qt_row<I: Iterator<Item = usize>>(grid: &Vec<Vec<u8>>, iter: I, r: usize, c: usize) -> u32 {
    let mut qt = 0;
    for j in iter {
        qt += 1;
        if grid[j][c] >= grid[r][c] {
            break;
        }
    }
    qt
}

pub fn solve(input: String) -> u32 {
    let mut grid: Vec<Vec<u8>> = vec![];

    for line in input.lines() {
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
            let qt_left = get_qt_col(&grid, (0..c).rev(), r, c);

            // right
            let qt_right = get_qt_col(&grid, c + 1..cols, r, c);

            // bottom
            let qt_bottom = get_qt_row(&grid, r + 1..rows, r, c);

            // top
            let qt_top = get_qt_row(&grid, (0..r).rev(), r, c);

            let scenic_score = qt_left * qt_right * qt_bottom * qt_top;
            if scenic_score > max {
                max = scenic_score;
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part2_sample() {
		let input = util::read_file("inputs/sample-day8.txt");
		assert_eq!(8, solve(input));
	}

	#[test]
	fn part2_input() {
		let input = util::read_file("inputs/input-day8.txt");
		assert_eq!(574080, solve(input));
	}
}
