/*
What is the grand total found by adding together all of the answers
to the individual problems?
*/
pub fn part1(input: &str) -> String {
	let mut ans = 0;
	let mut grid: Vec<Vec<usize>> = vec![];
	let mut operations = vec![];

	for line in input.lines() {
		let line = line.trim();
		if line.starts_with("+") || line.starts_with("*") {
			for c in line.split_ascii_whitespace() {
				operations.push(c);
			}
		} else {
			grid.push(line.split_ascii_whitespace()
				.map(|s| s.parse::<usize>().unwrap())
				.collect());
		}
	}
	// dbg!(grid, operations);
	let rows = grid.len();
	let cols = grid[0].len();
	for c in 0..cols {
		let mut res = grid[0][c];
		if operations[c] == "+" {
			for r in 1..rows {
				res += grid[r][c];
			}
		} else {
			for r in 1..rows {
				res *= grid[r][c];
			}
		}
		ans += res;
	}

	ans.to_string()
}


pub fn part2(input: &str) -> String {
	let mut ans = 0;
	let mut grid: Vec<Vec<char>> = vec![];
	let mut operations = vec![];
	for line in input.lines() {
		if line.starts_with("+") || line.starts_with("*") {
			for c in line.split_ascii_whitespace() {
				operations.push(c);
			}
		} else {
			grid.push(line.chars().collect());
		}
	}
	let rows = grid.len();
	let cols = grid[0].len();
	let mut col = 0;
	for o in operations {
		let mut res = if o == "+" { 0 } else { 1 };
		while col < cols {
			let mut all_spaces = true;
			let mut num: usize = 0;
			for r in 0..rows {
				if grid[r][col] != ' ' {
					num = num * 10 + (grid[r][col] as u8 - b'0') as usize;
					all_spaces = false;
				}
			}
			col += 1;
			if all_spaces { break; }
			res = if o == "+" { res + num } else { res * num };
		}
		ans += res;
	}
	ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2025/day6-sample.txt");
        assert_eq!("4277556", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2025/day6.txt");
        assert_eq!("5784380717354", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2025/day6-sample.txt");
        assert_eq!("3263827", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2025/day6.txt");
        assert_eq!("7996218225744", part2(input));
    }
}
