use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use rayon::prelude::*;

use util::*;

/*

Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82
Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43
Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86
Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35

ss -> source start
ds -> destination start
len
dv -> destination value

1) find which range

ss <= v < ss + len

50 <= 79 < 98

2) find destination value

let d = 79 - ss
d = 79 - 50
d = 29

dv = ds + d
dv = 52 + 29
dv = 81

3) Go to the next map ...

*/

struct Map {
	ds: Vec<u64>,
	ss: Vec<u64>,
	len: Vec<u64>,
}

impl Map {
	fn new() -> Self {
		Self {
			ds: vec![],
			ss: vec![],
			len: vec![],
		}
	}

	fn add_line(&mut self, s: &str) {
		let nums: Vec<u64> = s.split_to_nums(' ');
		self.ds.push(nums[0]);
		self.ss.push(nums[1]);
		self.len.push(nums[2]);
	}

	fn find_destination_value(&self, sv: u64) -> u64 {
		for i in 0..self.ds.len() {
			let ss = self.ss[i];
			if ss <= sv && sv < ss + self.len[i] {
				let d = sv - ss;
				return self.ds[i] + d
			}
		}
		sv
	}

	fn find_destination_value_reverse(&self, sv: u64) -> u64 {
		for i in 0..self.ds.len() {
			let ds = self.ds[i];
			if ds <= sv && sv < ds + self.len[i] {
				let d = sv - ds;
				return self.ss[i] + d
			}
		}
		sv
	}
}

pub fn part1(input: &str) -> String {
	let mut lowest = u64::MAX;

	let (seeds, maps_in) = input.split_once("\n\n").unwrap();
	let seeds = seeds.split_once(": ").unwrap().1;
	let seeds: Vec<u64> = seeds.split_to_nums(' ');
	let mut maps: Vec<Map> = vec![];

	for map in maps_in.split("\n\n") {
		let mut new_map = Map::new();
		for line in map.lines().skip(1) {
			new_map.add_line(line);
		}
		maps.push(new_map);
	}

	for seed in seeds {
		let mut sv = seed;
		for map in &maps {
			sv = map.find_destination_value(sv);
		}
		if sv < lowest {
			lowest = sv;
		}
	}

	lowest.to_string()
}

fn merge(seeds_in: Vec<u64>) -> Vec<(u64, u64)> {
	let mut seeds: Vec<(u64, u64)> = Vec::with_capacity(seeds_in.len() / 2);
	for i in (0..seeds_in.len()).step_by(2) {
		seeds.push((seeds_in[i], seeds_in[i] + seeds_in[i + 1] - 1));
	}
	seeds.sort();
	let mut idx = 0;
	while idx + 1 < seeds.len() {
		if seeds[idx].1 < seeds[idx + 1].0 {
			idx += 1;
			continue;
		}
		println!("Merging!");
		seeds[idx].1 = seeds[idx + 1].1;
		seeds.remove(idx + 1);
	}
	seeds
}

pub fn part2(input: &str) -> String {
	let (seeds, maps_in) = input.split_once("\n\n").unwrap();
	let seeds = seeds.split_once(": ").unwrap().1;
	let seeds: Vec<u64> = seeds.split_to_nums(' ');
	let seeds: Vec<(u64, u64)> = merge(seeds);
	let maps: Vec<Map> = {
		let mut maps: Vec<Map> = vec![];
		for map in maps_in.split("\n\n") {
			let mut new_map = Map::new();
			for line in map.lines().skip(1) {
				new_map.add_line(line);
			}
			maps.push(new_map);
		}
		maps
	};

	for lowest in 0.. {
		let mut sv = lowest;
		for map in maps.iter().rev() {
			sv = map.find_destination_value_reverse(sv);
		}
		for (l, r) in seeds.iter() {
			if *l <= sv && sv <= *r {
				return lowest.to_string();
			}
		}
	}

	panic!("Mission failed");
}

pub fn part2_rayon(input: &str) -> String {
	let (seeds, maps_in) = input.split_once("\n\n").unwrap();
	let seeds = seeds.split_once(": ").unwrap().1;
	let seeds: Vec<u64> = seeds.split_to_nums(' ');
	let seeds: Vec<(u64, u64)> = merge(seeds);
	let maps: Vec<Map> = {
		let mut maps: Vec<Map> = vec![];
		for map in maps_in.split("\n\n") {
			let mut new_map = Map::new();
			for line in map.lines().skip(1) {
				new_map.add_line(line);
			}
			maps.push(new_map);
		}
		maps
	};

	seeds.par_iter()
		.map(|(l, r)| {
			let mut lowest = u64::MAX;
			for seed in *l..=*r {
				let mut sv = seed;
				for map in &maps {
					sv = map.find_destination_value(sv);
				}
				if sv < lowest {
					lowest = sv;
				}
			}
			lowest
		}).min().unwrap().to_string()
}

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u64,
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
        let input = include_str!("../../inputs/2023/day5-sample.txt");
        assert_eq!("35", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day5.txt");
        assert_eq!("331445006", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day5-sample.txt");
        assert_eq!("46", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day5.txt");
        assert_eq!("6472060", part2(input));
    }
}
