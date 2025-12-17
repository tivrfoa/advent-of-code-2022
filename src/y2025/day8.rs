use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use new_derive::New;

use util::*;

#[derive(Debug, New)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn distance_to(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }
}

fn parse(input: &str) -> Vec<Point3D> {
	let mut ret = vec![];

	for l in input.lines() {
		let mut vv = l.split(',');
		let x = vv.next().unwrap().parse::<f64>().unwrap();
		let y = vv.next().unwrap().parse::<f64>().unwrap();
		let z = vv.next().unwrap().parse::<f64>().unwrap();
		ret.push(Point3D::new(x, y, z));
	}

	ret
}

struct Circuit {
	boxes: Vec<usize>,
}

fn find_distances(input: &[Point3D]) -> Vec<(f64, usize, usize)> {
	let mut dd = vec![];

	for (i, a) in input.iter().enumerate() {
		for (j, b) in input.iter().enumerate().skip(i + 1) {
			dd.push((a.distance_to(b), i, j));
		}
	}

	dd
}

fn solve(input: &[Point3D], max_conn: usize) -> usize {
	let mut circuits: Vec<Circuit> = vec![];
	let mut dd = find_distances(input);
	//dbg!(dd);
	// dd.sort_unstable(); // the trait `Ord` is not implemented for `f64`
	dd.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
	// for i in 0..5 {
	// 	dbg!(dd[i]);
	// }

	0
}

pub fn part1(input: &str, max_conn: usize) -> String {
	let boxes = parse(input);
	//dbg!(boxes);
	solve(&boxes, max_conn).to_string()
}

pub fn part2(input: &str) -> String {
    "".into()
}

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
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
        let input = include_str!("../../inputs/2025/day8-sample.txt");
        assert_eq!("", part1(input, 10));
    }

    //#[test]
    //fn p1() {
    //    let input = include_str!("../../inputs/2025/day8.txt");
    //    assert_eq!("", part1(input, 1000));
    //}

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2025/day8-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2025/day8.txt");
        assert_eq!("", part2(input));
    }
}
