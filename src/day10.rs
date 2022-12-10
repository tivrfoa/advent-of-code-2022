use crate::util;

// cycles % 40 == 20
pub fn solve(input: String) -> i32 {
    let mut ans = 0;

    let mut cycles = 0;
    let mut x: i32 = 1;
    for line in input.lines() {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        let op = tokens[0];
        if op == "noop" {
            cycles += 1;
            ans += if cycles % 40 == 20 { x * cycles } else { 0 };
        } else {
            cycles += 1;
            ans += if cycles % 40 == 20 { x * cycles } else { 0 };
            cycles += 1;
            ans += if cycles % 40 == 20 { x * cycles } else { 0 };
            x += tokens[1].parse::<i32>().unwrap();
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/sample-day10.txt");
        assert_eq!(13140, solve(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/input-day10.txt");
        assert_eq!(14060, solve(input));
    }

    // #[test]
    // fn part2_input() {
    //     let input = util::read_file("inputs/input-day9.txt");
    //     assert_eq!(2717, solve(input, 10));
    // }
}
