mod day15_based_on_elizarov;
mod util;

fn main() {
    let input = util::read_file("inputs/day15.txt");
    println!("{}", day15_based_on_elizarov::solve_part2(input, 4000000));
}

// fn main() {}
