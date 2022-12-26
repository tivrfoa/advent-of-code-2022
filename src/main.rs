// mod day17_elizarov;
mod day17;
mod util;

use day17::solve;
use day17::solve_part2;

fn main() {
    test_sample();
    test_input();
    test_part2();
}

fn test_sample() {
    let input = util::read_file("inputs/day17-sample.txt");
    println!("{}", solve(input));
    // println!("{}", solve(input, 2022));
}

fn test_input() {
    let input = util::read_file("inputs/day17.txt");

    println!("{}", solve(input));
    // println!("{}", solve(input, 2022));
}

fn test_part2() {
    let input = util::read_file("inputs/day17.txt");

    println!("{}", solve_part2(input));
    // println!("{}", solve(input, 1000000000000));
}
