use crate::util;

use std::char::MAX;
use std::collections::{HashMap, VecDeque};
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

// const MAX_MINUTES: usize = 6; // best 143?
// const MAX_MINUTES: usize = 7; // best 184?
// const MAX_MINUTES: usize = 8; // best 260?
const MAX_MINUTES: usize = 26;

/// return max pressure released from this state
/// till the end
fn bt(
    valves: &[Valve],
    graph: &[Vec<(usize, usize)>],
    mut mask: usize,
    players: &[Option<Player>; 2],
) -> usize {
    // check if time finished for all players
    if players.iter().filter(|o| o.is_none()).count() == players.len() {
        return 0;
    }

    // If it reached here, then the actions can be performed

    let mut next_actions: [Vec<Option<Player>>; 2] = [vec![None], vec![None]];
    let mut flow_released = 0;

    for (i, player) in players.iter().enumerate() {
        if let Some(player) = player {
            let idx = player.valve_to_open;
            let mut open_minute = 0;
            if player.minutes > 0 {
                if valves[idx].is_used(mask) {
                    // corner case: player is trying to open a valve that was
                    // opened by another player
                    return usize::MIN;
                }
                mask = toggle_bit(mask, idx);
                open_minute = 1;
                flow_released +=
                    valves[idx].flow_rate * (MAX_MINUTES - (player.minutes + open_minute));
            }

            for (conn_idx, mut cost) in &graph[idx] {
                cost += open_minute;
                if player.minutes + cost < MAX_MINUTES - 1 && !is_bit_set(mask, *conn_idx) {
                    next_actions[i].push(Some(player.open(*conn_idx, cost)));
                }
            }
        }
    }

    let mut max = 0;

    for a1 in &next_actions[0] {
        for a2 in &next_actions[1] {
            let pressure = bt(valves, graph, mask, &[a1.clone(), a2.clone()]);
            if pressure > max {
                max = pressure;
            }
        }
    }

    flow_released += max;

    flow_released
}

#[derive(Copy, Clone, PartialEq)]
struct Player {
    minutes: usize,
    valve_to_open: usize,
}

impl Player {
    fn open(&self, index: usize, cost: usize) -> Self {
        Self {
            minutes: self.minutes + cost,
            valve_to_open: index,
        }
    }
}

fn dfs(valves: &[Valve], costs: &mut Vec<usize>, curr_idx: usize, curr_cost: usize) {
    let mut new_adj = vec![];
    for adj in &valves[curr_idx].conn_indexes {
        if curr_cost < costs[*adj] {
            costs[*adj] = curr_cost;
            new_adj.push(*adj);
        }
    }

    for n in new_adj {
        dfs(valves, costs, n, curr_cost + 1);
    }
}

/// make a graph keeping valves with flow rate > 0 and connecting
/// all valves with the cost (minutes) to get to them.
fn compress(valves: &[Valve]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = Vec::with_capacity(valves.len());
    for i in 0..valves.len() {
        let mut costs = vec![usize::MAX; valves.len()];
        costs[i] = 0;

        // The cost to open is applied when the action is performed
        // dfs(&valves, &mut costs, i, 1);

        // do bfs!
        let mut queue = VecDeque::new();
        valves[i]
            .conn_indexes
            .iter()
            .for_each(|i| queue.push_back((*i, 1)));
        while !queue.is_empty() {
            let (adj, cost) = queue.pop_front().unwrap();
            if cost < costs[adj] {
                costs[adj] = cost;
                valves[adj]
                    .conn_indexes
                    .iter()
                    .for_each(|i| queue.push_back((*i, cost + 1)));
            }
        }

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
    let mask: usize = 0;

    let player = Player {
        valve_to_open: start_idx,
        minutes: 0,
    };

    // I'll use backtrack
    let ans = bt(&valves, &graph, mask, &[Some(player.clone()), Some(player)]);

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
    fn part2_sample() {
        let input = util::read_file("inputs/day16-sample.txt");
        assert_eq!(1707, solve(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/day16.txt");
        assert_eq!(2286, solve(input));
    }
}
