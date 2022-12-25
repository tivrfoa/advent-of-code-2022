mod day17_elizarov;
mod util;

fn main() {
    test_sample();
    test_input();
    test_part2();
}

fn test_sample() {
    let input = util::read_file("inputs/day17-sample.txt");
    println!("{}", day17_elizarov::solve(input, 2022));
}

fn test_input() {
    let input = util::read_file("inputs/day17.txt");

    println!("{}", day17_elizarov::solve(input, 2022));
}

fn test_part2() {
    let input = util::read_file("inputs/day17.txt");

    println!("{}", day17_elizarov::solve(input, 1000000000000));
}
