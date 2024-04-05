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

fn perfect_minimal_hash(lookup: &mut [usize], nodes: &mut Vec<Vec<usize>>, slice: &[u8]) -> usize {
    // Base 26 index.
    let hash = slice[..3].iter().fold(0, |acc, b| 26 * acc + ((b - b'a') as usize));
    let mut index = lookup[hash];

    // First time seeing this key so push a new node and return its index.
    if index == usize::MAX {
        index = nodes.len();
        lookup[hash] = index;
        nodes.push(Vec::with_capacity(10));
    }

    index
}

struct Input {
    edges: Vec<usize>,
    nodes: Vec<(usize, usize)>,
}

impl Input {
    #[inline]
    fn neighbours(&self, node: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (start, end) = self.nodes[node];
        (start..end).map(|edge| (edge, self.edges[edge]))
    }
}

fn parse(input: &str) -> Input {
    let mut lookup = vec![usize::MAX; 26 * 26 * 26];
    let mut neighbours = Vec::with_capacity(2_000);

    for line in input.lines().map(str::as_bytes) {
        let first = perfect_minimal_hash(&mut lookup, &mut neighbours, line);

        for chunk in line[5..].chunks(4) {
            let second = perfect_minimal_hash(&mut lookup, &mut neighbours, chunk);
            neighbours[first].push(second);
            neighbours[second].push(first);
        }
    }

    let mut edges = Vec::with_capacity(5_000);
    let mut nodes = Vec::with_capacity(neighbours.len());

    for list in neighbours {
        let start = edges.len();
        let end = edges.len() + list.len();
        edges.extend(list);
        nodes.push((start, end));
    }

    Input { edges, nodes }
}

fn furthest(input: &Input, start: usize) -> usize {
    let mut todo = VecDeque::new();
    todo.push_back(start);
    let mut seen = vec![false; input.nodes.len()];
    seen[start] = true;
    let mut result = start;
    while let Some(current) = todo.pop_front() {
        result = current;
        for (_, next) in input.neighbours(current) {
            if !seen[next] {
                todo.push_back(next);
                seen[next] = true;
            }
        }
    }

    result
}

fn flow(input: &Input, start: usize, end: usize) -> usize {
    let mut todo = VecDeque::new();
    let mut path = vec![];
    let mut used = vec![false; input.edges.len()];
    let mut result = 0;

    for _ in 0..4 {
        todo.push_back((start, usize::MAX));
        result = 0;
        let mut seen = vec![false; input.nodes.len()];
        seen[start] = true;
        while let Some((current, head)) = todo.pop_front() {
            result += 1;
            if current == end {
                let mut index = head;
                while index != usize::MAX {
                    let (edge, next) = path[index];
                    used[edge] = true;
                    index = next;
                }
                break;
            }

            for (edge, next) in input.neighbours(current) {
                if !used[edge] && !seen[next] {
                    seen[next] = true;
                    todo.push_back((next, path.len()));
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
    let input = parse(input);
    let start = furthest(&input, 0);
    let end = furthest(&input, start);
    let size = flow(&input, start, end);
    (size * (input.nodes.len() - size)).to_string()
}

pub fn part2(input: &str) -> String {
    "".into()
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

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day25-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day25.txt");
        assert_eq!("", part2(input));
    }
}
