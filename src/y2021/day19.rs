use crate::util;

use std::cell::Cell;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

#[derive(Debug)]
struct Scanner {
	sid: u8,
	points: Vec<[i32; 3]>,
}

fn parse_numbers_comma(s: &str) -> [i32; 3] {
	let mut numbers = s.split(',').map(|n| n.parse::<i32>().unwrap());
	let x = numbers.next().unwrap();
	let y = numbers.next().unwrap();
	let z = numbers.next().unwrap();
	[x, y, z]
}

impl Scanner {
	fn from_str(s: &str) -> Self {
		let mut lines = s.lines();
		let mut header = lines.next().unwrap().split_ascii_whitespace();
		let sid: u8 = {
			header.next();
			header.next();
			header.next().unwrap().parse().unwrap()
		};
		let mut points = vec![];
		for line in lines {
			points.push(parse_numbers_comma(line));
		}

		Self {
			sid,
			points,
		}
	}
}

struct AxisInfo {
	axis: usize,
	sign: i32,
	diff: i32,
}

fn x_edges_from(src: &Scanner, scanners_by_id: &HashMap<u8, Scanner>) -> HashMap<u8, AxisInfo> {
	let mut x_edges = HashMap::new();
	for other in scanners_by_id.values() {
		for axis in 0..=2 {
			for sign in [-1, 1] {
				let mut dx: Counter<i32> = Counter::new();
				for [x, _, _] in &src.points {
					for other_pt in &other.points {
						dx.count(x - other_pt[axis] * sign);
					}
				}
				let (x_diff, n) = dx.most_common(1)[0];
				if n >= 12 {
					x_edges.insert(other.sid, AxisInfo {
						axis,
						sign,
						diff: *x_diff,
					});
				}
			}
		}
	}

	x_edges
}

fn yz_edges_from(src: &Scanner, x_edges: &HashMap<u8, AxisInfo>, scanners_by_id: &HashMap<u8, Scanner>) -> (HashMap<u8, AxisInfo>, HashMap<u8, AxisInfo>) {
	let mut y_edges = HashMap::<u8, AxisInfo>::new();
	let mut z_edges = HashMap::<u8, AxisInfo>::new();

	for (dst_id, value) in x_edges {
		let other = scanners_by_id.get(&dst_id).unwrap();
		for axis in 0..=2 {
			for sign in [-1, 1] {
				let mut dy: Counter<i32> = Counter::new();
				let mut dz: Counter<i32> = Counter::new();
				for [_, y, z] in &src.points {
					for other_pt in &other.points {
						dy.count(y - other_pt[axis] * sign);
						dz.count(z - other_pt[axis] * sign);
					}
				}
				let (y_diff, yn) = dy.most_common(1)[0];
				if yn >= 12 {
					y_edges.insert(*dst_id, AxisInfo {
						axis,
						sign,
						diff: *y_diff,
					});
				}

				let (z_diff, zn) = dz.most_common(1)[0];
				if zn >= 12 {
					z_edges.insert(*dst_id, AxisInfo {
						axis,
						sign,
						diff: *z_diff,
					});
				}
			}
		}
	}

	(y_edges, z_edges)
}

fn part1(input: String) -> String {
	let scanners: Vec<Scanner> = input
		.split("\n\n")
		.map(|s| Scanner::from_str(s))
		.collect();

	let mut scanners_by_id = HashMap::<u8, Scanner>::with_capacity(scanners.len());
	for s in scanners {
		scanners_by_id.insert(s.sid, s);
	}

	let mut all_points: HashSet<[i32; 3]> = HashSet::new();
	for point in &scanners_by_id.get(&0).unwrap().points {
		all_points.insert(point.clone());
	}

	let mut todo: Vec<Scanner> = vec![scanners_by_id.remove(&0).unwrap()];

	while let Some(src) = todo.pop() {
		let x_edges = x_edges_from(&src, &scanners_by_id);
		let (y_edges, z_edges) = yz_edges_from(&src, &x_edges, &scanners_by_id);

		for k in x_edges.keys() {
			let x_edge = x_edges.get(k).unwrap();
			let y_edge = y_edges.get(k).unwrap();
			let z_edge = z_edges.get(k).unwrap();
			let dst_x = x_edge.diff;
			let dst_y = y_edge.diff;
			let dst_z = z_edge.diff;

			let mut next_scanner = scanners_by_id.remove(k).unwrap();
			next_scanner.points = {
				let mut tmp = vec![];
				for pt in next_scanner.points {
					tmp.push([
						dst_x + x_edge.sign * pt[x_edge.axis],
						dst_y + y_edge.sign * pt[y_edge.axis],
						dst_z + z_edge.sign * pt[z_edge.axis],
					]);
				}
				tmp
			};

			for point in &next_scanner.points {
				all_points.insert(point.clone());
			}

			todo.push(next_scanner);
		}
	}

	all_points.len().to_string()
}

