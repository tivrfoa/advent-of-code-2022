fn turn(pos: i32, dir: char, n: i32) -> i32 {
	let n = n % 100;
	let next = if dir == 'L' { pos - n } else { pos + n };
	if next > 99 {
		next - 100
	} else if next < 0 {
		next + 100
	} else {
		next
	}
}

fn parse(input: &str) -> Vec<(char, i32)> {
	let mut ret = vec![];

    for line in input.lines() {
		let (c, n) = line.split_at(1);
		let c = c.chars().next().unwrap();
		let n: i32 = n.parse().unwrap();
		ret.push((c, n));
    }

    ret
}

pub fn part1(input: &str) -> String {
	let mut ans = 0;
	let mut pos = 50;
    let seq = parse(input);
	for (c, n) in &seq {
		pos = turn(pos, *c, *n);
		if pos == 0 { ans += 1; }
	}
	ans.to_string()
}

fn turn2(pos: i32, dir: char, n: i32) -> (i32, i32) {
	let t1 = n / 100;
	let n = n % 100;
	let next = if dir == 'L' { pos - n } else { pos + n };
	let (t2, next) = if next > 99 {
		(1, next - 100)
	} else if next < 0 {
		(1, next + 100)
	} else {
		(0, next)
	};
	(t1 + t2, next)
}

pub fn part2(input: &str) -> String {
	let mut ans = 0;
	let mut pos = 50;
    let seq = parse(input);
	for (c, n) in &seq {
		let (t, p) = turn2(pos, *c, *n);
		ans += t;
		pos = p;
	}
	ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn() {
        assert_eq!(0, turn(0, 'R', 0));
        assert_eq!(1, turn(0, 'R', 1));
        assert_eq!(0, turn(0, 'L', 0));
        assert_eq!(99, turn(0, 'L', 1));

        assert_eq!(99, turn(0, 'R', 99));
        assert_eq!(0, turn(1, 'R', 99));
        assert_eq!(1, turn(2, 'R', 99));
    }

    #[test]
    fn test_turn2() {
        assert_eq!((0, 0), turn2(0, 'R', 0));
        assert_eq!((0, 1), turn2(0, 'R', 1));
        assert_eq!((0, 0), turn2(0, 'L', 0));
        assert_eq!((1, 99), turn2(0, 'L', 1));

        assert_eq!((0, 99), turn2(0, 'R', 99));
        assert_eq!((1, 0),  turn2(1, 'R', 99));
        assert_eq!((1, 1),  turn2(2, 'R', 99));

        assert_eq!((1, 82),  turn2(50, 'L', 68));

        assert_eq!((1, 0),  turn2(50, 'R', 50));
        assert_eq!((1, 50),  turn2(50, 'R', 100));
        assert_eq!((10, 50),  turn2(50, 'R', 1000));
    }

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2025/day1-sample.txt");
        assert_eq!("3", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2025/day1.txt");
        assert_eq!("995", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2025/day1-sample.txt");
        assert_eq!("6", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2025/day1.txt");
        assert_eq!("1", part2(input));
    }
}
