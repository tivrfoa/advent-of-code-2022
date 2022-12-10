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
fn move_dir(dir: char, hr: i32, hc: i32, tr: i32, tc: i32) -> (i32, i32, i32, i32) {
    let (row_step, col_step) = get_dir(dir);

    let new_hr = hr + row_step;
    let new_hc = hc + col_step;
    let row_diff = new_hr - tr;
    let col_diff = new_hc - tc;

    if row_diff == 0 && col_diff.abs() > 1 {
        (new_hr, new_hc, tr, tc + col_diff.signum())
    } else if col_diff == 0 && row_diff.abs() > 1 {
        (new_hr, new_hc, tr + row_diff.signum(), tc)
    } else if row_diff.abs() > 1 || col_diff.abs() > 1 {
        (
            new_hr,
            new_hc,
            tr + row_diff.signum(),
            tc + col_diff.signum(),
        )
    } else {
        // T does not move: row_diff == 0 && col_diff == 0
        (new_hr, new_hc, tr, tc)
    }
}

pub fn solve(input: String) -> u32 {
    let mut ans = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut hr = 0;
    let mut hc = 0;
    let mut tr = 0;
    let mut tc = 0;
    visited.insert((0, 0));
    ans += 1;

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        let dir = tokens[0].chars().next().unwrap();
        let steps: usize = tokens[1].parse().unwrap();

        for _ in 0..steps {
            let (new_hr, new_hc, new_tr, new_tc) = move_dir(dir, hr, hc, tr, tc);
            hr = new_hr;
            hc = new_hc;
            tr = new_tr;
            tc = new_tc;
            ans += if visited.insert((tr, tc)) { 1 } else { 0 };
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