fn part2(input: String) -> String {
	let scanners: Vec<Scanner> = input
		.split("\n\n")
		.map(|s| Scanner::from_str(s))
		.collect();

	let mut scanners_by_id = HashMap::<u8, Scanner>::with_capacity(scanners.len());
	for s in scanners {
		scanners_by_id.insert(s.sid, s);
	}

	let mut all_points: HashSet<[i32; 3]> = HashSet::new();
	for point in &scanners_by_id.get(&0).unwrap().points {
		all_points.insert(point.clone());
	}

	let mut scanner_positions = HashMap::<u8, (i32, i32, i32)>::new();
	scanner_positions.insert(0, (0, 0, 0));
	let mut todo: Vec<Scanner> = vec![scanners_by_id.remove(&0).unwrap()];

	while let Some(src) = todo.pop() {
		let x_edges = x_edges_from(&src, &scanners_by_id);
		let (y_edges, z_edges) = yz_edges_from(&src, &x_edges, &scanners_by_id);

		for k in x_edges.keys() {
			let x_edge = x_edges.get(k).unwrap();
			let y_edge = y_edges.get(k).unwrap();
			let z_edge = z_edges.get(k).unwrap();
			let dst_x = x_edge.diff;
			let dst_y = y_edge.diff;
			let dst_z = z_edge.diff;

			scanner_positions.insert(*k, (dst_x, dst_y, dst_z));

			let mut next_scanner = scanners_by_id.remove(k).unwrap();
			next_scanner.points = {
				let mut tmp = vec![];
				for pt in next_scanner.points {
					tmp.push([
						dst_x + x_edge.sign * pt[x_edge.axis],
						dst_y + y_edge.sign * pt[y_edge.axis],
						dst_z + z_edge.sign * pt[z_edge.axis],
					]);
				}
				tmp
			};

			for point in &next_scanner.points {
				all_points.insert(point.clone());
			}

			todo.push(next_scanner);
		}
	}

	let mut max_dist = 0;
	for (i, (x1, y1, z1)) in scanner_positions.values().enumerate() {
		for (x2, y2, z2) in scanner_positions.values().skip(i + 1) {
			max_dist = max_dist.max((x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs());
		}
	}

	max_dist.to_string()
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

struct Counter<T> {
    counts: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: std::hash::Hash + Eq + Clone + std::cmp::Ord,
{
    fn new() -> Self {
        Counter {
            counts: HashMap::new(),
        }
    }

    fn count(&mut self, item: T) {
        let entry = self.counts.entry(item).or_insert(0);
        *entry += 1;
    }

    fn get_count(&self, item: &T) -> usize {
        self.counts.get(item).copied().unwrap_or(0)
    }

    fn most_common(&self, n: usize) -> Vec<(&T, usize)> {
        let mut heap = BinaryHeap::new();

        for (item, &count) in self.counts.iter() {
            heap.push(Reverse((count, item)));
            if heap.len() > n {
                heap.pop();
            }
        }

        heap.into_sorted_vec().into_iter().map(|Reverse((count, item))| (item, count)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = util::read_file("inputs/2021/day19-sample.txt");
        assert_eq!("79", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day19.txt");
        assert_eq!("483", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day19-sample.txt");
        assert_eq!("3621", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day19.txt");
        assert_eq!("14804", part2(input));
    }
}
