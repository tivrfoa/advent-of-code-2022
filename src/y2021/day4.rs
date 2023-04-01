use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

use crate::aoc::AOC;

fn mark_boards(boards: &mut Vec<Vec<Vec<(i32, bool)>>>, n: i32) {
    for b in 0..boards.len() {
        for r in 0..5 {
            for c in 0..5 {
                if boards[b][r][c].0 == n {
                    boards[b][r][c].1 = true;
                }
            }
        }
    }
}

fn get_sum(board: &Vec<Vec<(i32, bool)>>) -> i32 {
    println!("Winner board:");
    for y in 0..5 {
        println!("{:?}", board[y]);
    }
    let mut sum = 0;
    for r in 0..5 {
        for c in 0..5 {
            if !board[r][c].1 {
                sum += board[r][c].0;
            }
        }
    }
    sum
}

fn get_winner(boards: &Vec<Vec<Vec<(i32, bool)>>>) -> Option<i32> {
    for b in 0..boards.len() {
        // check rows
        'c:
        for c in 0..5 {
            for r in 0..5 {
                if boards[b][r][c].1 == false {
                    continue 'c;
                }
            }
            return Some(get_sum(&boards[b]));
        }

        // check cols
        'r:
        for r in 0..5 {
            for c in 0..5 {
                if boards[b][r][c].1 == false {
                    continue 'r;
                }
            }
            return Some(get_sum(&boards[b]));
        }
    }
    None
}

fn read_boards(input: &str) -> Vec<Vec<Vec<(i32, bool)>>> {
    let mut boards = vec![];
    let cols = 5;
    let rows = 5;
    let mut curr_board = vec![vec![(0, false); 5]; 5];
    let mut r = 0;
    for l in input.lines() {
        if l.is_empty() {
            if r > 0 {
                boards.push(curr_board);
                curr_board = vec![vec![(0, false); 5]; 5];
            }
            r = 0;
        } else {
            let nums = util::get_numbers_in_line(l);
            for (c, n) in nums.into_iter().enumerate() {
                curr_board[r][c] = (n, false);
            }
            r += 1;
        }
    }
    if r == 5 {
        boards.push(curr_board);
    }
    boards
}

fn part1(input: String, bingo_nums: Vec<i32>) -> String {
    let mut boards = read_boards(&input);
    for n in bingo_nums {
        mark_boards(&mut boards, n);
        if let Some(sum) = get_winner(&boards) {
            dbg!(&sum, &n);
            return (sum * n).to_string();
        }
    }
    panic!("Nobody won!");
}

fn part2(input: String) -> String {
    todo!()
}

fn parse(input: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2021/day4-sample.txt");
        let bingo_nums = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
        assert_eq!("4512", part1(input, bingo_nums));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2021/day4.txt");
        let bingo_nums = vec![6,69,28,50,36,84,49,13,48,90,1,33,71,0,94,59,53,58,60,96,30,34,29,91,11,41,77,95,17,80,85,93,7,9,74,89,18,25,26,8,87,38,68,5,12,43,27,46,62,73,16,55,22,4,65,76,54,52,83,10,21,67,15,47,45,40,35,66,79,51,75,39,64,24,37,72,3,44,82,32,78,63,57,2,86,31,19,92,14,97,20,56,88,81,70,61,42,99,23,98];
        assert_eq!("71708", part1(input, bingo_nums));
    }

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/2021/day4-sample.txt");
    //    assert_eq!("", part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/2021/day4.txt");
    //    assert_eq!("", part2(input));
    //}
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}
