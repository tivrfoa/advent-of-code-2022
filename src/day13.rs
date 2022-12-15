use crate::util;

const TEN: u8 = 58;

#[derive(Debug)]
struct Pair {
	a_list: Vec<Vec<u8>>,
	b_list: Vec<Vec<u8>>,
}

impl Pair {
	fn new() -> Self {
		Self {
			a_list: vec![],
			b_list: vec![],
		}
	}

	fn is_right_order(&self) -> bool {
		let b_len = self.b_list.len();

		for i in 0..self.a_list.len() {
			if i == b_len {
				return false;
			}

			let b_i_len = self.b_list[i].len();
			for j in 0..self.a_list[i].len() {
				if j == b_i_len {
					return false;
				}
				if self.a_list[i][j] < self.b_list[i][j] {
					return true;
				} else if self.a_list[i][j] > self.b_list[i][j] {
					return false;
				} else {
					// just continue
				}
			}
		}

		true
	}
}

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

pub fn solve(input: String) -> usize {
	let mut ans = 0;
    let mut pairs: Vec<Pair> = vec![];

	let mut pair = Pair::new();

	let mut i = 0;
	for line in input.lines() {
		if i == 2 {
			pairs.push(pair);
			pair = Pair::new();
			i = 0;
		} else if i == 0 {
			pair.a_list = get_lists(line);
			i += 1;
		} else {
			pair.b_list = get_lists(line);
			i += 1;
		}
	}
	pairs.push(pair);

	//for pair in pairs {
	//	println!("{pair:?}");
	//}

	for i in 0..pairs.len() {
		if pairs[i].is_right_order() {
			ans += i + 1;
			println!("index {} is good", i + 1);
		}
	}

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
