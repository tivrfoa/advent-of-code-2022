#![feature(int_roundings)]
#[allow(dead_code, unused_imports)]
mod day21_p2;
mod util;

fn main() {
    test_sample();
    test_input();
    //test_part2();
}

fn test_sample() {
    let input = util::read_file("inputs/day21-sample.txt");
    println!("{}", day21_p2::part2(input));
}

fn test_input() {
    let input = util::read_file("inputs/day21.txt");
    println!("{}", day21_p2::part2(input));
}
