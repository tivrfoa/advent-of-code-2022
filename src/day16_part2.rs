use crate::util;

use std::collections::HashMap;
use std::process;

#[derive(Debug)]
struct Valve {
    label: String,
    idx: usize,
    flow_rate: usize,
    conn_indexes: Vec<usize>,
}

impl Valve {
    fn is_used(&self, mask: usize) -> bool {
        check_bit(mask, self.idx) == 1
    }
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

fn get_key(minutes: usize, mask: usize, actions: &[Action; 2]) -> String {
    let mut key_parts: Vec<String> = Vec::with_capacity(4);
    key_parts.push(minutes.to_string());
    key_parts.push(mask.to_string());

    let mut a1 = actions[0].to_string();
    let mut a2 = actions[1].to_string();

    if a1 > a2 {
        std::mem::swap(&mut a1, &mut a2);
    }
    key_parts.push(a1);
    key_parts.push(a2);

    key_parts.join("-")
}

const MAX_MINUTES: usize = 26;

/// return max pressure released from this state
/// till the end
fn bt(
    valves: &[Valve],
    graph: &[(usize, usize)],
    memo: &mut HashMap<String, usize>,
    mut mask: usize,
    actions: &[Action; 2],
    curr_flow: usize,
    mut used_valves: usize,
    valves_with_flow_greater_than_zero: usize,
) -> usize {

    // check if time finished for all players
    {
        let mut fin = true;
        for a in actions {
            match a {
                DontMove(m) => if m <= MAX_MINUTES { // TODO check it it is only '<'
                    fin = false;
                    break;
                }
                Open(_, m) => if m <= MAX_MINUTES {
                    fin = false;
                    break;
                }
            }
        }

        if fin {
            return curr_flow;
        }
    }

    // check if all valves are already open
    // TODO what's wrong here. If I uncomment I get wrong result for sample
    if used_valves == valves_with_flow_greater_than_zero {
        // println!("Used all {used_valves} valves!");
        return curr_flow * (MAX_MINUTES - minutes + 1);
    }

    let key = get_key(minutes, mask, &actions);
    if let Some(flow) = memo.get(&key) {
        // println!("Found in memo. minutes = {minutes}");
        return *flow;
    }

    // corner case: both are trying to open the same valve
    if actions[0] == actions[1] && let Action::Open(_, min) = actions[0] {
        if min > 0 {
            return usize::MIN;
        }
    }

    // If it reached here, then the actions can be performed

    let mut next_actions: [Vec<Player>; 2] = [vec![], vec![]];
    let mut additional_flow = 0;

    for i in 0..2 {
        match actions[i].current_action {
            Action::DontMove => {
                // it doesn't need to get possible actions.
                // if it's in this state, it will stay like this
                // foreverrrrrrr
                next_actions[i].push(Action::DontMove(minutes + 1);
            }
            Action::Open(idx, minutes) => {
                if minutes > 0 {
                    if valves[idx].is_used(mask) {
                        // corner case: player is trying to open a valve that was
                        // opened by another player
                        return usize::MIN;
                    }
                    mask = toggle_bit(mask, idx);
                    additional_flow += valves[idx].flow_rate;
                    used_valves += 1;
                }

                let mut found_path = false;
                for (conn_idx, cost) in &graph[idx] {
                    if minutes + cost < MAX_MINUTES && !is_bit_set(mask, conn_idx) {
                        next_actions[i].push(Action::Open(*conn, minutes + cost));
                        found_path = true;
                    }
                }
                if !found_path {
                    next_actions[i].push(Action::DontMove(minutes + 1));
                }
            }
        }
    }

    let mut max = 0;

    for a1 in &next_actions[0] {
        for a2 in &next_actions[1] {
            let pressure = bt(
                valves,
                memo,
                mask,
                &[a1.clone(), a2.clone()],
                minutes + 1,
                curr_flow + additional_flow,
                used_valves,
                valves_with_flow_greater_than_zero,
            );
            if pressure > max {
                max = pressure;
            }
        }
    }

    let pressure_released = curr_flow + max;
    memo.insert(key, pressure_released);

    pressure_released
}

#[derive(Copy, Clone, PartialEq)]
enum Action {
    DontMove,
    Open(usize), // current idx
}

impl Action {
    fn to_string(&self) -> String {
        use Action::*;
        match self {
            DontMove => "DM".to_string(),
            Open(i) => {
                let mut s = "OP".to_string();
                s.push_str(&i.to_string());
                s
            }
        }
    }
}

struct Player {
    minutes_elapsed: usize,
    flow: usize,
    current_action: Action,
}

impl Player {
    fn to_string(&self) -> String {
        let mut s = "".to_string();
        s.push(self.minutes_elapsed);
        s.push(self.current_action.to_string());
        s
    }
}

fn visit(valves: &[Valve], costs: &mut Vec<usize>, curr_idx: usize, curr_cost: usize) {
    let mut new_adj = vec![];
    for adj in &valves[curr_idx].conn_indexes {
        if costs[*adj] == usize::MAX {
            new_adj.push(*adj);
        }
        if curr_cost < costs[*adj] {
            costs[*adj] = curr_cost;
        }
    }

    for n in new_adj {
        visit(valves, costs, n, curr_cost + 1);
    }
}

/// make a graph keeping valves with flow rate > 0 and connecting
/// all valves with the cost (minutes) to get to them.
fn compress(valves: &[Valve]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![];
    for (i, v) in valves.iter().enumerate() {
        let mut costs = vec![usize::MAX; valves.len()];
        costs[i] = 0;
        visit(&valves, &mut costs, i, 1);

        // dbg!(costs);

        let mut edges: Vec<(usize, usize)> = vec![];
        for (idx, c) in costs.iter().enumerate() {
            if i == idx || valves[idx].flow_rate == 0 {
                continue;
            }
            edges.push((idx, *c));
        }

        graph.push(edges);
    }

    graph
}

pub fn solve(input: String) -> usize {
    let valves = parse_input(input);
    let graph = compress(&valves);

    // dbg!(graph); // graph is fine!

    let start_idx = valves.iter().position(|v| v.label == "AA").unwrap();

    // Memoize maximum pressure it get from a particular:
    // time-used_mask-action_a-action_b
    let mut memo: HashMap<String, usize> = HashMap::new();
    let mask: usize = 0;
    let valves_with_flow_greater_than_zero = valves.iter().filter(|v| v.flow_rate > 0).count();

    // I'll use backtrack
    let ans = bt(
        &valves,
        &graph,
        &mut memo,
        mask,
        &[
            Action::Open(start_idx, 1),
            Action::Open(start_idx, 1),
        ],
        0,
        0,
        0,
        valves_with_flow_greater_than_zero,
    );

    println!("memo len = {}", memo.len());

    ans
}

fn toggle_bit(n: usize, bit: usize) -> usize {
    n ^ 1 << bit
}

/// bit = (number >> n) & 1U;
fn check_bit(n: usize, bit: usize) -> usize {
    (n >> bit) & 1
}

fn is_bit_set(n: usize, bit: usize) -> bool {
    (n >> bit) & 1 == 1
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
        assert_eq!(1707, solve(input));
    }

    //#[test]
    //fn part1_input() {
    //    let input = util::read_file("inputs/day16.txt");
    //    assert_eq!(1845, solve(input));
    //}
}
