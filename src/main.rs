#![feature(int_roundings)]
#[allow(dead_code, unused_imports)]
mod day20;
mod util;

fn main() {
    // test_sample();
    test_input();
    //test_part2();
}

fn test_sample() {
    let input = util::read_file("inputs/day20-sample.txt");
    let ans = day20::part1(input);
    assert_eq!("3", ans);
    println!("{}", ans);
}

fn test_input() {
    let input = util::read_file("inputs/day20.txt");
    // let input = util::read_file("inputs/day20-simple1.txt");
    println!("{}", day20::part1(input));
}

fn test_part2() {
    let input = util::read_file("inputs/day20.txt");
    println!("{}", day20::part2(input));
}
