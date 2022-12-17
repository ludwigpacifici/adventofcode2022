use rayon::prelude::*;

use std::{collections::HashSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let input = parse(&input);

    let part1 = part1(&input, 2_000_000);

    println!("part 1: {}", part1);
    println!("part 2: {}", part2(input, 4_000_000));

    Ok(())
}

fn part1(input: &[((i64, i64), (i64, i64))], y_scan: i64) -> usize {
    let part1 = input.iter().fold(HashSet::new(), |mut acc, (s, b)| {
        let r = n1(s, b);

        for x in (s.0 - r)..=(s.0 + r) {
            if n1(s, &(x, y_scan)) <= r {
                acc.insert(x);
            }
        }
        acc
    });

    input
        .iter()
        .fold(part1, |mut acc, (s, b)| {
            if s.1 == y_scan {
                acc.remove(&s.0);
            }

            if b.1 == y_scan {
                acc.remove(&b.0);
            }

            acc
        })
        .len()
}

fn parse(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    let input = input
        .lines()
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            let sx = it
                .nth(2)
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .trim_end_matches(',')
                .parse::<i64>()
                .unwrap();

            let sy = it
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .trim_end_matches(':')
                .parse::<i64>()
                .unwrap();

            let bx = it
                .nth(4)
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .trim_end_matches(',')
                .parse::<i64>()
                .unwrap();

            let by = it
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .parse::<i64>()
                .unwrap();

            ((sx, sy), (bx, by))
        })
        .collect::<Vec<_>>();
    input
}

fn part2(input: Vec<((i64, i64), (i64, i64))>, bound: i64) -> i64 {
    let part2 = input
        .iter()
        .fold(HashSet::new(), |mut acc, (s, b)| {
            let r = n1(s, b);

            let mut y = s.1;
            for x in (s.0 - r) - 1..=s.0 {
                acc.insert((x, y));
                y -= 1;
            }

            let mut y = s.1;
            for x in (s.0 - r) - 1..=s.0 {
                acc.insert((x, y));
                y += 1;
            }

            let mut y = s.1 + r + 1;
            for x in s.0..=s.0 + r + 1 {
                acc.insert((x, y));
                y -= 1;
            }

            let mut y = s.1 - r - 1;
            for x in s.0..=s.0 + r + 1 {
                acc.insert((x, y));
                y += 1;
            }

            acc
        })
        .into_par_iter()
        .filter(|(x, y)| 0 <= *x && *x <= bound && 0 <= *y && *y <= bound)
        .filter(|xy| input.iter().all(|(s, b)| n1(s, b) < n1(s, xy)))
        .collect::<HashSet<_>>();

    hash(part2.into_iter().next().unwrap())
}

fn hash((x, y): (i64, i64)) -> i64 {
    x * 4_000_000 + y
}

fn n1(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
