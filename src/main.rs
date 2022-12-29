#![feature(int_roundings)]
#[allow(dead_code, unused_imports)]
mod day19;
mod util;

fn main() {
    //test_sample();
    //test_input();
    test_part2();
}

fn test_sample() {
    let input = util::read_file("inputs/day19-sample.txt");
    println!("{}", day19::part1(input));
}

fn test_input() {
    let input = util::read_file("inputs/day19.txt");
    println!("{}", day19::part1(input));
}

fn test_part2() {
    let input = util::read_file("inputs/day19.txt");
    println!("{}", day19::part2(input));
}
