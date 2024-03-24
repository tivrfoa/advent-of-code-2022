use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

#[derive(Debug)]
struct Module<'a> {
    module_type: ModuleType,
    destinations: Vec<&'a str>,
}

#[derive(Clone, Debug)]
enum ModuleType {
    FlipFlop { // %
        on: bool,
    },
    Conjunction, // &
    Broadcast,
}

#[derive(Debug)]
struct Pulse<'a> {
    module_name: &'a str,
    low_intensity: bool,
}

/*
Need to use a queue!
*/

use ModuleType::*;

fn parse(input: &str) -> (HashMap<&str, Vec<&str>>, HashMap<&str, Module>) {
    let mut inputs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut modules: HashMap<&str, Module> = HashMap::new();

    for line in input.lines() {
        let (l, r) = line.split_once(" -> ").unwrap();
        let module_type = &l[..1];
        let module_name = &l[1..];
        let destinations: Vec<&str> = r.split(", ").collect();
        for d in &destinations {
            inputs.entry(d).or_insert(vec![]).push(module_name);
        }

        match module_type {
            "b" => {
                modules.insert(l, Module {
                    module_type: Broadcast,
                    destinations,
                });
            }
            "%" => {
                modules.insert(module_name, Module {
                    module_type: FlipFlop { on: false, },
                    destinations,
                });
            }
            "&" => {
                modules.insert(module_name, Module {
                    module_type: Conjunction,
                    destinations,
                });
            }
            _ => panic!("{l}"),
        }
    }

    (inputs, modules)
}

pub fn part1(input: &str) -> String {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let (inputs, mut modules) = parse(input);
    dbg!(&inputs);
    dbg!(&modules);

    for _ in 0..1000 {
    // for _ in 0..1 {
        let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();
        pulse_queue.push_back(Pulse {
            module_name: "broadcaster",
            low_intensity: true,
        });
        while let Some(pulse) = pulse_queue.pop_front() {
            if pulse.low_intensity {
                low_pulses += 1;
            } else {
                high_pulses += 1;
            }
            println!("{} {}", if pulse.low_intensity { "low" } else { "high"}, pulse.module_name);
            if !modules.contains_key(pulse.module_name) {
                continue;
            }
            match modules[pulse.module_name].module_type {
                FlipFlop { on } => {
                    if pulse.low_intensity {
                        let module = modules.get_mut(pulse.module_name).unwrap();
                        match &mut module.module_type {
                            FlipFlop { on } => *on = !*on,
                            Conjunction => panic!(),
                            Broadcast => panic!(),
                        }
                        let is_on = !on;
                        let send_low = if is_on { false } else { true };
                        for d in &module.destinations {
                            pulse_queue.push_back(Pulse {
                                module_name: d,
                                low_intensity: send_low,
                            });
                        }
                    }
                },
                Conjunction => {
                    let is_all_high = is_all_high(&modules, &inputs, pulse.module_name);
                    
                    if is_all_high {
                        for d in &modules[pulse.module_name].destinations {
                            pulse_queue.push_back(Pulse {
                                module_name: d,
                                low_intensity: true,
                            });
                        }
                    } else {
                        for d in &modules[pulse.module_name].destinations {
                            pulse_queue.push_back(Pulse {
                                module_name: d,
                                low_intensity: false,
                            });
                        }
                    }
                },
                Broadcast => {
                    for d in &modules[pulse.module_name].destinations {
                        pulse_queue.push_back(Pulse {
                            module_name: d,
                            low_intensity: true,
                        });
                    }
                },
            }
        }
    }
dbg!(low_pulses, high_pulses);
    (low_pulses * high_pulses).to_string()
}

fn is_all_high(modules: &HashMap<&str, Module<'_>>, inputs: &HashMap<&str, Vec<&str>>, module_name: &str) -> bool {
    for input in &inputs[module_name] {
        let module = &modules[input];
        match module.module_type {
            FlipFlop { on } => {
                if !on {
                    return false;
                }
            },
            Conjunction => {
                // conjunction: sp rn
                if !is_all_high(modules, inputs, input) {
                    return false;
                }
            },
            Broadcast => todo!(),
        }
    }

    true
}

pub fn part2(input: &str) -> String {
    "".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2023/day20-sample.txt");
        assert_eq!("32000000", part1(input));
    }

    // #[test]
    // fn p12s() {
    //     let input = include_str!("../../inputs/2023/day20-sample2.txt");
    //     assert_eq!("11687500", part1(input));
    // }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day20.txt");
        assert_eq!("", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day20-sample.txt");
        assert_eq!("", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day20.txt");
        assert_eq!("", part2(input));
    }
}
