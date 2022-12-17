mod day15;
mod util;

fn main() {
    let input = util::read_file("inputs/day15.txt");
    println!("{}", day15::solve_part2(input, 4000000));
}
