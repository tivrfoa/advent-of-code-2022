mod day17;
mod util;

fn main() {
    test_sample();
    // test_input();
}

fn test_sample() {
    let input = util::read_file("inputs/day17-sample.txt");
    println!("{}", day17::solve(input));
}

fn test_input() {
    let input = util::read_file("inputs/day17.txt");

    println!("{}", day17::solve(input));
}
