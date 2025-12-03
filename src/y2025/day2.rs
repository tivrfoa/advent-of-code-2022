

fn parse(input: &str) -> Vec<(&str, &str)> {
	let mut ids_range = vec![];

    for line in input.trim().split(',') {
		let (first, last) = line.split_once('-').unwrap();
		ids_range.push((first, last));
    }

    ids_range
}

fn has_odd_digits(n: i64) -> bool {
    // number of digits = floor(log10(n)) + 1
    let digits = n.ilog10() + 1;
    digits % 2 == 1
}


pub fn part1(input: &str) -> String {
	let mut ans = 0;
    let seq = parse(input);

	for (f, l) in seq {
		let a: i64 = f.parse().unwrap();
		let b: i64 = l.parse().unwrap();
		for n in a..=b {
			if has_odd_digits(n) { continue; }
			let digits = n.ilog10() + 1;
            let half = digits / 2;
            let d = 10i64.pow(half);
            let x = n / d;
            let y = n % d;
			if x == y {
				// println!("{n} -> {x} {y} {d}");
				ans += n;
			}
		}
	}

	ans.to_string()
}

fn is_invalid(ns: String) -> bool {
	let len = ns.len();
	let half = len / 2;
	for rep_len in 1..=half {
		if len % rep_len != 0 {
			continue;
		}
		let mut good = true;
		let s = &ns[0..rep_len];
		for pos in (rep_len..=len - rep_len).step_by(rep_len) {
			let target = &ns[pos..pos + rep_len];
			if s != target {
				good = false;
				break;
			}
		}
		if good {
			println!("Good {ns}, rep length: {rep_len}, s = {s}");
			return true;
		}
	}
	false
}

pub fn part2(input: &str) -> String {
	let mut ans = 0;
    let seq = parse(input);

	for (f, l) in seq {
		let a: i64 = f.parse().unwrap();
		let b: i64 = l.parse().unwrap();
		for n in a..=b {
			if is_invalid(n.to_string()) {
				ans += n;
			}
		}
	}

	ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2025/day2-sample.txt");
        assert_eq!("1227775554", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2025/day2.txt");
        assert_eq!("54234399924", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2025/day2-sample.txt");
        assert_eq!("4174379265", part2(input));
    }

    #[test]
    fn p2() {
       let input = include_str!("../../inputs/2025/day2.txt");
       assert_eq!("70187097315", part2(input));
    }
}
