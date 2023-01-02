#![feature(int_roundings)]
#[allow(dead_code, unused_imports)]
mod day22;
mod util;

use std::io::{self, BufRead};

fn main() {
    // test_sample();
    run_stdin();
}

fn run_stdin() {
    let mut input = String::new();
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(len) => if len == 0 {
                break;
            } else {
                input.push_str(&line);
            }
            Err(error) => {
                eprintln!("error: {}", error);
                break;
            }
        }
    }
    println!("{}", day22::part1(input));
}

fn test_sample() {
    let input = util::read_file("inputs/day22.txt");
    println!("{}", day22::part1(input));
}
