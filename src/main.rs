mod day16_part2;
mod util;

fn main() {
    test_sample();
    test_input();
}

fn test_sample() {
    let input = util::read_file("inputs/day16-sample.txt");
    println!("{}", day16_part2::solve(input));
}

fn test_input() {
    let input = util::read_file("inputs/day16.txt");
    println!("{}", day16_part2::solve(input));
}
