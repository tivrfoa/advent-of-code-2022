use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;
use std::rc::Rc;
use core::cell::RefCell;

use util::*;

// use crate::circular_list::{CircularList, Node};
use crate::y2020::day23_p2_circular_list::circular_list::{CircularList, Node};

mod circular_list {
    use std::rc::Rc;
    use std::cell::RefCell;

    #[derive(Debug)]
    pub struct Node<T: Copy> {
        pub n: T,
        pub next: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T: Copy> Node<T> {
		pub fn new_rc_node(n: T) -> Rc<RefCell<Node<T>>> {
            let node = Rc::new(RefCell::new(Node {
                n,
                next: None,
            }));
            node.borrow_mut().next = Some(node.clone());
            node
        }
    }

    pub trait CircularList<T: Copy> {
        fn next(&self) -> Option<Rc<RefCell<Node<T>>>>;
        fn append(&self, next: Rc<RefCell<Node<T>>>);
        fn pushn(&self, next: Rc<RefCell<Node<T>>>, n: usize);
		fn popn(&self, n: usize) -> Rc<RefCell<Node<T>>>;
        fn get_n_values(&self, n: usize) -> Vec<T>;
    }

    impl<T: Copy> CircularList<T> for Rc<RefCell<Node<T>>> {
        fn next(&self) -> Option<Rc<RefCell<Node<T>>>> {
            if self.borrow().next.is_none() {
                None
            } else {
                Some(self.borrow().next.as_ref().unwrap().clone())
            }
        }

		fn append(&self, next: Rc<RefCell<Node<T>>>) {
            next.borrow_mut().next = self.borrow_mut().next.take();
            self.borrow_mut().next = Some(next);
        }

        fn pushn(&self, next: Rc<RefCell<Node<T>>>, n: usize) {
            let mut tail: Rc<RefCell<Node<T>>> = next.clone();

            for _ in 0..n-1 {
                tail = tail.clone().borrow().next.as_ref().unwrap().clone();
            }
            tail.borrow_mut().next = self.borrow_mut().next.take();
            self.borrow_mut().next = Some(next);
        }

		fn popn(&self, n: usize) -> Rc<RefCell<Node<T>>> {
            let mut next = self.clone().borrow().next.as_ref().unwrap().clone();
			let taken = next.clone();

            for _ in 0..n {
                next = next.clone().borrow().next.as_ref().unwrap().clone();
            }
			self.borrow_mut().next = Some(next);

			taken
        }

		fn get_n_values(&self, n: usize) -> Vec<T> {
            let mut values = vec![];
            let mut curr = self.clone();

            for _ in 0..n {
                values.push(curr.borrow().n);
                curr = curr.clone().borrow().next.as_ref().unwrap().clone();
            }

			values
        }
    }
}

pub fn part2(input: String) -> String {
	let mut nodes: Vec<Option<Rc<RefCell<Node<usize>>>>> = vec![None; 1_000_001];
	let mut chars = input.lines().next().unwrap().chars();
	let mut n = chars.next().unwrap().to_decimal();
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
    // let n_values = curr.get_n_values(N + 5);
    // dbg!(&n_values[..11]);
    // dbg!(&n_values[N-11..]);
    

	for _ in 0..N * 10 {
		let taken = curr.popn(3);
		let taken_n = taken.get_n_values(3);

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

	let values = nodes[1].as_ref().unwrap().get_n_values(3);
	dbg!(&values);
	(values[1] * values[2]).to_string()
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
