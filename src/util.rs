use std::env;
use std::fmt::{Debug, Display};
use std::fs;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::iter::zip;

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
pub fn split_space(input: &str) -> Vec<&str> {
	input.split_ascii_whitespace().collect()
}

#[allow(dead_code)]
pub fn split_once(input: &str, delimiter: char) -> (&str, &str) {
	input.split_once(delimiter).unwrap()
}

#[allow(dead_code)]
pub fn split_once_i32(input: &str, delimiter: char) -> (i32, i32) {
	let (a, b) = input.split_once(delimiter).unwrap();
	(a.parse().unwrap(), b.parse().unwrap())
}

#[allow(dead_code)]
pub fn split_once_usize(input: &str, delimiter: char) -> (usize, usize) {
	let (a, b) = input.split_once(delimiter).unwrap();
	(a.parse().unwrap(), b.parse().unwrap())
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
/// sum of intergers from a to b
///
/// https://math.stackexchange.com/questions/1100897/sum-of-consecutive-numbers
pub fn sum_of_consecutive_numbers(a: u32, b: u32) -> u32 {
    // decrement 1 from initial value because formula is: a + 1 to b
    if a == 0 {
        panic!("Initial value must be at least 1.");
    }
    let a = a - 1;
    ((b * (b + 1)) / 2) - ((a * (a + 1)) / 2)
}

#[allow(dead_code)]
pub fn dbg_grid<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

#[allow(dead_code)]
pub fn in_to_nums<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: Debug,
{
    input.split(',').map(|n| n.parse::<T>().unwrap()).collect()
}

#[allow(dead_code)]
pub fn split_str_to_nums<T: std::str::FromStr>(input: &str, separator: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: Debug,
{
    input
        .split(separator)
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn vec_max<T: std::str::FromStr + std::cmp::Ord + Copy>(vec: &[T]) -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    *vec.iter().max().unwrap()
}

#[allow(dead_code)]
pub fn vec_min<T: std::str::FromStr + std::cmp::Ord + Copy>(vec: &[T]) -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    *vec.iter().min().unwrap()
}

#[allow(dead_code)]
pub fn str_to_char_tuple(s: &str) -> (char, char) {
    (s[0..1].chars().next().unwrap(), s[1..2].chars().next().unwrap())
}

#[allow(dead_code)]
pub trait MapAddOrInsert<K, V> {
    fn add_or_insert(&mut self, k: K, v: V);
}

#[allow(dead_code)]
impl<K: Eq + Hash, V: std::ops::AddAssign + Copy> MapAddOrInsert<K, V> for HashMap<K, V> {
    fn add_or_insert(&mut self, k: K, v: V) {
        self.entry(k).and_modify(|qt| *qt += v).or_insert(v);
    }
}

#[allow(dead_code)]
pub fn get_dirs(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 4] {
    [
        // left
        (c > 0, (r, if c > 0 { c - 1 } else { 0 })),
        // right
        (c < cols - 1, (r, c + 1)),
        // top
        (r > 0, (if r > 0 { r - 1 } else { 0 }, c)),
        // bottom
        (r < rows - 1, (r + 1, c)),
    ]
}

#[allow(dead_code)]
pub fn get_dirs_with_diagonals(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 8] {
    [
        // left
        (c > 0, (r, if c > 0 { c - 1 } else { 0 })),
        // right
        (c < cols - 1, (r, c + 1)),
        // top
        (r > 0, (if r > 0 { r - 1 } else { 0 }, c)),
        // bottom
        (r < rows - 1, (r + 1, c)),
        // top left
        (
            r > 0 && c > 0,
            (if r > 0 { r - 1 } else { 0 }, if c > 0 { c - 1 } else { 0 }),
        ),
        // top right
        (
            r > 0 && c < cols - 1,
            (if r > 0 { r - 1 } else { 0 }, c + 1),
        ),
        // bottom left
        (
            r < rows - 1 && c > 0,
            (r + 1, if c > 0 { c - 1 } else { 0 }),
        ),
        // bottom right
        (r < rows - 1 && c < cols - 1, (r + 1, c + 1)),
    ]
}
