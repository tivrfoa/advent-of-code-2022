use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

// Solution copied from:
// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day25.rs

fn perfect_minimal_hash(lookup: &mut [usize], next_idx: usize, slice: &[u8]) -> usize {
    // Base 26 index.
    let hash = slice[..3].iter().fold(0, |acc, b| 26 * acc + ((b - b'a') as usize));
    if lookup[hash] == usize::MAX {
        lookup[hash] = next_idx;
    }
    lookup[hash]
}

struct Node {
    connections: Vec<usize>,
}

fn parse(input: &str) -> Vec<Node> {
    let mut lookup = vec![usize::MAX; 26 * 26 * 26];
    let mut nodes: Vec<Node> = Vec::with_capacity(2_000);

    for line in input.lines().map(str::as_bytes) {
        let first = perfect_minimal_hash(&mut lookup, nodes.len(), line);
        if first == nodes.len() {
            nodes.push(Node {
                connections: vec![],
            });
        }

        for chunk in line[5..].chunks(4) {
            let second = perfect_minimal_hash(&mut lookup, nodes.len(), chunk);
            if second == nodes.len() {
                nodes.push(Node {
                    connections: vec![first],
                });
            } else {
                nodes[second].connections.push(first);
            }
            nodes[first].connections.push(second);
        }
    }

    nodes
}

fn furthest(nodes: &[Node], start: usize) -> usize {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);
    let mut seen = vec![false; nodes.len()];
    seen[start] = true;
    let mut result = start;
    while let Some(current) = to_visit.pop_front() {
        result = current;
        for next in &nodes[current].connections {
            if !seen[*next] {
                to_visit.push_back(*next);
                seen[*next] = true;
            }
        }
    }

    result
}

fn flow(nodes: &[Node], start: usize, end: usize) -> usize {
    let mut todo = VecDeque::new();
    let mut path = vec![];
    let mut used: HashSet<(usize, usize)> = HashSet::new();
    let mut result = 0;

    for _ in 0..4 {
        todo.push_back((start, usize::MAX));
        result = 0;
        let mut seen = vec![false; nodes.len()];
        seen[start] = true;
        while let Some((current, head)) = todo.pop_front() {
            result += 1;
            if current == end {
                let mut index = head;
                while index != usize::MAX {
                    let (edge, next) = path[index];
                    used.insert(edge);
                    index = next;
                }
                break;
            }

            for next in &nodes[current].connections {
                let edge = (current, *next);
                if !used.contains(&edge) && !seen[*next] {
                    seen[*next] = true;
                    todo.push_back((*next, path.len()));
                    path.push((edge, head));
                }
            }
        }

        todo.clear();
        path.clear();
    }

    result
}

pub fn part1(input: &str) -> String {
    let nodes = parse(input);
    let start = furthest(&nodes, 0);
    // let end = furthest(&input, start);
    // let size = flow(&input, start, end);
    let size = flow(&nodes, 0, start);
    (size * (nodes.len() - size)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2023/day25-sample.txt");
        assert_eq!("54", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day25.txt");
        assert_eq!("592171", part1(input));
    }
}
