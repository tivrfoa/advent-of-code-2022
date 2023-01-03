#![feature(int_roundings)]
#[allow(dead_code, unused_imports)]
mod aoc;
mod day22;
mod day23;
mod util;

use std::env;
use std::io::{self};

use crate::aoc::AOC;

use crate::day22::Day22;
use crate::day23::Day23;

fn main() {
    /*
        args[0] = run stdin S or file F
        args[1] = day to run, eg: 22
    */
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let days: [&dyn AOC; 23] = [
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day22 {},
        &Day23 {},
    ];
    let mut day_to_run = days[days.len() - 1];

    let mut input = None;

    // args[0] is executable name
    if args.len() > 1 {
        if args[1] == "S" {
            input = Some(get_input());
        }

        if args.len() == 3 {
            day_to_run = days[args[2].parse::<usize>().unwrap() - 1];
        }
    }

    // test_sample();

    // TODO read args for which day to run
    // if none is passed, then run last day
    println!("{}", day_to_run.part1(input, vec![]));
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
