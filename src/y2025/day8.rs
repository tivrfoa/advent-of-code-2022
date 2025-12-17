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

	#[inline]
    fn dist2(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

type CircuitId = usize;
const NO_CIRCUIT: usize = usize::MAX;

fn parse(input: &str) -> Vec<(Point3D, CircuitId)> {
	let mut ret = vec![];

	for l in input.lines() {
		let mut vv = l.split(',');
		let x = vv.next().unwrap().parse::<f64>().unwrap();
		let y = vv.next().unwrap().parse::<f64>().unwrap();
		let z = vv.next().unwrap().parse::<f64>().unwrap();
		ret.push((Point3D::new(x, y, z), NO_CIRCUIT));
	}

	ret
}

fn find_distances(input: &[(Point3D, CircuitId)]) -> Vec<(f64, usize, usize)> {
	let mut dd = vec![];

	for (i, a) in input.iter().enumerate() {
		for (j, b) in input.iter().enumerate().skip(i + 1) {
			// dd.push((a.0.distance_to(&b.0), i, j));
			dd.push((a.0.dist2(&b.0), i, j));
		}
	}

	dd
}

struct Circuit {
	qt: usize,
	boxes: Vec<usize>,
}

fn solve(input: &mut Vec<(Point3D, CircuitId)>, max_conn: usize) -> usize {
	// index is its id, and the value is the number of boxes in it
	let num_boxes = input.len();
	let mut circuits: Vec<Circuit> = vec![];
	let mut dd = find_distances(input);
	dd.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

	for (loop_id, d) in (0..max_conn).zip(dd.iter()) {
		let i = d.1;
		let j = d.2;
		let cid_a = input[i].1;
		let cid_b = input[j].1;
		// println!("{loop_id}: {i} circuit: {cid_a}, {j} circuit: {cid_b}");

		if cid_a == NO_CIRCUIT && cid_b == NO_CIRCUIT {
			// both aren't in any circuit yet
			circuits.push(Circuit {
				qt: 2,
				boxes: vec![i, j],
			});
			input[i].1 = circuits.len() - 1;
			input[j].1 = circuits.len() - 1;
		} else if cid_a == NO_CIRCUIT {
			input[i].1 = cid_b;
			circuits[cid_b].qt += 1;
			circuits[cid_b].boxes.push(i);
		} else if cid_b == NO_CIRCUIT {
			input[j].1 = cid_a;
			circuits[cid_a].qt += 1;
			circuits[cid_a].boxes.push(j);
		} else {
			// Both have circuits. Merge B into A.

            // Check if they are already the same (cycle)
			if cid_a == cid_b {
                continue;
            }

			// Transfer count from B to A
            circuits[cid_a].qt += circuits[cid_b].qt;
            circuits[cid_b].qt = 0;

			// Relabel all nodes in B to A
			let old_v = std::mem::replace(&mut circuits[cid_b].boxes, Vec::new());
			for box_id in old_v {
				circuits[cid_a].boxes.push(box_id);
				input[box_id].1 = cid_a;
			}
		}
	}
	circuits.sort_unstable_by(|a, b| b.qt.cmp(&a.qt));

	circuits[0].qt * circuits[1].qt * circuits[2].qt
}

fn solve2(input: &mut Vec<(Point3D, CircuitId)>) -> usize {
	// index is its id, and the value is the number of boxes in it
	let num_boxes = input.len();
	let mut circuits: Vec<Circuit> = vec![];
	let mut dd = find_distances(input);
	dd.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

	for d in dd.iter() {
		let i = d.1;
		let j = d.2;
		let cid_a = input[i].1;
		let cid_b = input[j].1;
		// println!("{loop_id}: {i} circuit: {cid_a}, {j} circuit: {cid_b}");

		if cid_a == NO_CIRCUIT && cid_b == NO_CIRCUIT {
			// both aren't in any circuit yet
			circuits.push(Circuit {
				qt: 2,
				boxes: vec![i, j],
			});
			input[i].1 = circuits.len() - 1;
			input[j].1 = circuits.len() - 1;
		} else if cid_a == NO_CIRCUIT {
			input[i].1 = cid_b;
			circuits[cid_b].qt += 1;
			circuits[cid_b].boxes.push(i);
			if circuits[cid_b].boxes.len() == num_boxes {
				return input[i].0.x as usize * input[j].0.x as usize;
			}
		} else if cid_b == NO_CIRCUIT {
			input[j].1 = cid_a;
			circuits[cid_a].qt += 1;
			circuits[cid_a].boxes.push(j);
			if circuits[cid_a].boxes.len() == num_boxes {
				return input[i].0.x as usize * input[j].0.x as usize;
			}
		} else {
			// Both have circuits. Merge B into A.

            // Check if they are already the same (cycle)
			if cid_a == cid_b {
                continue;
            }

			// Transfer count from B to A
            circuits[cid_a].qt += circuits[cid_b].qt;
            circuits[cid_b].qt = 0;
			if circuits[cid_a].qt == num_boxes {
				return input[i].0.x as usize * input[j].0.x as usize;
			}

			// Relabel all nodes in B to A
			let old_v = std::mem::replace(&mut circuits[cid_b].boxes, Vec::new());
			for box_id in old_v {
				circuits[cid_a].boxes.push(box_id);
				input[box_id].1 = cid_a;
			}
		}
	}
	circuits.sort_unstable_by(|a, b| b.qt.cmp(&a.qt));

	circuits[0].qt * circuits[1].qt * circuits[2].qt
}

pub fn part1(input: &str, max_conn: usize) -> String {
	let mut boxes = parse(input);
	//dbg!(boxes);
	solve(&mut boxes, max_conn).to_string()
}

pub fn part2(input: &str) -> String {
	let mut boxes = parse(input);
	solve2(&mut boxes).to_string()
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

    //#[test]
    //fn p2() {
    //    let input = include_str!("../../inputs/2025/day8.txt");
    //    assert_eq!("", part2(input));
    //}
}
