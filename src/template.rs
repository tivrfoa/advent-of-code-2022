use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};

pub fn part1(input: String) -> String {
    "".into()
}

pub fn part2(input: String) -> String {
    todo!()
}

fn parse(input: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/dayX-sample.txt");
        assert_eq!("", part1(input));
    }

    //#[test]
    //fn part1_input() {
    //    let input = util::read_file("inputs/dayX.txt");
    //    assert_eq!("", part1(input));
    //}

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/dayX-sample.txt");
    //    assert_eq!("", part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/dayX.txt");
    //    assert_eq!("", part2(input));
    //}
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}
