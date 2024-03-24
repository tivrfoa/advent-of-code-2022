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
    module_type: ModuleType<'a>,
    destinations: Vec<&'a str>,
}

#[derive(Debug)]
enum ModuleType<'a> {
    FlipFlop { // %
        on: bool,
    },
    Conjunction { // &
        memory: HashMap<&'a str, PulseType>,
    },
    Broadcast,
}

#[derive(Debug)]
struct Pulse<'a> {
    module_name: &'a str,
    sender: &'a str,
    pulse_type: PulseType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PulseType {
    HIGH,
    LOW,
}

/*
Need to use a queue!
*/

use ModuleType::*;

fn parse(input: &str) -> HashMap<&str, Module> {
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
                    module_type: Conjunction { memory: HashMap::new() },
                    destinations,
                });
            }
            _ => panic!("{l}"),
        }
    }

    for (k, v) in modules.iter_mut() {
        if let Conjunction { memory } = &mut v.module_type {
            for i in &inputs[k] {
                memory.insert(i, PulseType::LOW);
            }
        }
    }

    modules
}

pub fn part1(input: &str) -> String {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut modules = parse(input);

    for _ in 0..1000 {
        let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();
        pulse_queue.push_back(Pulse {
            module_name: "broadcaster",
            sender: "",
            pulse_type: PulseType::LOW,
        });
        while let Some(pulse) = pulse_queue.pop_front() {
            match pulse.pulse_type {
                PulseType::LOW => low_pulses += 1,
                PulseType::HIGH => high_pulses += 1,
            }
            if !modules.contains_key(pulse.module_name) {
                continue;
            }

            let module = modules.get_mut(pulse.module_name).unwrap();
            match &mut module.module_type {
                FlipFlop { on } => {
                    if let PulseType::LOW = pulse.pulse_type {
                        *on = !*on;
                        let pulse_type = if *on { PulseType::HIGH } else { PulseType::LOW };
                        for d in &module.destinations {
                            pulse_queue.push_back(Pulse {
                                module_name: d,
                                sender: pulse.module_name,
                                pulse_type,
                            });
                        }
                    }
                },
                Conjunction { memory } => {
                    memory.insert(pulse.sender, pulse.pulse_type);
                    let pulse_type = if memory.iter().find(|(_k, v)| *v == &PulseType::LOW).is_none() {
                        PulseType::LOW
                    } else {
                        PulseType::HIGH
                    };
                    for d in &modules[pulse.module_name].destinations {
                        pulse_queue.push_back(Pulse {
                            module_name: d,
                            sender: pulse.module_name,
                            pulse_type,
                        });
                    }
                },
                Broadcast => {
                    for d in &modules[pulse.module_name].destinations {
                        pulse_queue.push_back(Pulse {
                            module_name: d,
                            sender: "broadcaster",
                            pulse_type: PulseType::LOW,
                        });
                    }
                },
            }
        }
    }

    (low_pulses * high_pulses).to_string()
}

pub fn part2(input: &str) -> String {
    let mut modules = parse(input);

    for i in 1..1_000_000_000 {
        let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();
        pulse_queue.push_back(Pulse {
            module_name: "broadcaster",
            sender: "",
            pulse_type: PulseType::LOW,
        });
        while let Some(pulse) = pulse_queue.pop_front() {
            if !modules.contains_key(pulse.module_name) {
                continue;
            }

            let module = modules.get_mut(pulse.module_name).unwrap();
            match &mut module.module_type {
                FlipFlop { on } => {
                    if let PulseType::LOW = pulse.pulse_type {
                        *on = !*on;
                        let pulse_type = if *on { PulseType::HIGH } else { PulseType::LOW };
                        for d in &module.destinations {
                            if *d == "rx" && pulse_type == PulseType::LOW {
                                return i.to_string();
                            }
                            pulse_queue.push_back(Pulse {
                                module_name: d,
                                sender: pulse.module_name,
                                pulse_type,
                            });
                        }
                    }
                },
                Conjunction { memory } => {
                    memory.insert(pulse.sender, pulse.pulse_type);
                    let pulse_type = if memory.iter().find(|(_k, v)| *v == &PulseType::LOW).is_none() {
                        PulseType::LOW
                    } else {
                        PulseType::HIGH
                    };
                    for d in &modules[pulse.module_name].destinations {
                        if *d == "rx" && pulse_type == PulseType::LOW {
                            return i.to_string();
                        }
                        pulse_queue.push_back(Pulse {
                            module_name: d,
                            sender: pulse.module_name,
                            pulse_type,
                        });
                    }
                },
                Broadcast => {
                    for d in &modules[pulse.module_name].destinations {
                        pulse_queue.push_back(Pulse {
                            module_name: d,
                            sender: "broadcaster",
                            pulse_type: PulseType::LOW,
                        });
                    }
                },
            }
        }
    }

    panic!("Failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2023/day20-sample.txt");
        assert_eq!("32000000", part1(input));
    }

    #[test]
    fn p12s() {
        let input = include_str!("../../inputs/2023/day20-sample2.txt");
        assert_eq!("11687500", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day20.txt");
        assert_eq!("886347020", part1(input));
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
