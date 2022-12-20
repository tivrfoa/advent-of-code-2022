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

fn get_key(minutes: usize, mask: usize, actions: &[Action; 2])
 -> String {
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

/// return max pressure released from this state
/// till the end
fn bt(
    valves: &[Valve],
	memo: &mut HashMap<String, usize>,
	mut mask: usize,
	actions: &[Action; 2],
    minutes: usize,
    mut curr_flow: usize,
) -> usize {
    if minutes == 26 {
        return curr_flow;
    }

	// check if all valves are already open
	if (0..valves.len())
		.map(|i| is_bit_set(mask, i))
		.find(|b| *b == false).is_none() {
		return curr_flow * (26 - minutes);
	}

	let key = get_key(minutes, mask, &actions);
	if let Some(flow) = memo.get(&key) {
		// println!("Found in memo. minutes = {minutes}");
		return *flow;
	}

	// corner case: both are trying to open the same valve
	if actions[0] == actions[1] && let Action::Open(_) = actions[0] {
		return usize::MIN;
	}

	// corner case: player is trying to open a valve that was
	// opened by another player
	for action in actions {
		if let Action::Open(i) = action {
			if valves[*i].is_used(mask) {
				return usize::MIN;
			}
		}
	}

	// If it reached here, then the actions can be performed

	let mut next_actions: [Vec<Action>; 2] = [vec![], vec![]];

	for i in 0..2 {
		match actions[i] {
			Action::DontMove => {
				// it doesn't need to get possible actions.
				// if it's in this state, it will stay like this
				// foreverrrrrrr
				next_actions[i].push(Action::DontMove);
			}
			Action::Open(idx) => {
				mask = toggle_bit(mask, idx);
				curr_flow += valves[idx].flow_rate;

				for conn in &valves[idx].conn_indexes {
					next_actions[i].push(Action::Move(idx, *conn));
				}
			}
			Action::Move(from, to) => {
				if !is_bit_set(mask, to) && valves[to].flow_rate > 0 {
					next_actions[i].push(Action::Open(to));
				}

				for conn in &valves[to].conn_indexes {
					if *conn != from {
						next_actions[i].push(Action::Move(to, *conn));
					}
				}
			}
		}
	}

    let mut max = 0;

	for a1 in &next_actions[0] {
		for a2 in &next_actions[1] {
			let pressure = bt(valves,
				memo,
				mask,
				&[a1.clone(), a2.clone()],
				minutes + 1,
				curr_flow,
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
	Move(usize, usize), // from -> to
	Open(usize), // current idx
}

impl Action {
	fn to_string(&self) -> String {
		use Action::*;
		match self {
			DontMove => "DM".to_string(),
			Move(from, to) => {
				let mut s = "MV".to_string();
				s.push_str(&to.to_string());
				s
			}
			Open(i) => {
				let mut s = "OP".to_string();
				s.push_str(&i.to_string());
				s
			}
		}
	}
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

pub fn solve(input: String) -> usize {
    let valves = parse_input(input);

    let start_idx = valves.iter().position(|v| v.label == "AA").unwrap();

	// Memoize maximum pressure it get from a particular:
	// time-used_mask-action_a-action_b
	let mut memo: HashMap<String, usize> = HashMap::new();
	let mask: usize = 0;

    // I'll use backtrack
    let ans = bt(&valves,
		&mut memo,
		mask,
		&[Action::Move(start_idx, start_idx), Action::Move(start_idx, start_idx)],
		0,
		0);
	println!("{:?}", memo);
	println!("memo len = {}", memo.len());

	ans
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

    //#[test]
    //fn part1_input() {
    //    let input = util::read_file("inputs/day16.txt");
    //    assert_eq!(1845, solve(input));
    //}
}
