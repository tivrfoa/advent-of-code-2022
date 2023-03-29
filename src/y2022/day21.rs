use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

use Op::*;

/*

All jobs (monkeys) are unique!!!

*/

fn solve(jobs: &mut HashMap<String, Job>, key: String) -> i64 {
    let job = jobs.get(&key).unwrap().clone();

    if let Some(r) = job.result {
        r
    } else {
        let calc = job.operation.unwrap();
        let a = solve(jobs, calc.left);
        let b = solve(jobs, calc.right);
        let result = calc.op.calc(a, b);
        jobs.get_mut(&key).unwrap().result = Some(result);
        result
    }
}

pub fn part1(input: String) -> String {
    let mut jobs = parse(input);
    // dbg!(&jobs);

    let ans = solve(&mut jobs, "root".to_string()).to_string();

    let left = jobs
        .get("root")
        .unwrap()
        .operation
        .as_ref()
        .unwrap()
        .left
        .clone();
    let left_value = jobs.get(&left).unwrap().result.as_ref().unwrap();
    let right = jobs
        .get("root")
        .unwrap()
        .operation
        .as_ref()
        .unwrap()
        .right
        .clone();
    let right_value = jobs.get(&right).unwrap().result.as_ref().unwrap();
    println!("left {left} = {left_value}");
    println!("right {right} = {right_value}");

    ans
}

pub fn part2(input: String) -> String {
    todo!()
}

#[derive(Clone, Debug)]
struct Job {
    result: Option<i64>,
    operation: Option<Calc>,
}

#[derive(Clone, Debug)]
struct Calc {
    left: String,
    op: Op,
    right: String,
}

#[derive(Clone, Debug)]
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

    fn calc(&self, a: i64, b: i64) -> i64 {
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
                    result: Some(tokens[1].parse::<i64>().unwrap()),
                    operation: None,
                },
            );
        } else {
            let (job_name, _) = tokens[0].split_once(':').unwrap();
            jobs.insert(
                job_name.into(),
                Job {
                    result: None,
                    operation: Some(Calc {
                        left: tokens[1].to_string(),
                        op: Op::from_str(tokens[2]),
                        right: tokens[3].to_string(),
                    }),
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
        let input = util::read_file("inputs/2022/day21-sample.txt");
        assert_eq!("152", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2022/day21.txt");
        assert_eq!("93813115694560", part1(input));
    }

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/2022/day21-sample.txt");
    //    assert_eq!("", part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/2022/day21.txt");
    //    assert_eq!("", part2(input));
    //}
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}
