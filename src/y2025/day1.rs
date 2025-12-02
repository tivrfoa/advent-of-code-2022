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
		assert!(c == 'L' || c == 'R');
		let n: i32 = n.parse().unwrap();
		assert!(0 < n && n <= 999);
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
    let started_at_zero = pos == 0;
	let t = n / 100;
	let n = n % 100;
	let next = if dir == 'L' { pos - n } else { pos + n };
	if next > 99 {
		(t + 1, next - 100)
	} else if next < 0 {
        if started_at_zero {
            (t, next + 100)
        } else {
		    (t + 1, next + 100)
        }
	} else if next == 0 {
		(t + 1, next)
	} else {
		(t, next)
	}
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
        assert_eq!((0, 1), turn2(0, 'R', 1));
        assert_eq!((0, 99), turn2(0, 'L', 1));

        assert_eq!((1, 98), turn2(99, 'R', 99));
        assert_eq!((1, 0), turn2(99, 'L', 99));

        assert_eq!((1, 99), turn2(99, 'R', 100));
        assert_eq!((2, 0), turn2(99, 'R', 101));

        assert_eq!((1, 55), turn2(95, 'R', 60));

        assert_eq!((1, 25), turn2(75, 'L', 150));

        assert_eq!((0, 99), turn2(0, 'R', 99));
        assert_eq!((1, 0),  turn2(1, 'R', 99));
        assert_eq!((1, 1),  turn2(2, 'R', 99));

        assert_eq!((1, 82),  turn2(50, 'L', 68));

        assert_eq!((1, 0),  turn2(50, 'R', 50));
        assert_eq!((1, 50),  turn2(50, 'R', 100));
        assert_eq!((10, 50),  turn2(50, 'R', 1000));


        assert_eq!((3, 60), turn2(10, 'L', 250));
        assert_eq!((3, 5), turn2(95, 'R', 210));
        assert_eq!((2, 50), turn2(50, 'R', 200));
    	assert_eq!((2, 50), turn2(50, 'L', 200));

    	// 4. Multiple wraps + remainder crossing boundary
    // 80 + 123 = 203 → wraps 2 times, ends at 3
    assert_eq!((2, 3), turn2(80, 'R', 123));

    // 5. Opposite direction version of above
    // 20 - 123 = -103 → wraps 2 times, ends at  -103 mod 100 =  -3 → 97
    assert_eq!((2, 97), turn2(20, 'L', 123));

    // 6. Massive movement but small remainder that crosses
    // 90 + 1005 = 1095 → wraps 10 times, ends at 95
    assert_eq!((10, 95), turn2(90, 'R', 1005));

    // 7. Movement that crosses exactly at a boundary
    // 99 + 101 = 200 → wraps 2 times, ends at 0
    assert_eq!((2, 0), turn2(99, 'R', 101));

    // 8. Movement that leaves final_pos same as original but wraps many times
    // 10 - 1000 = -990 → wraps 9 times, ends at 10
    // Buggy ChatGPT ...
    assert_eq!((10, 10), turn2(10, 'L', 1000));

    // 9. Zero remainder but negative direction multi-wrap
    // 5 - 300 = -295 → wraps 2 times, ends at 5
    assert_eq!((3, 5), turn2(5, 'L', 300));

    // 10. Forward overshoot + big remainder overshoot
    // 30 + 256 = 286 → wraps 2 times, ends at 86
    assert_eq!((2, 86), turn2(30, 'R', 256));
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

    #[test]
fn stress_turn2() {
    for pos in 0..100 {
        for &dir in &['L', 'R'] {
            for n in 1..1000 {
                let (t_my, p_my) = turn2(pos, dir, n);

                // Reference correct implementation
                let delta = if dir == 'L' { -n } else { n };
                let final_pos = (pos + delta).rem_euclid(100);
                let wraps = ((pos + delta) - final_pos).abs() / 100;

                assert_eq!(wraps, t_my, "wrap mismatch at pos={pos}, dir={dir}, n={n}");
                assert_eq!(final_pos, p_my, "pos mismatch at pos={pos}, dir={dir}, n={n}");
            }
        }
    }
}
}
