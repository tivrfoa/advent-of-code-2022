use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn hash(s: &str) -> usize {
    let mut curr = 0;
    for c in s.chars() {
        curr += c as usize;
        curr *= 17;
        curr %= 256;
    }
    curr
}

pub fn part1(input: &str) -> String {
    let mut sum = 0;
    let line = input.lines().next().unwrap();

    for s in line.split(',') {
        sum += hash(s);
    }

    sum.to_string()
}

fn order_boxes(boxes: Vec<(HashMap<&str, (&str, usize)>, usize)>) -> Vec<Vec<usize>> {
    let mut ordered_boxes = vec![vec![]; 256];

    for (i, b) in boxes.into_iter().enumerate() {
        let mut v: Vec<(&str, usize)> = b.0.into_values().collect();
        v.sort_by(|a, b| a.1.cmp(&b.1));
        ordered_boxes[i] = v.into_iter().map(|v| v.0.parse::<usize>().unwrap()).collect();
    }

    ordered_boxes
}

pub fn part2(input: &str) -> String {
    let line = input.lines().next().unwrap();
    let mut boxes: Vec<(HashMap<&str, (&str, usize)>, usize)> = vec![(HashMap::new(), 0); 256];

    for s in line.split(',') {
        if s.contains('=') {
            let (l, r) = s.split_once('=').unwrap();
            let b = hash(l);
            if let Some(e) = boxes[b].0.get_mut(&l) {
                e.0 = r;
            } else {
                let pos = boxes[b].1;
                boxes[b].1 += 1;
                boxes[b].0.insert(l, (r, pos));
            }
        } else {
            let (l, _) = s.split_once('-').unwrap();
            let b = hash(l);
            boxes[b].0.remove(l);
        }
    }

    let boxes: Vec<Vec<usize>> = order_boxes(boxes);
    let mut sum = 0;
    for (i, b) in boxes.into_iter().enumerate() {
        for (j, l) in b.into_iter().enumerate() {
            sum += (i + 1) * (j + 1) * l;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2023/day15-sample.txt");
        assert_eq!("1320", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day15.txt");
        assert_eq!("509784", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day15-sample.txt");
        assert_eq!("145", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day15.txt");
        assert_eq!("230197", part2(input));
    }
}
