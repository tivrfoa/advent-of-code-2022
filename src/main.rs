#![feature(int_roundings)]
#[allow(dead_code, unused_imports)]
mod day22p2;
mod util;

use std::io::{self, BufRead};

fn main() {
    test_sample();
}

fn run_stdin() {
    let mut input = String::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        input.push_str(&line.unwrap());
    }
    println!("{}", day22p2::part2(input));
}

fn test_sample() {
    let input = util::read_file("inputs/day22.txt");
    println!("{}", day22p2::part2(input));
}
