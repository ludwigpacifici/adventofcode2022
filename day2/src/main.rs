use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("part 1:{:?}", run(&input, part1));
    println!("part 2:{:?}", run(&input, part2));

    Ok(())
}

fn run<S>(input: &str, score: S) -> u64
where
    S: Fn((&str, &str)) -> u64,
{
    let part1 = input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let (Some(other_move) , Some(strategy)) = (it.next(), it.next()) else { panic!("cannot get moves for line {:?}", l) };
            (other_move, strategy)
        })
        .map(score)
        .sum::<u64>();
    part1
}

fn part1(moves: (&str, &str)) -> u64 {
    match moves {
        ("A", "X") => 1 + 3,
        ("A", "Y") => 2 + 6,
        ("A", "Z") => 3 + 0,

        ("B", "X") => 1 + 0,
        ("B", "Y") => 2 + 3,
        ("B", "Z") => 3 + 6,

        ("C", "X") => 1 + 6,
        ("C", "Y") => 2 + 0,
        ("C", "Z") => 3 + 3,

        _ => panic!("Unknown moves {:?}", moves),
    }
}

fn part2(moves: (&str, &str)) -> u64 {
    match moves {
        ("A", "X") => 3 + 0,
        ("B", "X") => 1 + 0,
        ("C", "X") => 2 + 0,

        ("A", "Y") => 1 + 3,
        ("B", "Y") => 2 + 3,
        ("C", "Y") => 3 + 3,

        ("A", "Z") => 2 + 6,
        ("B", "Z") => 3 + 6,
        ("C", "Z") => 1 + 6,

        _ => panic!("Unknown moves {:?}", moves),
    }
}
