use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};

#[derive(Copy, Clone, Debug)]
struct State {
    robots: [u16; 4], // number of robots of each type
    resources: [u16; 4], // number of resources of each type
    minutes: u16, // minutes used to get to this state
}

impl State {
    fn get_start_state() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
            minutes: 0,
        }
    }

    fn greedy(&self, blueprint: &Blueprint) -> Self {
        let mut clone = self.clone();
        clone.minutes += 1;

        // first check if it can be something with resources

        // try geode first
        if clone.resources[0] >= blueprint.geode.0 &&
                clone.resources[2] >= blueprint.geode.1 {
            clone.robots[3] += 1;
            clone.resources[0] -= blueprint.geode.0;
            clone.resources[2] -= blueprint.geode.1;
        }

        // try obsidian
        if clone.resources[0] >= blueprint.obsidian.0 &&
                clone.resources[1] >= blueprint.obsidian.1 {
            clone.robots[2] += 1;
            clone.resources[0] -= blueprint.geode.0;
            clone.resources[1] -= blueprint.geode.1;
        }

        // try clay
        if clone.resources[0] >= blueprint.clay {
            clone.robots[1] += 1;
            clone.resources[0] -= blueprint.clay;
        }

        // greedy probably wont' work. Maybe it's better not create a ore robot
        // now
        if clone.resources[0] >= blueprint.ore {
            clone.robots[0] += 1;
            clone.resources[0] -= blueprint.ore;
        }

        for (i, r) in self.robots.iter().enumerate() {
            clone.resources[i] += r;
        }

        clone
    }
}

#[derive(Debug)]
struct Blueprint {
    ore: u16,
    clay: u16,
    obsidian: (u16, u16), // ore and clay
    geode: (u16, u16), // ore and obsidian
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
/// As the types of resources needed to build a robot are different, a greedy
/// approach to build a robot as soon as possible might work.
///
///
/// Determine the quality level of each blueprint by multiplying that
/// blueprint's ID number with the largest number of geodes that can be
/// opened in 24 minutes using that blueprint.
pub fn part1(input: String) -> String {


    // let's see how the greedy approach goes

    let blueprints: Vec<Blueprint> = parse(input);

    let mut state = State::get_start_state();

    for m in 1..=24 {
        dbg!(&state);
        state = state.greedy(&blueprints[0]);
    }
    dbg!(&state);


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
