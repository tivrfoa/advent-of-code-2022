use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

pub fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let mut dir_idx = 0;
    let dirs: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut curr = "AAA";

    for line in lines {
        let (k, v) = line.split_once(" = ").unwrap();
        let (l, r) = v[1..v.len() - 1].split_once(", ").unwrap();
        map.insert(k, (l, r));
    }

    let mut steps = 0;
    while curr != "ZZZ" {
        steps += 1;

        if dirs[dir_idx] == 'L' {
            curr = map[curr].0;
        } else {
            curr = map[curr].1;
        }

        dir_idx += 1;
        if dir_idx == dirs.len() {
            dir_idx = 0;
        }
    }

    steps.to_string()
}

pub fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let dirs: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut curr: Vec<&str> = vec![];

    for line in lines {
        let (k, v) = line.split_once(" = ").unwrap();
        let (l, r) = v[1..v.len() - 1].split_once(", ").unwrap();
        map.insert(k, (l, r));
        if &k[2..] == "A" {
            curr.push(k);
        }
    }

    dbg!(&curr);

    let mut zs_times: Vec<Vec<u64>> = vec![vec![]; curr.len()];
    // solve all possible Zs for each start until a loop
    for (p, start) in curr.iter().enumerate() {
        let mut steps = 0;
        let mut pos = *start;
        let mut dir_idx = 0;
        let mut visited: HashSet<(usize, &str)> = HashSet::new();
        loop {
            steps += 1;
            if visited.contains(&(dir_idx, pos)) {
                println!("Loop detected for {pos} at {dir_idx} with {steps} steps.");
                break;
            }
            visited.insert((dir_idx, pos));

            if dirs[dir_idx] == 'L' {
                pos = map[pos].0;
            } else {
                pos = map[pos].1;
            }
            if &pos[2..] == "Z" {
                println!("{start} arrived at {pos}");
                zs_times[p].push(steps);
            }

            dir_idx += 1;
            if dir_idx == dirs.len() {
                dir_idx = 0;
            }
        }
    }

    dbg!(&zs_times);
    let mut lcm = zs_times[0][0];
    for i in 1..zs_times.len() {
        lcm = util::lcm(lcm, zs_times[i][0]);
    }

    lcm.to_string()
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
        let input = include_str!("../../inputs/2023/day8-sample.txt");
        assert_eq!("2", part1(input));
    }

    #[test]
    fn sample2() {
        let input = include_str!("../../inputs/2023/day8-sample2.txt");
        assert_eq!("6", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day8.txt");
        assert_eq!("24253", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day8-p2-sample.txt");
        assert_eq!("6", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day8.txt");
        assert_eq!("12357789728873", part2(input));
    }
}
