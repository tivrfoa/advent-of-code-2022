use crate::util;

#[derive(Debug, Clone)]
struct Pos {
	row: usize,
	col: usize,
}

impl Pos {
	fn new(row: usize, col: usize) -> Self {
		Self {
			row,
			col,
		}
	}
}

fn draw_lines(grid: &mut Vec<Vec<char>>, mut lines: Vec<Vec<Pos>>) {
	for positions in lines {
		for i in 0..positions.len() - 1 {
			let mut start_x = positions[i].col;
			let mut start_y = positions[i].row;
			let mut end_x   = positions[i + 1].col;
			let mut end_y   = positions[i + 1].row;

			if start_x > end_x {
				let tmp = end_x;
				end_x = start_x;
				start_x = tmp;
			}

			if start_y > end_y {
				let tmp = end_y;
				end_y = start_y;
				start_y = tmp;
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
    let mut ans = 0;
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
			if x < min_x { min_x = x; }
			if x > max_x { max_x = x; }
			if y < min_y { min_y = y; }
			if y > max_y { max_y = y; }

			l.push(Pos::new(y, x));
		}
		lines.push(l);
    }

	// println!("{lines:?}");

	let cols = max_x - min_x + 1;
	let rows = max_y + 1;
	// apply reduction on x to avoid grid greater than necessary
	for i in 0..lines.len() {
		for pos in &mut lines[i] {
			pos.col -= min_x;
		}
	}
	// println!("{lines:?}");


	let mut grid: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
	let sand_source: Pos = Pos::new(0, 500 - min_x);
	grid[sand_source.row][sand_source.col] = '+';
	draw_lines(&mut grid, lines);

	// dbg(&grid);

	let mut units_of_sand_come_to_rest = 0;

	// Instead of always starting from top ('+'), it'll just go to previous
	// location and it'll repeat the same process
	// it should stop when one sand goes outside the grid

	let mut curr_x = sand_source.col;
	let mut curr_y = sand_source.row;
	loop {
		// try down
		while curr_y + 1 < rows && grid[curr_x][curr_y + 1] == '.' {
			curr_y += 1;
		}

		if curr_y + 1 == rows {
			break;
		}

		// try down left
		if curr_x == 0 {
			break;
		}

		if grid[curr_x - 1][curr_y + 1] == '.' {
			curr_x = curr_x

		// try down right
	}

	units_of_sand_come_to_rest
}

//pub fn solve_part2(input: String) -> usize {
//}

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
        let input = util::read_file("inputs/day14-sample.txt");
        assert_eq!(13, solve(input));
    }

    //#[test]
    //fn part1_input() {
    //    let input = util::read_file("inputs/day14.txt");
    //    assert_eq!(5529, solve(input));
    //}

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day14-sample.txt");
    //    assert_eq!(140, solve_part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day14.txt");
    //    assert_eq!(27690, solve_part2(input));
    //}
}
