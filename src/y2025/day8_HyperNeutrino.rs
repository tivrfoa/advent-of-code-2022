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
    x: usize,
    y: usize,
    z: usize,
}

impl Point3D {
	#[inline(always)]
    fn distance(&self, other: &Point3D) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug)]
struct Edge {
	a: usize,
	b: usize,
	distance: usize,
}

impl Edge {
	fn print(&self) {
		println!("{a} - {b} -> {d}", a=self.a, b=self.b, d=self.distance);
	}
}

#[derive(Debug)]
struct Tree {
	parent: Vec<usize>,
}

impl Tree {
	fn print(&self) {
		print!("[");
		for i in &self.parent { print!("{i}, "); }
		println!("]");
	}
	fn root(&mut self, idx: usize) -> usize {
		if self.parent[idx] == idx { return idx; }
		self.parent[idx] = self.root(self.parent[idx]);
		self.parent[idx]
	}
	fn merge(&mut self, a: usize, b: usize) {
		let b_root = self.root(b);
		let a_root = self.root(a);
		self.parent[a_root] = b_root;
	}
	fn merge_n(&mut self, edges: &[Edge], n: usize) {
		for edge in edges.iter().take(n) {
			self.merge(edge.a, edge.b);
		}
	}
}

fn edges_from_boxes(boxes: &[Point3D]) -> Vec<Edge> {
	let mut edges = vec![];
	for (a, box1) in boxes.iter().enumerate() {
		for (b, box2) in boxes.iter().enumerate().skip(a + 1) {
			edges.push(Edge {
				a,
				b,
				distance: box1.distance(box2),
			});
		}
	}
	edges
}

fn solve1(boxes: &[Point3D], max_conn: usize) -> usize {
	let num_boxes = boxes.len();
	let mut tree = Tree { parent: (0..num_boxes).collect() };
	let mut edges = edges_from_boxes(boxes);
	edges.sort_unstable_by(|a, b| a.distance.cmp(&b.distance));
	tree.merge_n(&edges, max_conn);
	let mut sizes = vec![0; num_boxes];
	for (i, _) in boxes.iter().enumerate() {
		sizes[tree.root(i)] += 1;
	}
	sizes.sort_unstable_by_key(|&n| std::cmp::Reverse(n));

	sizes[0] * sizes[1] * sizes[2]
}

pub fn part1(input: &str, max_conn: usize) -> String {
	let boxes = parse(input);
	solve1(&boxes, max_conn).to_string()
}

fn solve2(boxes: &[Point3D]) -> usize {
	let mut num_boxes = boxes.len();
	let mut tree = Tree { parent: (0..num_boxes).collect() };
	let mut edges = edges_from_boxes(boxes);
	edges.sort_unstable_by(|a, b| a.distance.cmp(&b.distance));
	for edge in edges.iter() {
		if tree.root(edge.a) == tree.root(edge.b) { continue; }
		tree.merge(edge.a, edge.b);
		num_boxes -= 1;
		if num_boxes == 1 {
			return boxes[edge.a].x * boxes[edge.b].x;
		}
	}
	unreachable!()
}

pub fn part2(input: &str) -> String {
	let boxes = parse(input);
	solve2(&boxes).to_string()
}

fn parse(input: &str) -> Vec<Point3D> {
	let mut ret = vec![];

	for l in input.lines() {
		let mut vv = l.split(',');
		let x = vv.next().unwrap().parse::<usize>().unwrap();
		let y = vv.next().unwrap().parse::<usize>().unwrap();
		let z = vv.next().unwrap().parse::<usize>().unwrap();
		ret.push(Point3D::new(x, y, z));
	}

	ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2025/day8-sample.txt");
        assert_eq!("40", part1(input, 10));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2025/day8.txt");
        assert_eq!("46398", part1(input, 1000));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2025/day8-sample.txt");
        assert_eq!("25272", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2025/day8.txt");
        assert_eq!("8141888143", part2(input));
    }
}
