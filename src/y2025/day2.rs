

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
	let mut invalid_ids: Vec<i64> = vec![];
    let seq = parse(input);
	//dbg!(seq);

	for (f, l) in seq {
		let a: i64 = f.parse().unwrap();
		let b: i64 = l.parse().unwrap();
		for n in a..=b {
			let mut d = 10;
			while d <= 1_000_000 {
				if n <= d || has_odd_digits(n) { break; }
				let x = n / d;
				let y = n % d;
				if x == y {
					println!("{n} -> {x} {y} {d}");
					invalid_ids.push(n);
				}
				d *= 10;
			}
		}
	}
	// dbg!(&invalid_ids);

	invalid_ids.iter().sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2025/day2-sample.txt");
        assert_eq!("1227775554", part1(input));
    }

    //#[test]
    //fn p1() {
    //    let input = include_str!("../../inputs/2025/day2.txt");
    //    assert_eq!("995", part1(input));
    //}

    //#[test]
    //fn p2s() {
    //    let input = include_str!("../../inputs/2025/day2-sample.txt");
    //    assert_eq!("6", part2(input));
    //}

    //#[test]
    //fn p2() {
    //    let input = include_str!("../../inputs/2025/day2.txt");
    //    assert_eq!("5847", part2(input));
    //}
}
