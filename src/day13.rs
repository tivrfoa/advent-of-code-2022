use crate::util;

use std::collections::VecDeque;

const TEN: u8 = 58;

fn get_lists(line: &str) -> Vec<Vec<u8>> {
	let mut ret = vec![];
	let mut curr: Vec<u8> = vec![];
	let bytes = line.as_bytes();

	for i in 0..bytes.len() - 1 {
		if bytes[i] == b'[' {
			// do nothing
		} else if bytes[i] == b']' {
			if !curr.is_empty() {
				ret.push(curr);
				curr = vec![];
			}
		} else if bytes[i] == b',' {
			// do nothing
		} else {
			// it's a number, but it can be ten, so check next position
			let num = if bytes[i+1] == b'0' {
				TEN
			} else {
				bytes[i]
			};
			if curr.is_empty() && bytes[i-1] == b',' {
				ret.push(vec![num]);
			} else {
				curr.push(num);
			}
		}
	}
	if !curr.is_empty() {
		ret.push(curr);
	}

	ret
}

pub fn solve(input: String) -> u32 {
	let mut ans: u32 = 0;
    let mut pairs: Vec<Vec<u8>> = vec![];

	for line in input.lines() {
		if line.is_empty() {
			continue;
		}
		pairs.append(&mut get_lists(line));
	}

	for n in pairs {
		println!("{n:?}");
	}

	//let len = pairs.len();
	//for i in (0..len).step_by(2) {
	//	ans += compare(pairs[i], pairs[i + 1]);
	//}

	ans
}

fn dbg(grid: &Vec<Vec<u8>>) {
    for i in 0..grid.len() {
        println!("{:?}", grid[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/day13-sample.txt");
        assert_eq!(31, solve(input));
    }

    //#[test]
    //fn part1_input() {
    //    let input = util::read_file("inputs/day13.txt");
    //    assert_eq!(408, solve(input));
    //}

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day13-sample.txt");
    //    assert_eq!(29, solve_part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day13.txt");
    //    assert_eq!(399, solve_part2(input));
    //}
}
