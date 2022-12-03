use std::{collections::HashSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("part 1:{:?}", part1(&input));
    println!("part 2:{:?}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let left_len = l.len() / 2;
            let (l, r) = l.as_bytes().split_at(left_len);
            let (l, r) = (
                l.iter().collect::<HashSet<_>>(),
                r.iter().collect::<HashSet<_>>(),
            );
            r.into_iter()
                .filter(|c| l.contains(c))
                .map(priority)
                .sum::<u64>()
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.as_bytes())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|x| match x {
            &[a, b, c] => {
                let b = b.iter().collect::<HashSet<_>>();
                let c = c.iter().collect::<HashSet<_>>();

                a.iter()
                    .find(|x| b.contains(x) && c.contains(x))
                    .expect("Cannot find one in all 3 sets")
            }
            c => panic!("Unexpected chunk: {:?}", c),
        })
        .map(priority)
        .sum()
}

fn priority(&c: &u8) -> u64 {
    match c {
        b'a'..=b'z' => c as u64 - 96,
        b'A'..=b'Z' => c as u64 - 38,
        c => panic!("Unknown value: {:?}", c),
    }
}
