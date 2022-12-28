use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};

#[derive(Copy, Clone, Debug)]
struct State {
    robots: [u16; 4],    // number of robots of each type
    resources: [u16; 4], // number of resources of each type
    minutes: u16,        // minutes used to get to this state
}

fn cp(state: &State, bp: &Blueprint) -> Option<State> {
    let mut s = State::get_start_state();

    for (i, r) in state.robots.iter().enumerate() {
        s.robots[i] = *r;
    }

    for (i, r) in state.resources.iter().enumerate() {
        s.resources[i] = *r;
    }

    s.minutes = state.minutes;

    if s.resources[0] >= bp.clay {
        s.minutes += 1;
        s.robots[1] += 1;
        s.resources[0] -= bp.clay;

        for (i, r) in state.robots.iter().enumerate() {
            s.resources[i] += r;
        }

        Some(s)
    } else {
        None
    }
}

impl State {
    fn get_start_state() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
            minutes: 0,
        }
    }

    //fn cp(&self) -> Self {
    //    let mut s = Self::get_start_state();

    //    for (i, r) in self.robots.iter().enumerate() {
    //        s.robots[i] = *r;
    //    }

    //    for (i, r) in self.resources.iter().enumerate() {
    //        s.resources[i] = *r;
    //    }

    //    s.minutes = self.minutes;

    //    s
    //}

    //fn produce_geode(&self, bp: &Blueprint) -> Option<Self> {
    //    if self.resources[0] >= bp.geode.0 && self.resources[2] >= bp.geode.1 {
    //        let mut clone = self.clone();
    //        clone.minutes += 1;
    //        clone.robots[3] += 1;
    //        clone.resources[0] -= bp.geode.0;
    //        clone.resources[2] -= bp.geode.1;

    //        for (i, r) in self.robots.iter().enumerate() {
    //            clone.resources[i] += r;
    //        }

    //        Some(clone)
    //    } else {
    //        None
    //    }
    //}

    //fn produce_obsidan(&self, bp: &Blueprint) -> Option<Self> {
    //    if self.resources[0] >= bp.obsidian.0 && self.resources[1] >= bp.obsidian.1 {
    //        let mut clone = self.clone();
    //        clone.minutes += 1;
    //        clone.robots[2] += 1;
    //        clone.resources[0] -= bp.obsidian.0;
    //        clone.resources[1] -= bp.obsidian.1;

    //        for (i, r) in self.robots.iter().enumerate() {
    //            clone.resources[i] += r;
    //        }

    //        Some(clone)
    //    } else {
    //        None
    //    }
    //}

    fn produce_resources_only(mut self) -> Self {
        self.minutes += 1;

        for (i, r) in self.robots.iter().enumerate() {
            self.resources[i] += r;
        }

        self
    }
}

#[derive(Debug)]
struct Blueprint {
    ore: u16,
    clay: u16,
    obsidian: (u16, u16), // ore and clay
    geode: (u16, u16),    // ore and obsidian
}

/// You have exactly one ore-collecting robot in your pack that
/// you can use to kickstart the whole operation.
///
/// max minutes: 24
/// it takes 1 min to collect a resource
/// it takes 1 min to construct a robot
/// Resource collection and robot construction can be done at the same minute.
///
/// ore      needs ores
/// clay     needs ores
/// obsidian needs ore and clay
/// geode    needs ore and obsidian
///
///
///
/// The decision is which robot to build with the available resources ...
/// At minute 23, the only thing that makes sense to build is a geode robot.
///
///
/// Determine the quality level of each blueprint by multiplying that
/// blueprint's ID number with the largest number of geodes that can be
/// opened in 24 minutes using that blueprint.
pub fn part1(input: String) -> String {
    let blueprints: Vec<Blueprint> = parse(input);
    let mut geodes: Vec<u16> = Vec::with_capacity(blueprints.len());

    for bp in &blueprints {
        let mut state = State::get_start_state();

        let mut states: VecDeque<State> = VecDeque::new();
        states.push_back(state);

        while let Some(state) = states.pop_front() {
            // dbg!(&state);

            if state.minutes == 24 {
                let state = state.produce_resources_only();
                // todo best max
                geodes.push(state.resources[3]);

                break;
            }

            // if let Some(s) = state.produce_geode(bp) {
            //     states.push_back(s);
            // }

            // if let Some(s) = state.produce_obsidan(bp) {
            //     states.push_back(s);
            // }

            if let Some(s) = cp(&state, bp) {
                // println!("{}", states.len());
                states.push_back(s);
                // states.push_front(s);
            }

            states.push_back(state.produce_resources_only());
        }
    }

    "".into()
}

pub fn part2(input: String) -> String {
    todo!()
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/day19-sample.txt");
        assert_eq!("33", part1(input));
    }

    //#[test]
    //fn part1_input() {
    //    let input = util::read_file("inputs/day19.txt");
    //    assert_eq!("", part1(input));
    //}

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

fn parse(input: String) -> Vec<Blueprint> {
    let mut blueprints = vec![];

    for line in input.lines() {
        let (_, rem) = line.split_once(": ").unwrap();
        let tokens: Vec<&str> = rem.split_ascii_whitespace().collect();
        let ore = tokens[4].parse::<u16>().unwrap();
        let clay = tokens[10].parse::<u16>().unwrap();
        let obsidian_ore = tokens[16].parse::<u16>().unwrap();
        let obsidian_clay = tokens[19].parse::<u16>().unwrap();
        let geode_ore = tokens[25].parse::<u16>().unwrap();
        let geode_obsidian = tokens[28].parse::<u16>().unwrap();

        blueprints.push(Blueprint {
            ore,
            clay,
            obsidian: (obsidian_ore, obsidian_clay),
            geode: (geode_ore, geode_obsidian),
        });
    }

    blueprints
}
