use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

#[derive(Debug, Eq, Hash, PartialEq)]
struct State {
    row_idx: u8,
    group_idx: u8,
    cont: u8,
    prev: u8,
}

impl State {
    fn add(&self, c: u8) -> Self {
        State {
            cont: if c == b'#' { self.cont + 1 } else { 0 },
            row_idx: self.row_idx + 1,
            group_idx: self.group_idx,
            prev: c,
        }
    }

    fn advance_group(&mut self) {
        self.group_idx += 1;
        self.cont = 0;
    }
}

fn dfs(state: State, mem: &mut HashMap<State, u64>, row: &[u8], groups: &[u8]) -> u64 {
    let rlen = row.len() as u8;
    let glen = groups.len() as u8;

    if state.row_idx == rlen {
        return if state.group_idx == glen { 1 } else { 0 };
    }

    if let Some(qt) = mem.get(&state) {
        return *qt;
    }

    if state.group_idx == glen {
        // all groups were satisfied. Check if the remaining string
        // is valid
        for i in state.row_idx..rlen {
            if row[i as usize] == b'#' {
                // invalid arrangement
                mem.insert(state, 0);
                return 0;
            }
        }
        mem.insert(state, 1);
        return 1;
    }

    let target = groups[state.group_idx as usize];

    // Handle it in current idx, so state.cont will never be
    // == target

    let qt = match row[state.row_idx as usize] {
        b'.' => {
            if state.cont > 0 {
                mem.insert(state, 0);
                return 0;
            }
            let new_state = state.add(b'.');
            dfs(new_state, mem, row, groups)
        }
        b'#' => {
            if state.cont + 1 == target {
                // end of group
                if state.row_idx + 1 < rlen && row[state.row_idx as usize + 1] == b'#' {
                    mem.insert(state, 0);
                    return 0;
                }

                let mut new_state = state.add(b'#');
                new_state.advance_group();
                dfs(new_state, mem, row, groups)
            } else {
                let new_state = state.add(b'#');
                dfs(new_state, mem, row, groups)
            }
        }
        b'?' => {
            if state.cont == 0 {
                if state.row_idx > 0 && state.prev == b'#' {
                    // changed group in previous state
                    // it cannot be # here
                    let new_state = state.add(b'.');
                    dfs(new_state, mem, row, groups)
                } else {
                    // ? can be '.' or '#'

                    // '#'
                    let mut qt = 0;
                    if state.cont + 1 == target {
                        if !(state.row_idx + 1 < rlen && row[state.row_idx as usize + 1] == b'#') {
                            let mut new_state = state.add(b'#');
                            new_state.advance_group();
                            qt += dfs(new_state, mem, row, groups);
                        }
                    } else {
                        let new_state = state.add(b'#');
                        qt += dfs(new_state, mem, row, groups);
                    }

                    // '.'
                    let new_state = state.add(b'.');
                    qt += dfs(new_state, mem, row, groups);

                    qt
                }
            } else {
                // this ? can only be a '#' because we still need to match
                // current group
                if state.cont + 1 == target {
                    if state.row_idx + 1 < rlen && row[state.row_idx as usize + 1] == b'#' {
                        mem.insert(state, 0);
                        return 0;
                    }

                    let mut new_state = state.add(b'#');
                    new_state.advance_group();
                    dfs(new_state, mem, row, groups)
                } else {
                    let new_state = state.add(b'#');
                    dfs(new_state, mem, row, groups)
                }
            }
        }
        c @ _ => panic!("{}", c),
    };

    mem.insert(state, qt);
    qt
}

pub fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let (l, r) = line.split_once(' ').unwrap();
        let row: &[u8] = l.as_bytes();
        let groups: Vec<u8> = r.split_to_nums(',');
        let mut mem: HashMap<State, u64> = HashMap::new();
        let start_state = State {
            cont: 0,
            row_idx: 0,
            group_idx: 0,
            prev: b' ',
        };
        let qt = dfs(start_state, &mut mem, row, &groups);
        sum += qt;
    }

    sum.to_string()
}

fn expand_row(srow: &str) -> Vec<u8> {
    let mut row = srow.as_bytes().to_vec();
    let init_row = row.clone();
    for _ in 0..4 {
        row.push(b'?');
        row.append(&mut init_row.clone());
    }
    row
}

fn expand_groups(mut group: Vec<u8>) -> Vec<u8> {
    let init_group = group.clone();
    for _ in 0..4 {
        group.append(&mut init_group.clone());
    }
    group
}

pub fn part2(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        println!("{line}");
        let (l, r) = line.split_once(' ').unwrap();
        let row: Vec<u8> = expand_row(l);
        let groups: Vec<u8> = expand_groups(r.split_to_nums(','));
        let mut mem: HashMap<State, u64> = HashMap::new();
        let start_state = State {
            cont: 0,
            row_idx: 0,
            group_idx: 0,
            prev: b' ',
        };
        let qt = dfs(start_state, &mut mem, &row, &groups);
        sum += qt;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = include_str!("../../inputs/2023/day12-sample.txt");
        assert_eq!("21", part1(input));
    }

    #[test]
    fn p1() {
        let input = include_str!("../../inputs/2023/day12.txt");
        assert_eq!("7792", part1(input));
    }

    #[test]
    fn p2s() {
        let input = include_str!("../../inputs/2023/day12-sample.txt");
        assert_eq!("525152", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day12.txt");
        assert_eq!("13012052341533", part2(input));
    }
}
