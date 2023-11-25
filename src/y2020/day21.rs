use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn filter<'a>(allergens: &mut HashMap<&'a str, Vec<&str>>, a: &'a str, ingredients: &Vec<&str>) {
    let mut retain = vec![];
    if let Some(foods) = allergens.get(a) {
        for f in foods {
            if ingredients.contains(f) {
                retain.push(*f);
            }
        }
    } else {
        panic!("nothing to filter ...");
    }
    allergens.insert(a, retain);
}

fn part1(input: String) -> String {
    let mut allergens: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut ingredients: Vec<&str> = vec![];
    let mut used: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        let (ff, aa) = line.split_once(" (contains ").unwrap();
        let mut ff: Vec<&str> = ff.split(" ").filter(|f| !used.contains(f)).collect();
        let aa = aa.split_once(')').unwrap().0;
        let aa: Vec<&str> = aa.split(", ").collect();

        for a in &aa {
            if allergens.contains_key(a) {
                filter(&mut allergens, a, &ff);
                if allergens[a].len() == 1 {
                    let mut foods_to_remove: Vec<(&str, &str)> = vec![(a, allergens[a][0])];
                    while let Some((key, used_food)) = foods_to_remove.pop() {
                        used.insert(used_food);
                        // remove this food from other lists
                        for (k, v) in allergens.iter_mut() {
                            if k == &key {
                                continue;
                            }
                            if let Some(pos) = v.iter().position(|f| f == &used_food) {
                                v.remove(pos);
                                if v.len() == 1 {
                                    foods_to_remove.push((k, v[0]));
                                }
                            }
                        }
                    }
                }
            } else {
                allergens.insert(a, ff.clone());
            }
        }

        ingredients.append(&mut ff);
        dbg!(&allergens, &used);
    }

    ingredients
        .into_iter()
        .filter(|i| !used.contains(i))
        .count()
        .to_string()
}

fn part2(input: String) -> String {
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
        let input = util::read_file("inputs/2020/day21-sample.txt");
        assert_eq!("5", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day21.txt");
        assert_eq!("2211", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day21-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day21.txt");
        assert_eq!("", part2(input));
    }
}
