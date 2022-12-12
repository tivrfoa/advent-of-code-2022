use crate::util;

#[derive(Debug)]
enum Op {
    M,
    A,
}

#[derive(Debug)]
enum OpV {
    Old,
    V(u64),
}

#[derive(Debug)]
struct Monkey {
    op: Op,
    opv: OpV,
    div: u64,
    true_id: usize,
    false_id: usize,
    items_worry_level: Vec<u64>,
    items_inspected: u64,
}

impl Monkey {
    fn new() -> Self {
        Self {
            op: Op::A,
            opv: OpV::Old,
            div: 0,
            true_id: 0,
            false_id: 0,
            items_worry_level: vec![],
            items_inspected: 0,
        }
    }

    fn set_operation(&mut self, op: &str, opv: &str) {
        self.op = match op {
            "+" => Op::A,
            "*" => Op::M,
            _ => panic!("Invalid op: {op}"),
        };
        self.opv = match opv {
            "old" => OpV::Old,
            _ => OpV::V(opv.parse().unwrap()),
        };
    }

    fn calc_next_level(&self, item: u64, relief: u64) -> u64 {
        let rv = match self.opv {
            OpV::Old => item,
            OpV::V(v) => v,
        };
        let tmp = match self.op {
            Op::A => item + rv,
            Op::M => item * rv,
        };
        tmp / relief
    }

    fn throw_item(&self, item: u64) -> usize {
        if item % self.div == 0 {
            self.true_id
        } else {
            self.false_id
        }
    }
}

pub fn solve(input: String, relief: u64, rounds: usize) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkey_id = 0;
    let mut modulo = 1;

    for line in input.lines() {
        if line.starts_with("Monkey") {
            monkeys.push(Monkey::new());
        } else if line.starts_with("  S") {
            // eg: Starting items: 79, 98
            let itens: Vec<u64> = line
                .split_once(':')
                .unwrap()
                .1
                .split(',')
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect();
            monkeys[monkey_id].items_worry_level = itens;
        } else if line.starts_with("  O") {
            // eg: Operation: new = old * 19
            let (op, opv) = line.split_once("old ").unwrap().1.split_once(' ').unwrap();
            monkeys[monkey_id].set_operation(op, opv);
        } else if line.starts_with("  T") {
            // eg: Test: divisible by 13
            let div: u64 = line.split_once("by ").unwrap().1.parse().unwrap();
            modulo *= div;
            monkeys[monkey_id].div = div;
        } else if line.starts_with("    If t") {
            // eg: If true: throw to monkey 2
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
            let id: usize = tokens[tokens.len() - 1].parse().unwrap();
            monkeys[monkey_id].true_id = id;
        } else if line.starts_with("    If f") {
            // eg: If false: throw to monkey 3
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
            let id: usize = tokens[tokens.len() - 1].parse().unwrap();
            monkeys[monkey_id].false_id = id;
        } else {
            monkey_id += 1;
        }
    }

    // println!("{:#?}", monkeys);

    let len = monkeys.len();
    for _ in 0..rounds {
        for i in 0..len {
            while let Some(item) = monkeys[i].items_worry_level.pop() {
                monkeys[i].items_inspected += 1;
                let next_level = monkeys[i].calc_next_level(item, relief) % modulo;
                let to_monkey = monkeys[i].throw_item(next_level);
                monkeys[to_monkey].items_worry_level.push(next_level);
            }
        }
    }

    let mut items_inspected: Vec<u64> = monkeys.iter().map(|m| m.items_inspected).collect();
    items_inspected.sort();

    items_inspected[len - 2] * items_inspected[len - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/sample-day11.txt");
        assert_eq!(10605, solve(input, 3, 20));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/input-day11.txt");
        assert_eq!(119715, solve(input, 3, 20));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/sample-day11.txt");
        assert_eq!(2713310158, solve(input, 1, 10_000));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/input-day11.txt");
        assert_eq!(18085004878, solve(input, 1, 10_000));
    }
}
