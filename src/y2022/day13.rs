/*
It requires distinct types and populate it using recursion.

The solution below is basically the same from Uncle Scientist:
https://www.youtube.com/watch?v=CMhq3M-HE0I

I took most ideas from him.

*/
use crate::util;

use std::cmp::Ordering;
use std::str::Chars;

const TEN: u8 = 58;

#[derive(Debug, PartialEq)]
enum Val {
    Num(u8),
    List(Vec<Val>),
}

impl Val {
    fn new_divider_packet(n: u8) -> Self {
        use Val::*;

        List(vec![List(vec![Num(n)])])
    }

    fn is_right_order(&self, other: &Val) -> bool {
        self.compare(other) != Ordering::Greater
    }

    fn compare(&self, other: &Val) -> Ordering {
        match (self, other) {
            (Val::Num(a), Val::Num(b)) => {
                if a < b {
                    return Ordering::Less;
                }
                if a > b {
                    return Ordering::Greater;
                }
            }
            (Val::Num(a), Val::List(_)) => {
                let ret = Val::List(vec![Val::Num(*a)]).compare(other);
                if ret != Ordering::Equal {
                    return ret;
                }
            }
            (Val::List(_), Val::Num(b)) => {
                let ret = self.compare(&Val::List(vec![Val::Num(*b)]));
                if ret != Ordering::Equal {
                    return ret;
                }
            }
            (Val::List(la), Val::List(lb)) => {
                for i in 0..la.len() {
                    if i >= lb.len() {
                        return Ordering::Greater;
                    }
                    let ret = la[i].compare(&lb[i]);
                    if ret != Ordering::Equal {
                        return ret;
                    }
                }
                if la.len() < lb.len() {
                    return Ordering::Less;
                }
            }
        }

        Ordering::Equal
    }

    fn parse_chars(mut chars: Chars) -> Self {
        // ignore first '['
        chars.next();

        Val::List(Self::parse(&mut chars))
    }

    fn parse(chars: &mut Chars) -> Vec<Val> {
        let mut result: Vec<Val> = vec![];
        let mut num = u8::MAX;

        while let Some(c) = chars.next() {
            match c {
                '[' => {
                    result.push(Val::List(Self::parse(chars)));
                }
                ']' => {
                    if num != u8::MAX {
                        result.push(Val::Num(num));
                    }
                    return result;
                }
                ',' => {
                    if num != u8::MAX {
                        result.push(Val::Num(num));
                        num = u8::MAX;
                    }
                }
                _ => {
                    // number
                    if num != u8::MAX {
                        num = TEN;
                    } else {
                        num = c as u8;
                    }
                }
            }
        }

        result
    }
}

#[derive(Debug)]
struct Pair {
    a: Val,
    b: Val,
}

impl Pair {
    fn new(a: Val, b: Val) -> Self {
        Self { a, b }
    }

    fn is_right_order(&self) -> bool {
        self.a.is_right_order(&self.b)
    }
}

pub fn solve(input: String) -> usize {
    let mut ans = 0;
    let mut pairs: Vec<Pair> = vec![];
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

    for line in lines.chunks(2) {
        let a = Val::parse_chars(line[0].chars());
        let b = Val::parse_chars(line[1].chars());
        pairs.push(Pair::new(a, b));
    }

    for pair in &pairs {
        println!("{pair:?}");
    }

    for (i, pair) in pairs.iter().enumerate() {
        if pair.is_right_order() {
            ans += i + 1;
            println!("index {} is good", i + 1);
        }
    }

    ans
}

pub fn solve_part2(input: String) -> usize {
    let dp2 = Val::new_divider_packet(b'2');
    let dp6 = Val::new_divider_packet(b'6');
    let mut packets: Vec<Val> = vec![Val::new_divider_packet(b'2'), Val::new_divider_packet(b'6')];

    for line in input.lines().filter(|l| !l.is_empty()) {
        packets.push(Val::parse_chars(line.chars()));
    }

    packets.sort_by(|a, b| a.compare(b));

    // for p in &packets {
    //     println!("{p:?}");
    // }

    let (mut idx1, mut idx2) = (0, 0);
    for (i, packet) in packets.into_iter().enumerate() {
        if packet == dp2 {
            idx1 = i + 1;
        } else if packet == dp6 {
            idx2 = i + 1;
        }
    }

    idx1 * idx2
}

#[allow(dead_code)]
fn dbg(grid: &Vec<Vec<u8>>) {
    for item in grid {
        println!("{:?}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2022/day13-sample.txt");
        assert_eq!(13, solve(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2022/day13.txt");
        assert_eq!(5529, solve(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2022/day13-sample.txt");
        assert_eq!(140, solve_part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2022/day13.txt");
        assert_eq!(27690, solve_part2(input));
    }
}
