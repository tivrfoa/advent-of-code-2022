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
    cont: u16,
    row_idx: usize,
    group_idx: usize,
    seq: Vec<u8>
}

impl State {
    fn add(&self, c: u8) -> Self {
        let mut seq = self.seq.clone();
        seq.push(c);
        State {
            cont: if c == b'#' { self.cont + 1 } else { 0 },
            row_idx: self.row_idx + 1,
            group_idx: self.group_idx,
            seq,
        }
    }

    fn advance_group(&mut self) {
        self.group_idx += 1;
        self.cont = 0;
    }
}

fn dfs(state: State, mem: &mut HashMap<State, u32>, row: &[u8],
        groups: &[u16]) -> u32 {
    let rlen = row.len();
    let glen = groups.len();

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
            if row[i] == b'#' {
                // invalid arrangement
                mem.insert(state, 0);
                return 0;
            }
        }
        mem.insert(state, 1);
        return 1;
    }

    let target = groups[state.group_idx];

    // Handle it in current idx, so state.cont will never be
    // == target

    let qt = match row[state.row_idx] {
        b'.' => {
            if state.cont > 0 {
                mem.insert(state, 0);
                return 0;
            }
            let mut new_state = state.add(b'.');
            dfs(new_state, mem, row, groups)
        }
        b'#' => {
            if state.cont + 1 == target {
                // end of group
                if state.row_idx + 1 < rlen && row[state.row_idx + 1] == b'#' {
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
                if state.row_idx > 0 && state.seq[state.seq.len() - 1] == b'#' {
                    // changed group in previous state
                    // it cannot be # here
                    let new_state = state.add(b'.');
                    dfs(new_state, mem, row, groups)
                } else {
                    // ? can be '.' or '#'

                    // '#'
                    let mut qt = 0;
                    if state.cont + 1 == target {
                        if !(state.row_idx + 1 < rlen && row[state.row_idx + 1] == b'#') {
                            let mut new_state = state.add(b'#');
                            new_state.advance_group();
                            qt += dfs(new_state, mem, row, groups);
                        }
                    } else {
                        let new_state = state.add(b'#');
                        qt += dfs(new_state, mem, row, groups);
                    }

                    // '.'
                    let mut new_state = state.add(b'.');
                    qt += dfs(new_state, mem, row, groups);

                    qt
                }
            } else {
                // this ? can only be a '#' because we still need to match
                // current group
                if state.cont + 1 == target {
                    if state.row_idx + 1 < rlen && row[state.row_idx + 1] == b'#' {
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
        let groups: Vec<u16> = r.split_to_nums(',');
        let mut mem: HashMap<State, u32> = HashMap::new();
        let start_state = State {
            cont: 0,
            row_idx: 0,
            group_idx: 0,
            seq: vec![],
        };
        let qt = dfs(start_state, &mut mem, row, &groups);
        sum += qt;
    }

    sum.to_string()
}

fn expand_row(srow: &str) -> Vec<u8> {
    let mut row = srow.as_bytes().to_vec();
    let init_row = row.clone();
    for i in 0..4 {
        row.push(b'?');
        row.append(&mut init_row.clone());
    }
    row
}

fn expand_groups(mut group: Vec<u16>) -> Vec<u16> {
    let init_group = group.clone();
    for i in 0..4 {
        group.append(&mut init_group.clone());
    }
    group
}

pub fn part2(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let (l, r) = line.split_once(' ').unwrap();
        let row: Vec<u8> = expand_row(l);
        let groups: Vec<u16> = expand_groups(r.split_to_nums(','));
        let mut mem: HashMap<State, u32> = HashMap::new();
        let start_state = State {
            cont: 0,
            row_idx: 0,
            group_idx: 0,
            seq: vec![],
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
        assert_eq!("", part2(input));
    }
}
