use crate::{util, circular_list};

use core::cell::RefCell;
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;
use std::rc::Rc;

use util::*;
use crate::circular_list::{CircularList, Node};

pub fn part2(input: String) -> String {
    let mut nodes: Vec<Option<Rc<RefCell<Node<usize>>>>> = vec![None; 1_000_001];
    let mut chars = input.lines().next().unwrap().chars();
    let n = chars.next().unwrap().to_decimal();
    let head = Node::new_rc_node(n);
    let mut curr = head.clone();
    nodes[n] = Some(head.clone());
    for c in chars {
        let n = c.to_decimal();
        let next = Node::new_rc_node(n);
        nodes[n] = Some(next.clone());
        curr.append(next.clone());
        curr = next;
    }

    const N: usize = 1_000_000;

    for i in 10..=N {
        let next = Node::new_rc_node(i);
        nodes[i] = Some(next.clone());
        curr.append(next.clone());
        curr = next;
    }

    curr = head;
	let mut taken_n = vec![0, 0, 0];

    for _ in 0..N * 10 {
        let taken = curr.popn(3);
        taken.set_n_values(3, &mut taken_n);

        let mut current_label = curr.borrow().n - 1;
        if current_label == 0 {
            current_label = N;
        }
        while taken_n.contains(&current_label) {
            current_label -= 1;
            if current_label == 0 {
                current_label = N;
            }
        }

        nodes[current_label].as_ref().unwrap().pushn(taken, 3);

        curr = curr.next().unwrap();
    }

    nodes[1].as_ref().unwrap().set_n_values(3, &mut taken_n);
    (taken_n[1] * taken_n[2]).to_string()
}

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day23-sample.txt");
        assert_eq!("149245887792", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day23.txt");
        assert_eq!("131152940564", part2(input));
    }
}
