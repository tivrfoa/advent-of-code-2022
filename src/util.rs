use std::env;
use std::fmt::{Debug, Display};
use std::fs;

#[allow(dead_code)]
pub fn get_file_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {file_path}");
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

#[allow(dead_code)]
pub fn read_file(file_path: &str) -> String {
    println!("In file {file_path}");
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

#[allow(dead_code)]
pub fn input_as_vec_i32(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[allow(dead_code)]
pub fn input_line_len(input: &str) -> usize {
    input.lines().next().unwrap().len()
}

#[allow(dead_code)]
pub fn input_as_vec_char(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[allow(dead_code)]
pub fn get_numbers_in_line(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn parse_matrices(
    input: &str,
    rows: usize,
    lines_between_matrices: usize,
) -> Vec<Vec<Vec<i32>>> {
    let mut matrices = vec![];
    let mut curr_matrix = vec![];
    let mut skip_counter = 0;
    let mut skip = false;

    for l in input.lines() {
        if skip {
            skip_counter += 1;
            if skip_counter == lines_between_matrices {
                skip_counter = 0;
                skip = false;
            }
        } else {
            curr_matrix.push(get_numbers_in_line(l));
            if curr_matrix.len() == rows {
                matrices.push(curr_matrix);
                curr_matrix = vec![];
                skip = true;
            }
        }
    }

    matrices
}

#[allow(dead_code)]
pub fn map_matrices(
    matrices: Vec<Vec<Vec<i32>>>,
    map_fn: fn(i32) -> (i32, bool),
) -> Vec<Vec<Vec<(i32, bool)>>> {
    let mut boards = vec![];

    for m in matrices {
        let mut board = vec![];
        for r in m {
            board.push(r.into_iter().map(map_fn).collect());
        }
        boards.push(board);
    }

    boards
}

#[allow(dead_code)]
pub fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

#[allow(dead_code)]
/// sum of intergers from a + 1 to b
/// So you need to pass initial_value - 1 for it to be
/// included, eg:
///
/// 1 to 3 -> sum_of_consecutive_numbers(0, 3)
///
/// https://math.stackexchange.com/questions/1100897/sum-of-consecutive-numbers
pub fn sum_of_consecutive_numbers(a: u32, b: u32) -> u32 {
    ((b * (b + 1)) / 2) - ((a * (a + 1)) / 2)
}
