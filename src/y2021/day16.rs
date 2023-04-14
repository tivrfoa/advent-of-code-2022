use crate::util;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

fn part1(input: String) -> String {
    let map: HashMap<char, &str> = HashMap::from_iter([
    ('0', "0000"),
    ('1', "0001"),
    ('2', "0010"),
    ('3', "0011"),
    ('4', "0100"),
    ('5', "0101"),
    ('6', "0110"),
    ('7', "0111"),
    ('8', "1000"),
    ('9', "1001"),
    ('A', "1010"),
    ('B', "1011"),
    ('C', "1100"),
    ('D', "1101"),
    ('E', "1110"),
    ('F', "1111"),
    ]);

    let mut binary = String::new();

    for line in input.lines() {
        for c in line.chars() {
            binary.push_str(map[&c]);
        }
    }

    // check where zero starts at the end
    let zeros_start_pos = {
        let mut qt = 0;
        for c in binary.chars().rev() {
            if c != '0' { break; }
            qt += 1;
        }
        binary.len() - qt
    };

    let len = binary.len();
    let mut pos = 0;
    let mut ans = 0;

    'outer:
    while pos < zeros_start_pos {
        let packet_version = to_u32(&binary[pos..pos+3]);
        ans += packet_version;
        pos += 3;
        let packet_id = to_u8(&binary[pos..pos+3]);
        pos += 3;

        if packet_id == 4 {
            let mut number = String::new();
            loop {
                number.push_str(&binary[pos+1..pos+5]);
                if &binary[pos..pos+1] == "0" {
                    pos += 5;
                    break;
                }
                pos += 5;
            }
        } else {
            let type_id = &binary[pos..pos+1];
            pos += 1;
            if type_id == "0" {
                // 15-bit number representing the number of bits in the sub-packets.
                let subpackets_len = to_u32(&binary[pos..pos+15]);
                pos += 15;
            } else {
                let qt_subpackets = to_u32(&binary[pos..pos+11]);
                pos += 11;
            }
        }
    }

    ans.to_string()
}

fn part2(input: String) -> String {
    "".into()
}

#[allow(dead_code)]
fn dbg_grid<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

#[allow(dead_code)]
fn in_to_nums<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: Debug,
{
    input.split(',').map(|n| n.parse::<T>().unwrap()).collect()
}

#[allow(dead_code)]
fn split_str_to_nums<T: std::str::FromStr>(input: &str, separator: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: Debug,
{
    input
        .split(separator)
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}

#[allow(dead_code)]
fn vec_max<T: std::str::FromStr + std::cmp::Ord + Copy>(vec: &[T]) -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    *vec.iter().max().unwrap()
}

#[allow(dead_code)]
fn vec_min<T: std::str::FromStr + std::cmp::Ord + Copy>(vec: &[T]) -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    *vec.iter().min().unwrap()
}

#[allow(dead_code)]
fn str_to_char_tuple(s: &str) -> (char, char) {
    (s[0..1].chars().next().unwrap(), s[1..2].chars().next().unwrap())
}

#[allow(dead_code)]
trait MapAddOrInsert<K, V> {
    fn add_or_insert(&mut self, k: K, v: V);
}

#[allow(dead_code)]
impl<K: Eq + Hash, V: std::ops::AddAssign + Copy> MapAddOrInsert<K, V> for HashMap<K, V> {
    fn add_or_insert(&mut self, k: K, v: V) {
        self.entry(k).and_modify(|qt| *qt += v).or_insert(v);
    }
}

#[allow(dead_code)]
fn get_dirs(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 4] {
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
fn get_dirs_with_diagonals(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 8] {
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
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn to_u8(s: &str) -> u8 {
    u8::from_str_radix(s, 2).unwrap()
}

#[allow(dead_code)]
fn to_u16(s: &str) -> u16 {
    u16::from_str_radix(s, 2).unwrap()
}

#[allow(dead_code)]
fn to_u32(s: &str) -> u32 {
    u32::from_str_radix(s, 2).unwrap()
}

#[allow(dead_code)]
fn to_u64(s: &str) -> u64 {
    u64::from_str_radix(s, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = util::read_file("inputs/2021/day16-sample.txt");
        assert_eq!("12", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day16.txt");
        assert_eq!("917", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day16-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day16.txt");
        assert_eq!("", part2(input));
    }
}
