use crate::util;

use std::cell::Cell;
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

/*


A cube can be desintegrated if it has no cubes above it, or
if the cube above it is also supported by other cubes.

1) Merge each line into one cube
2) Sort by y
3) Track for each cube the cubes that support it;
4) Track for each cube the cubes that it supports;

*/

#[derive(Debug)]
struct Cube {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
    supported_by: Vec<usize>,
    supports: Vec<usize>,
}

impl Cube {
    fn overlaps(&self, other: &Cube) -> bool {
        (is_between(self.x1, other.x1, other.x2) ||
            is_between(self.x2, other.x1, other.x2) ||
            self.x1 < other.x1 && self.x2 > other.x2) &&
            (is_between(self.y1, other.y1, other.y2) ||
            is_between(self.y2, other.y1, other.y2) ||
            self.y1 < other.y1 && self.y2 > other.y2)

    }
}

pub fn part1(input: &str) -> String {
    let mut cubes: Vec<Cube> = parse(input);
    cubes.sort_unstable_by(|a, b| a.z1.cmp(&b.z1).then(a.z2.cmp(&b.z2)));
    let mut heights: BTreeMap<Reverse<i32>, Vec<usize>> = BTreeMap::new();
    // make them fall
    for i in 0..cubes.len() {
        let mut z1 = cubes[i].z1;
        let mut z2 = cubes[i].z2;
        let diff = z2 - z1;
        for (h, cubes_indexes) in heights.iter() {
            for j in cubes_indexes {
                let j = *j;
                if cubes[i].overlaps(&cubes[j]) {
                    cubes[j].supports.push(i);
                    cubes[i].supported_by.push(j);
                    z1 = h.0 + 1;
                    z2 = z1 + diff;
                }
            }
            if !cubes[i].supported_by.is_empty() {
                break;
            }
        }

        if cubes[i].supported_by.is_empty() {
            let diff = cubes[i].z2 - cubes[i].z1;
            cubes[i].z1 = 1;
            cubes[i].z2 = 1 + diff;
        } else {
            cubes[i].z1 = z1;
            cubes[i].z2 = z2;
        }
        heights.entry(Reverse(cubes[i].z2)).or_insert(vec![]).push(i);
    }

    let mut qt = 0;
    'c: for i in 0..cubes.len() {
        for s in &cubes[i].supports {
            if cubes[*s].supported_by.len() == 1 {
                continue 'c;
            }
        }
        qt += 1;
    }

    qt.to_string()
}

fn parse(input: &str) -> Vec<Cube> {
    let mut ret = vec![];
    for (_id, line) in input.lines().enumerate() {
        ret.push(parse_cube(line));
    }
    ret
}

fn parse_cube(line: &str) -> Cube {
    let (l, r) = line.split_once('~').unwrap();
    let l: Vec<i32> = l.split(',').map(|n| n.parse().unwrap()).collect();
    let r: Vec<i32> = r.split(',').map(|n| n.parse().unwrap()).collect();
    let (x1, x2) = (l[0], r[0]);
    let (y1, y2) = (l[1], r[1]);
    let (z1, z2) = (l[2], r[2]);

    Cube {
        x1,
        x2,
        y1,
        y2,
        z1,
        z2,
        supported_by: vec![],
        supports: vec![],
    }
}

pub fn part2(input: &str) -> String {
    let mut cubes: Vec<Cube> = parse(input);
    cubes.sort_unstable_by(|a, b| a.z1.cmp(&b.z1).then(a.z2.cmp(&b.z2)));
    let mut heights: BTreeMap<Reverse<i32>, Vec<usize>> = BTreeMap::new();
    let mut supported_by: Vec<i32> = vec![0; cubes.len()];
    // make them fall
    for i in 0..cubes.len() {
        let mut z1 = cubes[i].z1;
        let mut z2 = cubes[i].z2;
        let diff = z2 - z1;
        for (h, cubes_indexes) in heights.iter() {
            for j in cubes_indexes {
                let j = *j;
                if cubes[i].overlaps(&cubes[j]) {
                    cubes[j].supports.push(i);
                    cubes[i].supported_by.push(j);
                    supported_by[i] += 1;
                    z1 = h.0 + 1;
                    z2 = z1 + diff;
                }
            }
            if !cubes[i].supported_by.is_empty() {
                break;
            }
        }

        if cubes[i].supported_by.is_empty() {
            let diff = cubes[i].z2 - cubes[i].z1;
            cubes[i].z1 = 1;
            cubes[i].z2 = 1 + diff;
        } else {
            cubes[i].z1 = z1;
            cubes[i].z2 = z2;
        }
        heights.entry(Reverse(cubes[i].z2)).or_insert(vec![]).push(i);
    }

    let mut qt = 0;

    let mut to_visit = vec![];
    'c: for i in 0..cubes.len() {
        for s in &cubes[i].supports {
            if cubes[*s].supported_by.len() == 1 {
                to_visit.push(i);
                continue 'c;
            }
        }
    }

    for idx in to_visit {
        let mut is_desintegrated = vec![false; cubes.len()];
        let mut supported_by = supported_by.clone();
        let mut next: VecDeque<usize> = VecDeque::new();
        next.push_back(idx);
        
        while let Some(i) = next.pop_front() {
            for s in &cubes[i].supports {
                if supported_by[*s] <= 1 {
                    for z in &cubes[*s].supports {
                        supported_by[*z] -= 1;
                    }
                    if !is_desintegrated[*s] {
                        eprintln!("{i} destroying {}", *s);
                        is_desintegrated[*s] = true;
                        next.push_back(*s);
                        qt += 1;
                    }
                }
            }
        }
    }

    qt.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2023/day22-sample.txt");
        assert_eq!("5", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day22.txt");
        assert_eq!("505", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day22-sample.txt");
        assert_eq!("7", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day22.txt");
        assert_eq!("", part2(input));
    }
}
