use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd)]
enum Card {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

impl Card {
    fn get(c: char) -> Self {
        match c {
            'A' => A,
            'K' => K,
            'Q' => Q,
            'J' => J,
            'T' => T,
            '9' => N9,
            '8' => N8,
            '7' => N7,
            '6' => N6,
            '5' => N5,
            '4' => N4,
            '3' => N3,
            '2' => N2,
            _ => panic!("{c}"),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self > other {
            Ordering::Greater
        } else if self < other {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

use Card::*;

#[derive(Debug, Eq, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

use HandType::*;

impl HandType {
    fn get(cards: &[Card; 5]) -> Self {
        let mut map: HashMap<&Card, u8> = HashMap::new();
        let mut qt_j = 0;
        for c in cards {
            if *c == J { qt_j += 1; }
            else {
                let qt = map.entry(c).or_insert(0);
                *qt += 1;
            }
        }
        if qt_j == 5 { return FiveOfKind; }
        let mut qts: Vec<u8> = map.values().cloned().collect();
        qts.sort_by(|a, b| b.cmp(a));
        qts[0] += qt_j;
        let qt_pairs = qts.iter().filter(|qt| **qt == 2).count();
        match qts[0] {
            5 => FiveOfKind,
            4 => FourOfKind,
            3 => if qt_pairs == 1 {
                    FullHouse
                } else {
                    ThreeOfKind
                }
            2 => if qt_pairs == 1 {
                    OnePair
                } else {
                    TwoPairs
                }
            _ => HighCard,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self > other {
            Ordering::Greater
        } else if self < other {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Debug, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    hand_type: HandType,
}

impl Hand {
    fn parse(line: &str) -> Self {
        let (cards_tmp, bid) = line.split_once(' ').unwrap();
        let mut cards = [A, A, A, A, A];
        for (i, c) in cards_tmp.chars().enumerate() {
            cards[i] = Card::get(c);
        }
        let hand_type = HandType::get(&cards);

        Self {
            cards,
            bid: bid.parse().unwrap(),
            hand_type,
        }
    }

    fn compare_cards(&self, other_cards: &[Card; 5]) -> Ordering {
        for i in 0..self.cards.len() {
            if self.cards[i] == other_cards[i] {
                continue;
            }
            return self.cards[i].cmp(&other_cards[i]);
        }
        panic!("Same cards?! {:?} - {:?}", self.cards, other_cards);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then(self.compare_cards(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

pub fn part2(input: &str) -> String {
    assert!(J < N2);
    let mut hands: Vec<Hand> = vec![];
    for line in input.lines() {
        hands.push(Hand::parse(line));
    }
    hands.sort();

    let mut ans = 0;
    for (i, h) in hands.into_iter().enumerate() {
        // println!("{:?}", h);
        print(&h.cards);
        ans += (i as u64 + 1) * h.bid;
    }

    ans.to_string()
}

fn print(cards: &[Card; 5]) {
    print!("[");
    for c in cards {
        let c = match c {
            J => 74,
            N2 => 50,
            N3 => 51,
            N4 => 52,
            N5 => 53,
            N6 => 54,
            N7 => 55,
            N8 => 56,
            N9 => 57,
            T => 84,
            Q => 81,
            K => 75,
            A => 65,
        };
        print!("{} ", c);
    }

    println!("]");
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
        let input = include_str!("../../inputs/2023/day7-sample.txt");
        assert_eq!("5905", part2(input));
    }

    #[test]
    fn p2() {
        let input = include_str!("../../inputs/2023/day7.txt");
        assert_eq!("248750699", part2(input));
    }

    #[test]
    fn test200() {
        let input = include_str!("../../inputs/2023/day7-test200.txt");
        assert_eq!("9446128", part2(input));
    }

    #[test]
    fn test400() {
        let input = include_str!("../../inputs/2023/day7-test400.txt");
        assert_eq!("38071640", part2(input));
    }

    #[test]
    fn test500() {
        let input = include_str!("../../inputs/2023/day7-test500.txt");
        assert_eq!("61215331", part2(input));
    }

    #[test]
    fn test400to500() {
        let input = include_str!("../../inputs/2023/day7-test400to500.txt");
        assert_eq!("2806430", part2(input));
    }
}
