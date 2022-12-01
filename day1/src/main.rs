use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("part 1:{:?}", part1(&input));
    println!("part 2:{:?}", part2(&input));

    Ok(())
}

fn part2(input: &str) -> u64 {
    let mut answer = input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse::<u64>().unwrap_or_default())
                .sum::<u64>()
        })
        .collect::<Vec<_>>();
    answer.sort();
    answer.into_iter().rev().take(3).sum()
}

fn part1(input: &str) -> Option<u64> {
    input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse::<u64>().unwrap_or_default())
                .sum()
        })
        .max()
}
