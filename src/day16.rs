use crate::util;

use std::collections::HashMap;

#[derive(Debug)]
struct Valve {
    label: String, // just for debugging
    idx: usize,
    flow_rate: usize,
    conn_indexes: Vec<usize>,
}

fn parse_input(input: String) -> Vec<Valve> {
    let mut valves: Vec<Valve> = vec![];
    let mut valves_index_map: HashMap<&str, usize> = HashMap::new();
    let mut next_idx = 0;
    for line in input.lines() {
        let (_, rem) = line.split_once("Valve ").unwrap();
        let (valve_label, rem) = rem.split_once(" has flow rate=").unwrap();
        let (flow_rate, rem) = rem.split_once(';').unwrap();
        let conn = match rem.split_once("to valves ") {
            Some((_, c)) => c,
            None => rem.split_once("to valve ").unwrap().1,
        };

        if !valves_index_map.contains_key(valve_label) {
            valves_index_map.insert(valve_label, next_idx);
            next_idx += 1;
        }

        let mut conn_indexes: Vec<usize> = Vec::new();
        for c in conn.split(", ") {
            match valves_index_map.get(c) {
                Some(idx) => {
                    conn_indexes.push(*idx);
                }
                None => {
                    conn_indexes.push(next_idx);
                    valves_index_map.insert(c, next_idx);
                    next_idx += 1;
                }
            }
        }

        valves.push(Valve {
            label: valve_label.into(),
            idx: *valves_index_map.get(valve_label).unwrap(),
            flow_rate: flow_rate.parse().unwrap(),
            conn_indexes,
        });
    }

    // set in correct index
    valves.sort_by(|a, b| a.idx.cmp(&b.idx));

    valves
}

#[derive(PartialEq)]
enum Action {
    Open,
    Move,
    Start,
    Stay,
}

fn bt(
    valves: &mut Vec<Valve>,
    used_valves: &mut Vec<bool>,
    minutes: usize,
    previous_idx: usize,
    curr_idx: usize,
    last_action: Action,
    curr_flow: usize,
) -> usize {
    if minutes == 30 {
        return curr_flow;
    }
    let mut max = 0;

    // option 1 - open valve
    if !used_valves[curr_idx] && valves[curr_idx].flow_rate > 0 {
        used_valves[curr_idx] = true;

        let pressure = bt(
            valves,
            used_valves,
            minutes + 1,
            previous_idx,
            curr_idx,
            Action::Open,
            curr_flow + valves[curr_idx].flow_rate,
        );
        if pressure > max {
            max = pressure;
        }

        // undo
        used_valves[curr_idx] = false;
    }

    // option 2 - move to some connection
    let len = valves[curr_idx].conn_indexes.len();
    if len == 0 {
        let pressure = bt(
            valves,
            used_valves,
            minutes + 1,
            previous_idx,
            curr_idx,
            Action::Move, // used to avoid go to previous valve
            curr_flow,
        );
        if pressure > max {
            max = pressure;
        }
    } else {
        for i in 0..len {
            let idx = valves[curr_idx].conn_indexes[i];
            if idx == previous_idx && last_action == Action::Move {
                continue;
            }
            let pressure = bt(
                valves,
                used_valves,
                minutes + 1,
                curr_idx,
                idx,
                Action::Move,
                curr_flow,
            );
            if pressure > max {
                max = pressure;
            }
        }
    }

    curr_flow + max
}

pub fn solve(input: String) -> usize {
    let mut valves = parse_input(input);
    let mut used_valves = vec![false; valves.len()];

    // I'll use backtrack
    bt(&mut valves, &mut used_valves, 1, 0, 0, Action::Start, 0)
}

pub fn solve_part2(input: String, max: i64) -> i64 {
    0
}

#[allow(dead_code)]
fn dbg(grid: &Vec<Vec<char>>) {
    for item in grid {
        println!("{:?}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/day16-sample.txt");
        assert_eq!(1651, solve(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/day16.txt");
        assert_eq!(5181556, solve(input));
    }

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day16-sample.txt");
    //    assert_eq!(56000011, solve_part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day16.txt");
    //    assert_eq!(12817603219131, solve_part2(input));
    //}
}
