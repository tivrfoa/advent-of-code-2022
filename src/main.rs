#![feature(int_roundings)]
#[allow(dead_code, unused_imports)]
mod aoc;
mod day22;
mod util;

use std::io::{self};

use crate::aoc::AOC;

use crate::day22::Day22;

fn main() {
    let days: [&dyn AOC; 1] = [&Day22 {}];
    // test_sample();
    let input = get_input();

    // TODO read args for which day to run
    // if none is passed, then run last day
    println!("{}", days[days.len() - 1].part1(input, vec![]));
}

fn get_input() -> String {
    let mut input = String::new();
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(len) => {
                if len == 0 {
                    break;
                } else {
                    input.push_str(&line);
                }
            }
            Err(error) => {
                eprintln!("error: {}", error);
                break;
            }
        }
    }

    input
}

fn test_sample() {
    let input = util::read_file("inputs/day22.txt");
    println!("{}", day22::part1(input));
}
