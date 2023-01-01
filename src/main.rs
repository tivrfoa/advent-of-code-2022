#![feature(int_roundings)]
#[allow(dead_code, unused_imports)]
mod day22;
mod util;

fn main() {
    test_sample();
    //test_input();
    //test_part2();
}

fn test_sample() {
    let input = util::read_file("inputs/day22-sample.txt");
    println!("{}", day22::part1(input));
}

fn test_input() {
    let input = util::read_file("inputs/day22.txt");
    println!("{}", day22::part1(input));
}
