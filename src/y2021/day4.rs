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

fn get_winners(boards: &Vec<Vec<Vec<(i32, bool)>>>) -> Vec<usize> {
    let mut ret = vec![];

    'b:
    for b in 0..boards.len() {
        // check rows
        'c:
        for c in 0..5 {
            for r in 0..5 {
                if boards[b][r][c].1 == false {
                    continue 'c;
                }
            }
            ret.push(b);
            continue 'b;
        }

        // check cols
        'r:
        for r in 0..5 {
            for c in 0..5 {
                if boards[b][r][c].1 == false {
                    continue 'r;
                }
            }
            ret.push(b);
            continue 'b;
        }
    }
    ret
}

fn part1(input: String, bingo_nums: Vec<i32>) -> String {
    let mut boards = read_boards(&input);
    for n in bingo_nums {
        mark_boards(&mut boards, n);
        let winners = get_winners(&boards);
        if !winners.is_empty() {
            let sum = get_sum(&boards[winners[0]]);
            dbg!(&sum, &n);
            return (sum * n).to_string();
        }
    }
    panic!("Nobody won!");
}

fn part2(input: String, bingo_nums: Vec<i32>) -> String {
    let mut boards = read_boards(&input);
    let number_of_boards = boards.len();
    let mut win_count = 0;
    for n in bingo_nums {
        mark_boards(&mut boards, n);
        let winners = get_winners(&boards);
        let mut decre = 0;
        for winner in winners {
            let board_index = winner - decre;
            let sum = get_sum(&boards[board_index]);
            win_count += 1;
            if win_count == number_of_boards {
                return (sum * n).to_string();
            } else {
                boards.remove(board_index);
                decre += 1;
            }
        }
    }
    panic!("Nobody won?!");
}

fn read_boards(input: &str) -> Vec<Vec<Vec<(i32, bool)>>> {
    let mut matrices = util::parse_matrices(input, 5, 1);
    util::map_matrices(matrices, |v| (v, false))
}

#[allow(dead_code)]
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

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2021/day4-sample.txt");
        let bingo_nums = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
        assert_eq!("1924", part2(input, bingo_nums));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2021/day4.txt");
        let bingo_nums = vec![6,69,28,50,36,84,49,13,48,90,1,33,71,0,94,59,53,58,60,96,30,34,29,91,11,41,77,95,17,80,85,93,7,9,74,89,18,25,26,8,87,38,68,5,12,43,27,46,62,73,16,55,22,4,65,76,54,52,83,10,21,67,15,47,45,40,35,66,79,51,75,39,64,24,37,72,3,44,82,32,78,63,57,2,86,31,19,92,14,97,20,56,88,81,70,61,42,99,23,98];
        assert_eq!("34726", part2(input, bingo_nums));
    }
}

