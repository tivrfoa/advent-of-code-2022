use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::iter::zip;

use Op::*;

/*

All jobs (monkeys) are unique!!!

*/

fn solve(jobs: &mut Vec<Job>, pos: usize) -> i64 {
    if let Some(r) = jobs[pos].result {
        r
    } else {
        let a = solve(jobs, jobs[pos].left);
        let b = solve(jobs, jobs[pos].right);
        let result = jobs[pos].op.calc(a, b);
        jobs[pos].result = Some(result);
        result
    }
}

/*

Need to know which path from humn leads to root, and check if it's left
or right from root.

*/
pub fn part2(input: String) -> String {
    let (mut jobs, used_by, you, root) = parse(input);
    //dbg!(&jobs);

    // build path from you to root
    let mut path = vec![];
    let mut pos = you;
    while pos != root {
        path.push(pos);
        pos = used_by[pos];
    }

    dbg!(&path);

    // now we need to find the difference between root.left and root.right
    let root_result = solve(&mut jobs, root);
    // assert_eq!(152, root_result); // for sample
    // assert_eq!(93813115694560, root_result); // for input
    let left = jobs[root].left;
    let right = jobs[root].right;
    println!("left = {:?}", jobs[left].result);
    println!("right = {:?}", jobs[right].result);

    // last index in path will be either left or right of root
    // value is what we aim for at each operation
    let (mut pos, mut value) = if path[path.len() - 1] == left {
        (left, jobs[right].result.take().unwrap())
    } else {
        (right, jobs[left].result.take().unwrap())
    };
    // dbg!(pos, value);

    // walk path in reverse order
    for i in (1..path.len()).rev() {
        let idx = path[i];
        dbg!(&path[i]);

        // who is in path
        value = if path[i - 1] == jobs[idx].left {
            let ri = jobs[idx].right;
            match jobs[idx].op {
                Add => value - jobs[ri].result.take().unwrap(),
                Sub => value + jobs[ri].result.take().unwrap(),
                Mul => value / jobs[ri].result.take().unwrap(),
                Div => value * jobs[ri].result.take().unwrap(),
                _ => panic!("..."),
            }
        } else {
            let li = jobs[idx].left;
            match jobs[idx].op {
                Add => value - jobs[li].result.take().unwrap(),
                Sub => jobs[li].result.take().unwrap() - value,
                Mul => value / jobs[li].result.take().unwrap(),
                Div => jobs[li].result.take().unwrap() / value,
                _ => panic!("..."),
            }
        };
    }

    value.to_string()
}

#[derive(Clone, Debug, Default)]
struct Job {
    result: Option<i64>,
    op: Op,
    left: usize,
    right: usize,
}

#[derive(Clone, Debug, Default)]
enum Op {
    #[default]
    Nop,
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
            _ => panic!("{:?}", self),
        }
    }
}

const MAX: usize = 3000; // any number greater than number of lines

fn parse(input: String) -> (Vec<Job>, Vec<usize>, usize, usize) {
    let mut jobs_id: HashMap<&str, usize> = HashMap::new();
    let num_lines = input.lines().count();
    let mut jobs: Vec<Job> = vec![Job::default(); num_lines];
    let mut used_by: Vec<usize> = vec![MAX; num_lines];
    let mut idx = 0;
    let mut you = 0;
    let mut root = 0;
    for line in input.lines() {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();

        // tokens len will be 2 or 4
        let (job_name, _) = tokens[0].split_once(':').unwrap();
        let job_idx = match jobs_id.get(job_name) {
            Some(i) => *i,
            None => {
                jobs_id.insert(job_name, idx);
                idx += 1;
                idx - 1
            }
        };
        if job_name == "root" {
            root = job_idx;
        } else if job_name == "humn" {
            you = job_idx;
        }

        if tokens.len() == 2 {
            jobs[job_idx] = Job {
                result: Some(tokens[1].parse::<i64>().unwrap()),
                ..Default::default()
            };
        } else {
            let left = match jobs_id.get(tokens[1]) {
                Some(i) => *i,
                None => {
                    jobs_id.insert(tokens[1], idx);
                    idx += 1;
                    idx - 1
                }
            };
            used_by[left] = job_idx;
            let right = match jobs_id.get(tokens[3]) {
                Some(i) => *i,
                None => {
                    jobs_id.insert(tokens[3], idx);
                    idx += 1;
                    idx - 1
                }
            };
            used_by[right] = job_idx;
            jobs[job_idx] = Job {
                result: None,
                op: Op::from_str(tokens[2]),
                left,
                right,
            };
        }
    }

    (jobs, used_by, you, root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/day21-sample.txt");
        assert_eq!("301", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/day21.txt");
        assert_eq!("3910938071092", part2(input));
    }
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}
