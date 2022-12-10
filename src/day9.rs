use crate::util;

use std::collections::HashSet;

/// U, D, L, R
const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_dir(c: char) -> (i32, i32) {
    match c {
        'U' => DIR[0],
        'D' => DIR[1],
        'L' => DIR[2],
        'R' => DIR[3],
        _ => panic!("Invalid direction: {c}"),
    }
}

///
/// Apply the rules as in the problem statement:
/// 1. If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough.
/// 2. Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up.
///
/// These rules were made clear watching this great video:
/// https://www.youtube.com/watch?v=FU4fCTWauq0
///
/// It's also from there that I got the signum idea.
fn move_dir(dir: char, seg: &mut Vec<(i32, i32)>) {
    let (row_step, col_step) = get_dir(dir);

    seg[0].0 += row_step;
    seg[0].1 += col_step;
    let row_diff = seg[0].0 - seg[1].0;
    let col_diff = seg[0].1 - seg[1].1;

    if row_diff == 0 && col_diff.abs() > 1 {
        seg[1].1 += col_diff.signum();
    } else if col_diff == 0 && row_diff.abs() > 1 {
        seg[1].0 += row_diff.signum();
    } else if row_diff.abs() > 1 || col_diff.abs() > 1 {
        seg[1].0 += row_diff.signum();
        seg[1].1 += col_diff.signum();
    } else {
        // T does not move: row_diff == 0 && col_diff == 0
    }
}

pub fn solve(input: String) -> u32 {
    let mut ans = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut seg: Vec<(i32, i32)> = vec![(0, 0), (0, 0)];
    visited.insert((0, 0));
    ans += 1;

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        let dir = tokens[0].chars().next().unwrap();
        let steps: usize = tokens[1].parse().unwrap();

        for _ in 0..steps {
            move_dir(dir, &mut seg);
            ans += if visited.insert(seg[1].clone()) { 1 } else { 0 };
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/sample-day9.txt");
        assert_eq!(13, solve(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/input-day9.txt");
        assert_eq!(6522, solve(input));
    }

    // #[test]
    // fn part2_sample() {
    // 	let input = util::read_file("inputs/sample-day9.txt");
    // 	assert_eq!(8, solve(input));
    // }
}
