#[allow(dead_code, unused_imports)]
mod day18;
mod util;

use day18::solve;
use day18::solve_part2;

fn main() {
    test_sample();
    //test_input();
    //test_part2();
}

fn test_sample() {
    let input = util::read_file("inputs/day18-sample.txt");
    println!("{}", solve(input));
    // println!("{}", solve(input, 2022));
}

fn test_input() {
    let input = util::read_file("inputs/day18.txt");

    println!("{}", solve(input));
    // println!("{}", solve(input, 2022));
}

fn test_part2() {
    let input = util::read_file("inputs/day18.txt");

    println!("{}", solve_part2(input));
    // println!("{}", solve(input, 1000000000000));
}
