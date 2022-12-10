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

pub fn solve_part2(input: String) -> Vec<String> {
    let mut cycles = 0;
    let mut x: i32 = 1;
    let mut crt: Vec<String> = vec![];
    let mut crt_line = String::new();
    for line in input.lines() {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        let op = tokens[0];

        if cycles == x || cycles + 1 == x || cycles - 1 == x {
            crt_line.push('#');
        } else {
            crt_line.push('.');
        }

        if op == "noop" {
            cycles += 1;
        } else {
            cycles += 1;
            if cycles == x || cycles + 1 == x || cycles - 1 == x {
                crt_line.push('#');
            } else {
                crt_line.push('.');
            }
            if cycles == 40 {
                cycles = 0;
                crt.push(crt_line);
                crt_line = String::new();
            }
            cycles += 1;
            x += tokens[1].parse::<i32>().unwrap();
        }
        if cycles == 40 {
            cycles = 0;
            crt.push(crt_line);
            crt_line = String::new();
        }
    }

    for i in 0..crt.len() {
        println!("{}", crt[i]);
    }

    crt
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

    #[test]
    fn part2_sample() {
        let expected: Vec<String> = vec![
            "##..##..##..##..##..##..##..##..##..##..".into(),
            "###...###...###...###...###...###...###.".into(),
            "####....####....####....####....####....".into(),
            "#####.....#####.....#####.....#####.....".into(),
            "######......######......######......####".into(),
            "#######.......#######.......#######.....".into(),
        ];
        let input = util::read_file("inputs/sample-day10.txt");
        assert_eq!(expected, solve_part2(input));
    }

    #[test]
    fn part2_input() {
        let expected: Vec<String> = vec![
            "##..##..##..##..##..##..##..##..##..##..".into(),
            "###...###...###...###...###...###...###.".into(),
            "####....####....####....####....####....".into(),
            "#####.....#####.....#####.....#####.....".into(),
            "######......######......######......####".into(),
            "#######.......#######.......#######.....".into(),
        ];
        let input = util::read_file("inputs/input-day10.txt");
        assert_eq!(expected, solve_part2(input));
    }
}
