use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

/*

x, m, a, s

accept (A) or reject (R)

workflows = map<String, Vec<Rule>>
start -> in

*/

fn r_to_usize(r: &str) -> usize {
    match r {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("{r}"),
    }
}

#[derive(Debug)]
struct Rule<'a> {
    r: usize,
    v: u32,
    comp: char,
    dest: &'a str,
}

type Ratings = [u32; 4];

impl<'a> Rule<'a> {
    fn is_valid_condition(&self, ratings: &Ratings) -> bool {
        if self.comp == '=' {
            true
        } else if self.comp == '>' {
            ratings[self.r] > self.v
        } else {
            ratings[self.r] < self.v
        }
    }
}

fn parse(input: &str) -> (HashMap<&str, Vec<Rule>>, Vec<Ratings>) {
    let mut lines = input.lines();

    // workflows
    let mut workflows: HashMap<&str, Vec<Rule>> = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (key, rules) = line.split_once('{').unwrap();
        for rule in rules[..rules.len() - 1].split(',') {
            let (r, v, comp, dest) = {
                if let Some((l, dest)) = rule.split_once(':') {
                    if rule.contains('>') {
                        let (r, v) = l.split_once('>').unwrap();
                        (r_to_usize(r), v.parse::<u32>().unwrap(), '>', dest)
                    } else {
                        let (r, v) = l.split_once('<').unwrap();
                        (r_to_usize(r), v.parse::<u32>().unwrap(), '<', dest)
                    }
                } else {
                    (0, 0, '=', rule)
                }
            };
            let rules = workflows.entry(key).or_insert(vec![]);
            rules.push(Rule { r, v, comp, dest });
        }
    }

    // ratings
    let mut ratings = vec![];
    for line in lines {
        let mut rating = [0; 4];
        for (i, values) in (&line[1..line.len() - 1]).split(',').enumerate() {
            let (_, v) = values.split_once('=').unwrap();
            rating[i] = v.parse().unwrap();
        }
        ratings.push(rating);
    }

    (workflows, ratings)
}

pub fn part1(input: &str) -> String {
    let mut sum: u32 = 0;
    let (workflows, ratings) = parse(input);

    for rating in ratings {
        let mut rules = &workflows["in"];
        let mut v = "in";
        while v != "A" && v != "R" {
            for r in &workflows[v] {
                if r.is_valid_condition(&rating) {
                    v = r.dest;
                    break;
                }
            }
        } 
        if v == "A" {
            sum += rating.iter().sum::<u32>();
        }
    }

    sum.to_string()
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
        let input = include_str!("../../inputs/2023/day19-sample.txt");
        assert_eq!("19114", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day19.txt");
        assert_eq!("353553", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day19-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day19.txt");
        assert_eq!("", part2(input));
    }
}
