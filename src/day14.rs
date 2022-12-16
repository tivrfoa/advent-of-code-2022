use crate::util;

use std::cmp::Ordering;
use std::str::Chars;

type Row = usize;
type Col = usize;

#[derive(Debug, Clone)]
struct Pos {
	row: Row,
	col: Col,
}

impl Pos {
	fn new(row: Row, col: Col) -> Self {
		Self {
			row,
			col,
		}
	}
}

fn draw_lines(grid: &mut Vec<Vec<char>>, lines: &Vec<Vec<Pos>>) {
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
			println!("{xy:?}");
			let (x, y) = xy.split_once(',').unwrap();
			let x: Col = (*x).parse::<usize>().unwrap();
			let y: Row = (*y).parse::<usize>().unwrap();
			if x < min_x { min_x = x; }
			if x > max_x { max_x = x; }
			if y < min_y { min_y = y; }
			if y > max_y { max_y = y; }

			l.push(Pos::new(x, y));
		}
		lines.push(l);
    }

	println!("{lines:?}");

	let cols = max_x - min_x;
	let rows = max_y - min_y;
	let mut grid: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
	let sand_source: Pos = Pos::new(0, 500 - min_x);
	grid[sand_source.row][sand_source.col] = '+';
	draw_lines(&mut grid, &lines);


	0
}

//pub fn solve_part2(input: String) -> usize {
//}

#[allow(dead_code)]
fn dbg(grid: &Vec<Vec<u8>>) {
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
