use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

use Op::*;

/*

All jobs (monkeys) are unique!!!

*/

fn solve(jobs: &mut HashMap<String, Job>, key: String) -> i32 {
    let job = jobs.get(&key).unwrap();

    if let Some(r) = job.result {
        r
    } else {
        let (a, op, b) = job.operation.unwrap();
        let a = solve(jobs, a);
        let b = solve(jobs, b);
        let result = op.calc(a, b);
        job.result = Some(result);
        result
    }
}

pub fn part1(input: String) -> String {
    let mut jobs = parse(input);
    // dbg!(&jobs);

    solve(&mut jobs, "root".to_string()).to_string()
}

pub fn part2(input: String) -> String {
    todo!()
}

#[derive(Debug)]
struct Job {
    result: Option<i32>,
    operation: Option<(String, Op, String)>,
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Op {
    fn from_str(s: &str) -> Self {
        match s {
            "+" => Add,
            "-" => Sub,
            "/" => Div,
            "*" => Mul,
            _ => panic!("{s}"),
        }
    }

    fn calc(&self, a: i32, b: i32) -> i32 {
        match self {
            Add => a + b,
            Sub => a - b,
            Div => a / b,
            Mul => a * b,
        }
    }
}

fn parse(input: String) -> HashMap<String, Job> {
    let mut jobs: HashMap<String, Job> = HashMap::new();
    for line in input.lines() {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        // tokens len will be 2 or 4

        if tokens.len() == 2 {
            let (job_name, _) = tokens[0].split_once(':').unwrap();
            jobs.insert(
                job_name.into(),
                Job {
                    result: Some(tokens[1].parse::<i32>().unwrap()),
                    operation: None,
                },
            );
        } else {
            let (job_name, _) = tokens[0].split_once(':').unwrap();
            jobs.insert(
                job_name.into(),
                Job {
                    result: None,
                    operation: Some((
                        tokens[1].to_string(),
                        Op::from_str(tokens[2]),
                        tokens[3].to_string(),
                    )),
                },
            );
        }
    }

    jobs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/day21-sample.txt");
        assert_eq!("152", part1(input));
    }

    //#[test]
    //fn part1_input() {
    //    let input = util::read_file("inputs/day21.txt");
    //    assert_eq!("", part1(input));
    //}

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day21-sample.txt");
    //    assert_eq!("", part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day21.txt");
    //    assert_eq!("", part2(input));
    //}
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}
