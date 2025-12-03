fn parse(input: &str) -> Vec<Vec<usize>> {
	let mut ret = vec![];
    for line in input.lines() {
		let mut r: Vec<usize> = Vec::with_capacity(line.len());
		for c in line.chars() {
			r.push((c as u8 - b'0') as usize);
		}
		ret.push(r);
    }
    ret
}

pub fn part1(input: &str) -> String {
	let mut sum: usize = 0;
    let banks = parse(input);
	for b in &banks {
		sum += solve(b);
	}
	sum.to_string()
}

fn solve(bank: &[usize]) -> usize {
	let mut left = 0;
	let mut right = 0;
	let last = bank.len() - 1;

	for (i, n) in bank.iter().enumerate() {
		let n = *n;
		if n > left && i != last {
			left = n;
			right = 0;
		} else if n > right {
			right = n;
		}
	}
	left * 10 + right
}

pub fn part2(input: &str) -> String {
	let mut sum: usize = 0;
    let banks = parse(input);
	for b in &banks {
		sum += solve2(b);
	}
	sum.to_string()
}

fn solve2(bank: &[usize]) -> usize {
	let mut n = 0;
	let mut bb = [0; 12];
	let last = bank.len() - 1;

	for (i, n) in bank.iter().enumerate() {
		let n = *n;
		for j in 0..bb.len() {
			// must make sure that there are at least enough numbers
			// remaining to fill bb
			if n > bb[j] && i + (bb.len() - j) - 1 <= last {
				// zero right values
				for k in j..bb.len() {
					bb[k] = 0;
				}
				bb[j] = n;
				break;
			}
		}
	}
	for b in bb {
		n = n * 10 + b;
	}
	n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2025/day3-sample.txt");
        assert_eq!("357", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2025/day3.txt");
        assert_eq!("17493", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2025/day3-sample.txt");
        assert_eq!("3121910778619", part2(input));
    }

    #[test]
    fn p2() {
      let input = include_str!("../../inputs/2025/day3.txt");
      assert_eq!("173685428989126", part2(input));
    }
}
