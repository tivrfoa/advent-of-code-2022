use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

use util::*;

fn parse_players(input: &str) -> Vec<VecDeque<u16>> {
	let mut players: Vec<VecDeque<u16>> = vec![VecDeque::new(); 2];
	let mut player_idx = 0;
	for player in input.split("\n\n") {
		let mut lines = player.lines();
		lines.next();
		for line in lines {
			players[player_idx].push_back(line.parse().unwrap());
		}
		player_idx += 1;
	}

	players
}

pub fn part1(input: String) -> String {
	let mut players: Vec<VecDeque<u16>> = parse_players(&input);

	while !players[0].is_empty() && !players[1].is_empty() {
		if players[0][0] > players[1][0] {
			let x = players[0].pop_front().unwrap();
			let y = players[1].pop_front().unwrap();
			players[0].push_back(x);
			players[0].push_back(y);
		} else {
			let x = players[1].pop_front().unwrap();
			let y = players[0].pop_front().unwrap();
			players[1].push_back(x);
			players[1].push_back(y);
		}
	}

	let won = if players[0].is_empty() { 1 } else { 0 };
	let mut score = 0;
	for (i, v) in players[won].iter().rev().enumerate() {
		score += v * (i as u16 + 1);
	}

	score.to_string()
}

fn play_p2(visited: &mut HashSet<(u16, VecDeque<u16>)>,
		mut player1: VecDeque<u16>,
		mut player2: VecDeque<u16>,
		game: &mut u16) -> (usize, Option<VecDeque<u16>>) {
    let start_game = *game;

	while !player1.is_empty() && !player2.is_empty() {
        if !visited.insert((start_game, player1.clone())) {
            return (1, None);
        }
		let p1_first = player1.pop_front().unwrap();
		let p2_first = player2.pop_front().unwrap();

		let won = if p1_first <= player1.len() as u16 && p2_first <= player2.len() as u16 {
            *game += 1;
			play_p2(visited,
                player1.range(..p1_first as usize).copied().collect(),
                player2.range(..p2_first as usize).copied().collect(),
                game).0
		} else {
			if p1_first > p2_first { 1 } else {	2 }
		};
        if won == 1 {
            player1.push_back(p1_first);
            player1.push_back(p2_first);
        } else {
            player2.push_back(p2_first);
            player2.push_back(p1_first);
        }
	}

	if !player1.is_empty() { (1, Some(player1)) } else { (2, Some(player2)) }
}

pub fn part2(input: String) -> String {
	let mut players: Vec<VecDeque<u16>> = parse_players(&input);
	let player2 = players.pop().unwrap();
	let player1 = players.pop().unwrap();
	let mut visited: HashSet<(u16, VecDeque<u16>)> = HashSet::new();
    let mut game = 1;

	let (_, p) = play_p2(&mut visited, player1, player2, &mut game);
	let mut score = 0;
	for (i, v) in p.unwrap().iter().rev().enumerate() {
		score += v * (i as u16 + 1);
	}

	score.to_string()
}

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u16,
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
        let input = util::read_file("inputs/2020/day22-sample.txt");
        assert_eq!("306", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2020/day22.txt");
        assert_eq!("32472", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2020/day22-sample.txt");
        assert_eq!("291", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2020/day22.txt");
        assert_eq!("36463", part2(input));
    }
}
