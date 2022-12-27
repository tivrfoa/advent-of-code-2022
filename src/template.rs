use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};

pub fn solve(input: String) -> String {
    "".into()
}

pub fn solve_part2(input: String) -> String {
    todo!()
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/dayX-sample.txt");
        assert_eq!("", solve(input));
    }

    //#[test]
    //fn part1_input() {
    //    let input = util::read_file("inputs/dayX.txt");
    //    assert_eq!("", solve(input));
    //}

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day18-sample.txt");
    //    assert_eq!("", solve_part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day18.txt");
    //    assert_eq!("", solve_part2(input));
    //}
}

fn parse(input: String) -> String {
    todo!()
}
