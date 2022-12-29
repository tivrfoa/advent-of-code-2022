use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

#[derive(Debug)]
struct Blueprint {
    recipes: [Vec<(i32, usize)>; 4],
    max_spend: [i32; 3], // ore, clay, obsidian
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    robots: [i32; 4],    // number of robots of each type
    resources: [i32; 4], // number of resources of each type
    minutes: i32,        // minutes used to get to this state
}

impl State {
    fn get_start_state() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
            minutes: 24,
        }
    }
}

fn dfs(bp: &Blueprint, cache: &mut HashMap<State, i32>, state: State) -> i32 {
    //println!("Running minute: {}", state.minutes);
    if state.minutes == 0 {
        dbg!(state);
        return state.resources[3];
    }

    if let Some(v) = cache.get(&state) {
        return *v;
    }

    // starts with the case that we do nothing
    let mut max = state.resources[3] + state.robots[3] * state.minutes;

    'lr: for (btype, recipe) in bp.recipes.iter().enumerate() {
        if btype != 3 && state.robots[btype] >= bp.max_spend[btype] {
            continue;
        }

        let mut wait = 0;
        for (ramt, rtype) in recipe {
            if state.robots[*rtype] == 0 {
                continue 'lr;
            }

            // let div = ceil_div2(*ramt, state.resources[*rtype], state.robots[*rtype]);
            // wait = wait.max(div);
            // if *ramt >= state.resources[*rtype] {
            wait = wait.max((*ramt - state.resources[*rtype]).div_ceil(state.robots[*rtype]));
            // }
        }

        let remtime = state.minutes - wait - 1;
        if remtime <= 0 {
            continue;
        }
        let mut new_state = state.clone();
        new_state.minutes = remtime;

        new_state.resources = zip(state.resources, state.robots)
            .map(|(x, y)| x + y * (wait + 1))
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();

        for (ramt, rtype) in recipe {
            new_state.resources[*rtype] -= ramt;
        }
        new_state.robots[btype] += 1;

        for i in 0..3 {
            // new_state.resources[i].min((bp.max_spend[i] - new_state.robots[i]) * remtime);
            new_state.resources[i] = new_state.resources[i].min(bp.max_spend[i] * remtime);
        }

        max = max.max(dfs(bp, cache, new_state));
    }

    cache.insert(state, max);
    max
}

fn ceil_div2(a: i32, b: i32, c: i32) -> i32 {
    let a = a as i32;
    let b = b as i32;
    let a = -(a - b);
    let c = c as i32;
    let d = -(a / c);

    if d < 0 {
        0
    } else {
        d as i32
    }
}

pub fn part1(input: String) -> String {
    let blueprints = parse(input);

    let mut total: i32 = 0;
    for (i, bp) in blueprints.iter().enumerate() {
        dbg!(bp);
        let mut cache: HashMap<State, i32> = HashMap::new();
        let v = dfs(bp, &mut cache, State::get_start_state());
        println!("{v}");
        // TODO bug in blueprints: 14 and 23: they should return 1
        total += (i as i32 + 1) * v;
    }

    total.to_string()
}

pub fn part2(input: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/day19-sample.txt");
        assert_eq!("33", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/day19.txt");
        assert_eq!("978", part1(input));
    }

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day19-sample.txt");
    //    assert_eq!("", part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day19.txt");
    //    assert_eq!("", part2(input));
    //}
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

fn parse(input: String) -> Vec<Blueprint> {
    let mut blueprints = vec![];

    for line in input.lines() {
        let (_, rem) = line.split_once(": ").unwrap();
        let tokens: Vec<&str> = rem.split_ascii_whitespace().collect();
        let ore = tokens[4].parse::<i32>().unwrap();
        let clay = tokens[10].parse::<i32>().unwrap();
        let obsidian_ore = tokens[16].parse::<i32>().unwrap();
        let obsidian_clay = tokens[19].parse::<i32>().unwrap();
        let geode_ore = tokens[25].parse::<i32>().unwrap();
        let geode_obsidian = tokens[28].parse::<i32>().unwrap();

        let max_spend = [
            ore.max(clay.max(obsidian_ore.max(geode_ore))),
            clay.max(obsidian_clay),
            geode_obsidian,
        ];

        blueprints.push(Blueprint {
            recipes: [
                vec![(ore, 0)],
                vec![(clay, 0)],
                vec![(obsidian_ore, 0), (obsidian_clay, 1)],
                vec![(geode_ore, 0), (geode_obsidian, 2)],
            ],
            max_spend,
        });
        //blueprints.push(Blueprint {
        //    ore,
        //    clay,
        //    obsidian: (obsidian_ore, obsidian_clay),
        //    geode: (geode_ore, geode_obsidian),
        //    max_spend: [
        //        ore.max(obsidian_ore.max(geode_ore)),
        //        clay.max(obsidian_clay),
        //        geode_obsidian,
        //    ],
        //});
    }

    blueprints
}
