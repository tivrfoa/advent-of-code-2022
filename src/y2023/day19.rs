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
    v: u64,
    comp: char,
    dest: &'a str,
}

type Ratings = [u64; 4];

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
                        (r_to_usize(r), v.parse::<u64>().unwrap(), '>', dest)
                    } else {
                        let (r, v) = l.split_once('<').unwrap();
                        (r_to_usize(r), v.parse::<u64>().unwrap(), '<', dest)
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
    let mut sum: u64 = 0;
    let (workflows, ratings) = parse(input);

    for rating in ratings {
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
            sum += rating.iter().sum::<u64>();
        }
    }

    sum.to_string()
}

#[derive(Clone)]
struct Range {
    l: u64,
    r: u64,
}

impl Range {
    fn new(l: u64, r: u64) -> Self {
        Self {
            l, r,
        }
    }
}

fn calc_product(ranges: &[Range; 4]) -> u64 {
    let mut mul = 1;
    for r in ranges {
        mul *= r.r - r.l + 1;
    }
    mul
}

struct State<'a> {
    cur: &'a str,
    rule_idx: usize,
    ranges: [Range; 4],
}

impl<'a> State<'a> {
    fn new(cur: &'a str, rule_idx: usize, x: (u64, u64), m: (u64, u64), a: (u64, u64), s: (u64, u64)) -> Self {
        Self {
            cur,
            rule_idx,
            ranges: [
                Range::new(x.0, x.1),
                Range::new(m.0, m.1),
                Range::new(a.0, a.1),
                Range::new(s.0, s.1),
            ],
        }
    }
}

fn initial_range() -> [Range; 4] {
    [
        Range::new(1, 4000),
        Range::new(1, 4000),
        Range::new(1, 4000),
        Range::new(1, 4000),
    ]
}

fn update_range(idx: usize, l: u64, r: u64, ranges: &[Range; 4]) -> [Range; 4] {
    let mut new_ranges = ranges.clone();
    new_ranges[idx].l = l;
    new_ranges[idx].r = r;

    new_ranges
}

pub fn part2(input: &str) -> String {
    let (workflows, _) = parse(input);
    let mut valid_ranges: Vec<[Range; 4]> = vec![];
    let mut states: Vec<State> = vec![
        State {
            cur: "in",
            rule_idx: 0,
            ranges: initial_range(),
        }
    ];

    while let Some(state) = states.pop() {
        if state.cur == "A" || state.cur == "R" {
            if state.cur == "A" {
                valid_ranges.push(state.ranges);
            }
            continue;
        }
        let rule = &workflows[state.cur][state.rule_idx];
        let l = state.ranges[rule.r].l;
        let r = state.ranges[rule.r].r;
        
        match rule.comp {
            '>' => {
                if r > rule.v {
                    if l > rule.v {
                        states.push(State {
                            cur: rule.dest,
                            rule_idx: 0,
                            ranges: state.ranges,
                        });
                    } else {
                        let new_ranges = update_range(rule.r, rule.v + 1, r, &state.ranges);
                        states.push(State {
                            cur: rule.dest,
                            rule_idx: 0,
                            ranges: new_ranges,
                        });

                        // else branch. Same workflow
                        let new_ranges = update_range(rule.r, l, rule.v, &state.ranges);
                        states.push(State {
                            cur: state.cur,
                            rule_idx: state.rule_idx + 1,
                            ranges: new_ranges,
                        });
                    }
                } else {
                    // go to else: next rule
                    states.push(State {
                        cur: state.cur,
                        rule_idx: state.rule_idx + 1,
                        ranges: state.ranges,
                    });
                }
            }
            '<' => {
                if r < rule.v {
                    states.push(State {
                        cur: rule.dest,
                        rule_idx: 0,
                        ranges: state.ranges,
                    });
                } else if l < rule.v {
                    // split

                    // if
                    let new_ranges = update_range(rule.r, l, rule.v - 1, &state.ranges);
                    states.push(State {
                        cur: rule.dest,
                        rule_idx: 0,
                        ranges: new_ranges,
                    });

                    // else branch
                    let new_ranges = update_range(rule.r, rule.v, r, &state.ranges);
                    states.push(State {
                        cur: state.cur,
                        rule_idx: state.rule_idx + 1,
                        ranges: new_ranges,
                    });
                } else {
                    // go to else: next rule
                    states.push(State {
                        cur: state.cur,
                        rule_idx: state.rule_idx + 1,
                        ranges: state.ranges,
                    });
                }
            }
            '=' => {
                states.push(State {
                    cur: rule.dest,
                    rule_idx: 0,
                    ranges: state.ranges,
                });
            }
            _ => panic!("{}", rule.comp),
        }
    }

    let mut ans = 0;
    for ranges in valid_ranges {
        ans += calc_product(&ranges);
    }

    ans.to_string()
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
        assert_eq!("167409079868000", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day19.txt");
        assert_eq!("124615747767410", part2(input));
    }
}
