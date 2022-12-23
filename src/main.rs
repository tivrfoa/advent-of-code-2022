mod day16_part2;
mod util;

fn main() {
    test_sample();
    test_input();
    test_input_umnikos();
}

fn test_sample() {
    // let input = util::read_file("inputs/day16-very-simple.txt");
    let input = util::read_file("inputs/day16-sample.txt");
    println!("{}", day16_part2::solve(input));
}

fn test_input() {
    let input = util::read_file("inputs/day16.txt");
    
    println!("{}", day16_part2::solve(input));
}

// Input from umnikos: friend from Rust Discord
fn test_input_umnikos() {
    let input = util::read_file("inputs/day16-2.txt"); // should be 2594

    println!("{}", day16_part2::solve(input));
}
