use crate::util;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

/*


probe with any integer velocity in the x (forward) and
y (upward, or downward if negative)

(x, y)

target area: x=20..30, y=-10..-5

1) 7, 2

x postion after each step:

7 + 6 + 5 + 4 = 22

y position after each step:

2 + 1 + 0 + -1 = 2

2 + 1 + 0 + -1 + -2 + -3 = -3

2 + 1 + 0 + -1 + -2 + -3 + -4 = -7 -> you need 7 steps

2 + 1 + 0 + -1 + -2 + -3 + -4 + -5 = -12 -> outside target area

for y, we just need to make sure that it goes down on time.

Why can't y be huge value? Because it would pass through the target area.

Maybe one approach:
  1. find x velocities range that reach the target, and store at which step
  that's going to happen.
  2. check highest y possible after that many steps.

I can do binary search to find an x that goes into the target area


1) 6, 2

6 + 5 + 4 + 3 + 2 + 1 = 21 in 6 steps

y:
0 -> 2 -> 3 -> 3 -> 2 -> 0 -> -3 -> -7
*/

fn calc_y_pos_after_n_steps(mut y: i32, steps: u32) -> i32 {
    let mut dest = 0;
    for _ in 0..steps {
        dest += y;
        y -= 1;
    }
    dest
}

fn part1(min_x: u32, max_x: u32, min_y: i32, max_y: i32) -> String {
    let mut lo = 1;
    let mut hi = max_x;
    while util::sum_of_consecutive_numbers(1, lo) < min_x {
        lo += 1;
    }
    while util::sum_of_consecutive_numbers(1, hi) > max_x {
        hi -= 1;
    }
    dbg!(lo, hi);

    let mut steps = lo;
    let mut y = 300;
    let mut ans = 0;

    loop {
        let y_dest = calc_y_pos_after_n_steps(y, steps);
        println!("y {} after {} steps -> {}", y, steps, y_dest);
        if y_dest >= min_y && y_dest <= max_y {
            println!("It found an possible answer: {}", y);
            if y > ans {
                ans = y;
            }
            y = 100;
            steps += 1;
            continue;
        } else {
            if y_dest < min_y {
                steps += 1;
                y = 100;
                continue;
            }
        }
        y -= 1;
        if y == 0 {
            steps += 1;
            y = 100;
        }
        if steps > hi {
            break;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        // let input = util::read_file("inputs/2021/day17-sample.txt");
        assert_eq!("9", part1(20, 30, -10, -5));
    }

    #[test]
    fn p1() {
        // let input = util::read_file("inputs/2021/day17.txt");
        assert_eq!("", part1(119, 176, -141, -84));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day17-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day17.txt");
        assert_eq!("", part2(input));
    }
}
