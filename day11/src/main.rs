use std::{collections::VecDeque, error::Error, fs::read_to_string, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;
    let monkeys = input
        .split("\n\n")
        .map(|m| m.parse().unwrap())
        .collect::<Vec<_>>();

    println!("part 1: {}", run(monkeys.clone(), 20, |x: u64| x / 3));

    let mul_primes = monkeys.iter().map(|m| m.test.condition).product::<u64>();
    println!("part 2: {}", run(monkeys, 10_000, |x: u64| x % mul_primes));

    Ok(())
}

fn run<F>(mut monkeys: Vec<Monkey>, rounds: usize, keep_calm: F) -> usize
where
    F: Fn(u64) -> u64,
{
    let mut activity = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            activity[m] += monkeys[m].items.len();

            while let Some(w) = monkeys[m].items.pop_front() {
                let w = keep_calm(monkeys[m].operation.compute(w));
                let next = if w % monkeys[m].test.condition == 0 {
                    monkeys[m].test.if_true
                } else {
                    monkeys[m].test.if_false
                };
                monkeys[next].items.push_back(w);
            }
        }
    }
    activity.sort_unstable();
    activity.into_iter().rev().take(2).product::<usize>()
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn new(items: VecDeque<u64>, operation: Operation, test: Test) -> Monkey {
        Monkey {
            items,
            operation,
            test,
        }
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines().skip(1);
        let items = it
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|n| n.parse::<_>().unwrap())
            .collect::<_>();

        let operation = it
            .next()
            .unwrap()
            .split_once("old ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let condition = it
            .next()
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse::<_>()
            .unwrap();

        let if_true = it
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();

        let if_false = it
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();

        Ok(Monkey::new(
            items,
            operation,
            Test::new(condition, if_true, if_false),
        ))
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Addition(u64),
    Multiplication(u64),
    Square,
}

impl Operation {
    fn compute(&self, a: u64) -> u64 {
        match self {
            Operation::Addition(b) => a + b,
            Operation::Multiplication(b) => a * b,
            Operation::Square => a * a,
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, n) = s.split_once(' ').unwrap();
        let n = n.parse::<_>();

        match op {
            "+" if n.is_ok() => Ok(Operation::Addition(n.unwrap())),
            "*" if n.is_ok() => Ok(Operation::Multiplication(n.unwrap())),
            "*" if n.is_err() => Ok(Operation::Square),
            op => Err(format!("Unknown operation: {:?}", op)),
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    condition: u64,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn new(condition: u64, if_true: usize, if_false: usize) -> Test {
        Test {
            condition,
            if_true,
            if_false,
        }
    }
}
