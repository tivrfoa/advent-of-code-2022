#![feature(int_roundings)]
#[allow(dead_code, unused_imports)]
mod day22;
mod util;

use std::io::{self, BufRead};

fn main() {
    test_simple();
}

fn run_stdin() {
    let mut input = String::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        input.push_str(&line.unwrap());
    }
    println!("{}", day22::part1(input));
}

fn test_sample() {
    let input = util::read_file("inputs/day22-sample.txt");
    println!("{}", day22::part1(input));
}

fn test_simple() {
    // working: 87222
    let input = util::read_file("inputs/day22-simple1.txt");

    // wrong ... should be 35474
    let input = util::read_file("inputs/day22-simple2.txt");

    // good: 72334
    let input = util::read_file("inputs/day22-simple3.txt");

    // good: 9136
    let _input = util::read_file("inputs/day22-simple4.txt");

    let input = util::read_file("inputs/day22-simple5.txt");

    println!("{}", day22::part1(input));
}
