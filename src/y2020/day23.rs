use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn pick(cups: &[usize], curr: usize, n: usize) -> Vec<usize> {
	let mut picked: Vec<usize> = Vec::with_capacity(n);
	let mut i = (curr + 1) % cups.len();
	for _ in 0..n {
		picked.push(cups[i]);
		i = (i + 1) % cups.len();
	}

	picked
}

// at most 8 cups move for part 1
static mut BUFFER: [(usize, usize); 8] = [(0, 0); 8];

fn update_cups(mut cups: Vec<usize>, curr: usize, picked: &[usize],
		destination_idx: usize) -> Vec<usize> {
	let len = cups.len();
	let mut pos_to_fill = (curr + 1) % len;
	let mut pos_to_look = (curr + 4) % len;
	let mut idx = 0;

	for _ in 0..4 {
		if pos_to_look == destination_idx {
			break;
		}
		unsafe {
			BUFFER[idx] = (pos_to_fill, cups[pos_to_look]);
			idx += 1;
		}
		pos_to_fill = (pos_to_fill + 1) % len;
		pos_to_look = (pos_to_look + 1) % len;
	}

	unsafe {
		BUFFER[idx] = (pos_to_fill, cups[pos_to_look]);
		idx += 1;
	}
	pos_to_fill = (pos_to_fill + 1) % len;

	for i in 0..picked.len() {
		unsafe {
			BUFFER[idx] = (pos_to_fill, picked[i]);
			idx += 1;
		}

		pos_to_fill = (pos_to_fill + 1) % len;
	}

	// update cups
	for i in 0..idx {
		unsafe {
			cups[BUFFER[i].0] = BUFFER[i].1;
		}
	}

	cups
}

fn find_destination(cups: &[usize], curr_idx: usize) -> usize {
	for target in (0..cups[curr_idx]).rev() {
		let mut i = (curr_idx + 4) % cups.len();
		while i != curr_idx {
			if cups[i] == target {
				return i;
			}

			i = (i + 1) % cups.len();
		}
	}

	// find highest
	let mut i = (curr_idx + 4) % cups.len();
	let mut max = 0;
	let mut max_idx = 0;
	while i != curr_idx {
		if cups[i] > max {
			max_idx = i;
			max = cups[i];
		}

		i = (i + 1) % cups.len();
	}

	max_idx
}

pub fn play(mut cups: Vec<usize>, times: usize) -> Vec<usize> {
	let len = cups.len();
	let mut curr = 0;

	for _ in 0..times {
		let picked = pick(&cups, curr, 3);
		let destination_idx = find_destination(&cups, curr);
		cups = update_cups(cups, curr, &picked, destination_idx);
		curr = (curr + 1) % len;
	}

	cups
}

pub fn part1(input: String) -> String {
	let cups: Vec<usize> = input.lines().next().unwrap().chars().map(|c| c.to_decimal()).collect();
	let cups = play(cups, 100);
	let len = cups.len();
	let cup1_pos = cups.iter().position(|&v| v == 1).unwrap();
	let mut ans = String::with_capacity(len - 1);

	for i in cup1_pos + 1..cup1_pos + len {
		ans.push_str(&cups[i % len].to_string());
	}

	ans
}

#[derive(Clone, Debug)]
struct Node {
    next: Cell<usize>,
}

impl Node {
	fn new(n: usize) -> Self {
		Self {
			next: n.into(),
		}
	}

	fn append(&self, n: usize) -> Self {
		let new_node = Node::new(n);
		new_node.next.replace(self.next.get());
		self.next.replace(n);

		new_node
	}

    fn popn(&self, nodes: &[Node], n: usize) -> usize {
        let mut next: usize = self.next.get();
        let taken = next;

        for _ in 0..n {
            next = nodes[next].next.get();
        }
        self.next.replace(next);

        taken
    }

    fn pushn(&self, nodes: &[Node], node: usize, n: usize) {
        let head = node;
        let mut next: usize = node;
        for _ in 0..n - 1 {
            next = nodes[next].next.get();
        }
        nodes[next].next.replace(self.next.get());
        self.next.replace(head);
    }
}

pub fn part2(input: String) -> String {
	let mut cups: Vec<Node> = vec![Node { next: 0.into() }; 1_000_001];
	let mut chars = input.lines().next().unwrap().chars();
	let mut current = chars.next().unwrap().to_decimal();
	cups[current] = Node::new(current);
	let mut prev = current;
	for c in chars {
		let n = c.to_decimal();
		cups[n] = cups[prev].append(n);
		prev = n;
	}

	const N: usize = 1_000_000;

	for i in 10..=N {
		cups[i] = cups[prev].append(i);
		prev = i;
	}

	dbg!(&cups[0..11], &cups[current], &cups[N]);

	for _ in 0..N * 10 {
		let taken = cups[current].popn(&cups, 3);
		let taken_n = {
			let taken2 = cups[taken].next.get();
			let taken3 = cups[taken2].next.get();
			[taken, taken2, taken3]
		};

		let mut current_label = current - 1;
		if current_label == 0 {
			current_label = N;
		}
		while taken_n.contains(&current_label) {
			current_label -= 1;
			if current_label == 0 {
				current_label = N;
			}
		}

		cups[current_label].pushn(&cups, taken, 3);

		current = cups[current].next.get();
	}

	let star1 = cups[1].next.get();
	let star2 = cups[star1].next.get();
	(star1 * star2).to_string()
}

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
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
    fn p1s() {
        let input = util::read_file("inputs/2020/day23-sample.txt");
        assert_eq!("67384529", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day23.txt");
        assert_eq!("38925764", part1(input));
    }

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
