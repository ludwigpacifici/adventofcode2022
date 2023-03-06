use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let monkeys = parse(&input);

    let part1 = compute(&monkeys, "root");
    println!("part 1: {part1}");

    let part2 = yell(monkeys, "fcgj", "bsbd", "humn");
    println!("part 2: {part2}");
}

fn yell(monkeys: HashMap<&str, Job>, target_key: &str, other_key: &str, yell_key: &str) -> i64 {
    let target = compute(&monkeys, target_key);
    let mut h = 1_000_000_000_000_000;
    let mut l = 0;
    let mut m = (h + l) / 2;
    let mut current = compute_with(&monkeys, other_key, yell_key, m);

    while current != target {
        if current < target {
            h = m - 1;
        } else {
            l = m + 1;
        }
        m = (h + l) / 2;
        current = compute_with(&monkeys, other_key, yell_key, m);
    }

    while current == target {
        m -= 1;
        current = compute_with(&monkeys, other_key, yell_key, m);
    }

    m + 1
}
fn compute(jobs: &HashMap<&str, Job>, root: &str) -> i64 {
    match jobs[root] {
        Job::Value(v) => v,
        Job::Add(l, r) => compute(jobs, l) + compute(jobs, r),
        Job::Minus(l, r) => compute(jobs, l) - compute(jobs, r),
        Job::Multiply(l, r) => compute(jobs, l) * compute(jobs, r),
        Job::Divide(l, r) => compute(jobs, l) / compute(jobs, r),
    }
}

fn compute_with(jobs: &HashMap<&str, Job>, root: &str, with: &str, value: i64) -> i64 {
    if root == with {
        return value;
    }

    match jobs[root] {
        Job::Value(v) => v,
        Job::Add(l, r) => compute_with(jobs, l, with, value) + compute_with(jobs, r, with, value),
        Job::Minus(l, r) => compute_with(jobs, l, with, value) - compute_with(jobs, r, with, value),
        Job::Multiply(l, r) => {
            compute_with(jobs, l, with, value) * compute_with(jobs, r, with, value)
        }
        Job::Divide(l, r) => {
            compute_with(jobs, l, with, value) / compute_with(jobs, r, with, value)
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Job> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            let monkey = it.next().unwrap().trim_end_matches(':');
            let x = it.next().unwrap();
            let job = if let Ok(n) = x.parse::<i64>() {
                Job::Value(n)
            } else {
                let lhs = x;
                let op = it.next().unwrap();
                let rhs = it.next().unwrap();
                match op {
                    "+" => Job::Add(lhs, rhs),
                    "-" => Job::Minus(lhs, rhs),
                    "*" => Job::Multiply(lhs, rhs),
                    "/" => Job::Divide(lhs, rhs),
                    _ => panic!("Unknown operator {:?}", op),
                }
            };
            (monkey, job)
        })
        .collect::<HashMap<_, _>>()
}

#[derive(Debug)]
enum Job<'a> {
    Value(i64),
    Add(&'a str, &'a str),
    Minus(&'a str, &'a str),
    Multiply(&'a str, &'a str),
    Divide(&'a str, &'a str),
}
