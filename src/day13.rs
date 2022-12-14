use crate::util;

use std::collections::VecDeque;

fn compare(l1: &str, l2: &str) -> u32 {
	println!("comparing:\n{}\n{}", l1, l2);

	0
}

pub fn solve(input: String) -> u32 {
	let mut ans: u32 = 0;
    let mut pair = ["", ""];
	let mut i = 0;
	for line in input.lines() {
		if i == 2 {
			ans += compare(pair[0], pair[1]);
			i = 0;
		} else {
			pair[i] = line;
			i += 1;
		}
	}
	ans += compare(pair[0], pair[1]);

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
