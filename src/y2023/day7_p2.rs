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
    J,
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
        let mut max = 1;
        for c in cards {
            if let Some(q) = map.get_mut(c) {
                *q += 1;
                if *q > max {
                    max = *q;
                }
            } else {
                map.insert(c, 1);
            }
        }

        let qt_j = if let Some(qt) = map.get(&J) {
            *qt
        } else {
            0
        };
        dbg!(qt_j);
        match max {
            5 => FiveOfKind,
            4 => {
                if qt_j == 1 {
                    FiveOfKind
                } else {
                    FourOfKind
                }
            }
            3 => {
                if qt_j == 2 {
                    return FiveOfKind;
                }
                if qt_j == 1 {
                    return FourOfKind;
                }

                // check if there's a pair
                // TODO use find ?
                if map.iter().filter(|(_, qt)| **qt == 2).count() == 1 {
                    FullHouse
                } else {
                    ThreeOfKind
                }
            }
            2 => {
                // count pairs
                let mut qt_pairs = 0;
                for (_, qt) in map {
                    if qt == 2 {
                        qt_pairs += 1;
                    }
                }

                if qt_j == 2 {
                    if qt_pairs == 1 {
                        OnePair
                    } else {
                        FourOfKind
                    }
                } else if qt_j == 1 {
                    if qt_pairs == 1 {
                        ThreeOfKind
                    } else {
                        FullHouse
                    }
                } else if qt_pairs == 1 {
                    OnePair
                } else {
                    TwoPairs
                }
            }
            _ => {
                if qt_j == 1 {
                    OnePair
                } else {
                    HighCard
                }
            },
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
        self.hand_type.cmp(&other.hand_type)
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
    let mut hands: Vec<Hand> = vec![];
    for line in input.lines() {
        hands.push(Hand::parse(line));
    }
    hands.sort();

    let mut ans = 0;
    for (i, h) in hands.into_iter().enumerate() {
        ans += (i as u64 + 1) * h.bid;
    }

    ans.to_string()
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
        assert_eq!("", part2(input));
    }
}
