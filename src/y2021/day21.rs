use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

#[inline(always)]
fn roll(mut dice: u32) -> u32 {
	dice += 1;
	if dice > 100 {
		dice - 100
	} else {
		dice
	}
}

#[inline(always)]
fn play_round(mut pos: u32, mut dice: u32) -> (u32, u32) {
	let mut sum = 0;
	for _ in 0..3 {
		sum += dice;
		dice = dice % 100 + 1;
	}
	((pos + sum - 1) % 10 + 1, dice)
}

fn part1(mut pos1: u32, mut pos2: u32) -> String {
	let mut dice = 1;
	let mut p1 = 0;
	let mut p2 = 0;
	let mut times = 0;
	loop {
		(pos1, dice) = play_round(pos1, dice);
		times += 3;
		p1 += pos1;

		if p1 >= 1000 {
			dbg!(p1, p2, times);
			return (p2 * times).to_string();
		}

		(pos2, dice) = play_round(pos2, dice);
		times += 3;
		p2 += pos2;

		if p2 >= 1000 {
			dbg!(p1, p2, times);
			return (p1 * times).to_string();
		}
	}
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Player {
	pos: u64,
	score: u64,
}

impl Player {
	fn new(pos: u64, score: u64) -> Self {
		Self {
			pos,
			score,
		}
	}
}

const GOAL: u64 = 21;
// const SUMS: [u64; 7] = [
// 	3,
//     4,
//     5,
//     6,
//     7,
//     8,
//     9,
// ];
const SUMS: [u64; 27] = sums();

const fn sums() -> [u64; 27] {
	let mut ret = [0; 27];
	let mut idx = 0;
	let mut a = 1;
	let mut b = 1;
	let mut c = 1;
	while a < 4 {
		while b < 4 {
			while c < 4 {
				let s = a + b + c;
				ret[idx] = s;
				idx += 1;
				c += 1;
			}
			c = 1;
			b += 1;
		}
		b = 1;
		a += 1;
	}
	ret
}

fn solvep2(mem: &mut HashMap<(Player, Player), (u64, u64)>, p1: &Player, p2: &Player) -> (u64, u64) {
	if let Some(qt) = mem.get(&(p1.clone(), p2.clone())) {
		return *qt;
	}
	let mut p1_won: u64 = 0;
	let mut p2_won: u64 = 0;
	let mut p1s = vec![];

	for s in SUMS {
		let mut new_pos = p1.pos + s;
		if new_pos > 10 {
			new_pos -= 10;
		}
		if p1.score + new_pos >= GOAL {
			p1_won += 1;
		} else {
			p1s.push(Player::new(new_pos, p1.score + new_pos));
		}
	}

	for p in &p1s {
		for s in SUMS {
			let mut new_pos = p2.pos + s;
			if new_pos > 10 {
				new_pos -= 10;
			}
			if p2.score + new_pos >= GOAL {
				p2_won += 1;
			} else {
				let new_p2 = Player::new(new_pos, p2.score + new_pos);
				let (t1, t2) = solvep2(mem, p, &new_p2);
				p1_won += t1;
				p2_won += t2;
			}
		}
	}

	mem.insert((p1.clone(), p2.clone()), (p1_won, p2_won));
	(p1_won, p2_won)
}

fn part2(mut pos1: u64, mut pos2: u64) -> String {
	let mut mem: HashMap<(Player, Player), (u64, u64)> = HashMap::new();
	let p1 = Player::new(pos1, 0);
	let p2 = Player::new(pos2, 0);
	let (p1_won, p2_won) = solvep2(&mut mem, &p1, &p2);

	p1_won.max(p2_won).to_string()
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
        let input = util::read_file("inputs/2021/day21-sample.txt");
        assert_eq!("739785", part1(4, 8));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day21.txt");
        assert_eq!("864900", part1(4, 5));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day21-sample.txt");
        assert_eq!("444356092776315", part2(4, 8));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day21.txt");
        assert_eq!("575111835924670", part2(4, 5));
    }
}
