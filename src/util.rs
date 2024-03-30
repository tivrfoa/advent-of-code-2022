use std::env;
use std::fmt::{Debug, Display};
use std::fs;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::iter::zip;

use num::Integer;

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

// slice must start after opening paren '('
#[allow(dead_code)]
pub fn find_close_paren(chars: &[char]) -> usize {
    let mut qt = 0;

    for i in 0..chars.len() {
        if chars[i] == ')' {
            if qt == 0 {
                return i;
            }
            qt -= 1;
        } else if chars[i] == '(' {
            qt += 1;
        }
    }
    panic!("Failed to find close parentheses");
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
pub fn dbg_grid(grid: &[Vec<char>]) {
    println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&");
    for item in grid {
        println!("{}", item.iter().collect::<String>());
    }
    println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&");
}

#[allow(dead_code)]
pub fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&");
    for item in grid {
        println!("{item:?}");
    }
    println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&");
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
pub fn print_vec_inline<T: std::str::FromStr + std::fmt::Display>(vec: &[T])
where
    <T as std::str::FromStr>::Err: Debug,
{
    for i in 0..vec.len() {
        print!("{}, ", vec[i]);
    }
    println!();
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
    (
        s[0..1].chars().next().unwrap(),
        s[1..2].chars().next().unwrap(),
    )
}

/// Set bit at pos with value 0
#[allow(dead_code)]
pub fn set_zero_at(n: usize, pos: usize) -> usize {
    n & !(1 << pos)
}

/// Set bit at pos with value 1
#[allow(dead_code)]
pub fn set_one_at(n: usize, pos: usize) -> usize {
    n | (1 << pos)
}

/// Set bit at pos with value 0
#[allow(dead_code)]
pub fn u32_set_zero_at(n: u32, pos: u32) -> u32 {
    n & !(1 << pos)
}

/// Set bit at pos with value 1
#[allow(dead_code)]
pub fn u32_set_one_at(n: u32, pos: u32) -> u32 {
    n | (1 << pos)
}


// trait Between {
//     fn is_between(&self, l: Integer, r: impl Integer) -> bool
//             where Self: Integer {
//         &l <= self && self <= &r
//     }
// }

pub fn is_between<I: Integer>(n: I, l: I, r: I) -> bool{
    l <= n && n <= r
}

#[allow(dead_code)]
pub trait CharAsNum {
    fn asu32(self) -> u32;
    fn asu64(self) -> u64;
    fn to_decimal(self) -> usize;
}

impl CharAsNum for char {
    fn asu32(self) -> u32 {
        self as u32 - '0' as u32
    }
    fn asu64(self) -> u64 {
        self.asu32() as u64
    }

    fn to_decimal(self) -> usize {
        self.asu32() as usize
    }
}

#[allow(dead_code)]
pub trait MapAddOrInsert<K, V> {
    fn add_or_insert(&mut self, k: K, v: V);
    fn count(&self, n: V) -> usize;
}

#[allow(dead_code)]
impl<K: Eq + Hash, V: std::ops::AddAssign + Copy + std::cmp::PartialEq> MapAddOrInsert<K, V>
    for HashMap<K, V>
{
    fn add_or_insert(&mut self, k: K, v: V) {
        self.entry(k).and_modify(|qt| *qt += v).or_insert(v);
    }
    fn count(&self, n: V) -> usize {
        self.iter().filter(|(_, &v)| v == n).count()
    }
}

#[allow(dead_code)]
pub trait GroupBy<K> {
    fn grouping_by_ref(&self) -> HashMap<&K, usize>;
    fn grouping_by(&self) -> HashMap<K, usize>;
}

#[allow(dead_code)]
impl<K: Copy + Eq + Hash> GroupBy<K> for Vec<K> {
    fn grouping_by_ref(&self) -> HashMap<&K, usize> {
        let mut map: HashMap<&K, usize> = HashMap::new();
        if self.is_empty() {
            return map;
        }
        for v in self {
            map.entry(v).and_modify(|qt| *qt += 1).or_insert(1);
        }

        map
    }

    fn grouping_by(&self) -> HashMap<K, usize> {
        let mut map: HashMap<K, usize> = HashMap::new();
        if self.is_empty() {
            return map;
        }
        for v in self {
            map.entry(*v).and_modify(|qt| *qt += 1).or_insert(1);
        }

        map
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
pub fn get_dirs_with_diagonals(
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
) -> [(bool, (usize, usize)); 8] {
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

#[allow(dead_code)]
pub fn get_dirs_with_diagonals_i32(
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
) -> [(bool, (i32, i32)); 8] {
    [
        (c > 0, (0, -1)),                 // left
        (c + 1 < cols, (0, 1)),           // right
        (r > 0, (-1, 0)),                 // top
        (r + 1 < rows, (1, 0)),           // bottom
        (r > 0 && c > 0, (-1, -1)),       // top left
        (r > 0 && c + 1 < cols, (-1, 1)), // top right
        (r + 1 < rows && c > 0, (1, -1)),
        (r + 1 < rows && c + 1 < cols, (1, 1)),
    ]
}

#[allow(dead_code)]
pub fn move_while(
    grid: &[Vec<char>],
    mut r: i32,
    mut c: i32,
    rd: i32,
    cd: i32,
    t: char,
) -> (usize, usize) {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    while grid[r as usize][c as usize] == t
        && r + rd >= 0
        && r + rd < rows
        && c + cd >= 0
        && c + cd < cols
    {
        r += rd;
        c += cd;
    }
    (r as usize, c as usize)
}

pub trait ParseInput {
    fn to_num_vec<T: std::str::FromStr>(input: &str) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: Debug,
    {
        input.split(',').map(|n| n.parse::<T>().unwrap()).collect()
    }

    fn as_char(&self) -> char;
    fn is_in(&self, l: &str, r: &str) -> bool;
    fn left(&self, delim: char) -> &str;
    fn split_delim(&self, delim: char) -> (&str, &str);
    fn split_space(&self) -> Vec<&str>;
    fn split_to_nums<T: std::str::FromStr>(&self, delim: char) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: Debug;
    fn to_char_grid(&self) -> Vec<Vec<char>>;
    fn to_nums<T: std::str::FromStr>(&self) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: Debug;
}

impl ParseInput for str {
    fn as_char(&self) -> char {
        self.chars().next().unwrap()
    }
    fn is_in(&self, l: &str, r: &str) -> bool {
        l <= self && self <= r
    }
    fn left(&self, delim: char) -> &str {
        self.split_once(delim).unwrap().0
    }
    fn split_delim(&self, delimiter: char) -> (&str, &str) {
        self.split_once(delimiter).unwrap()
    }
    fn split_space(&self) -> Vec<&str> {
        self.split_ascii_whitespace().collect()
    }
    fn split_to_nums<T: std::str::FromStr>(&self, delim: char) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: Debug,
    {
        self.split(delim).map(|n| n.parse::<T>().unwrap()).collect()
    }
    fn to_char_grid(&self) -> Vec<Vec<char>> {
        self.lines().map(|l| l.chars().collect()).collect()
    }
    fn to_nums<T: std::str::FromStr>(&self) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: Debug,
    {
        self.lines().map(|n| n.parse::<T>().unwrap()).collect()
    }
}

#[allow(dead_code)]
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[allow(dead_code)]
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}
