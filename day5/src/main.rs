use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    // Crates parsing is left as an exercise for the reader
    let crates = vec![
        vec!['Z', 'T', 'F', 'R', 'W', 'J', 'G'],
        vec!['G', 'W', 'M'],
        vec!['J', 'N', 'H', 'G'],
        vec!['J', 'R', 'C', 'N', 'W'],
        vec!['W', 'F', 'S', 'B', 'G', 'Q', 'V', 'M'],
        vec!['S', 'R', 'T', 'D', 'V', 'W', 'C'],
        vec!['H', 'B', 'N', 'C', 'D', 'Z', 'G', 'V'],
        vec!['S', 'J', 'N', 'M', 'G', 'C'],
        vec!['G', 'P', 'N', 'W', 'C', 'J', 'D', 'L'],
    ];

    let input = read_to_string("input.txt")?;

    println!(
        "part 1: {:?}",
        run(&input, crates.clone(), crate_mover_9000)
    );
    println!("part 2: {:?}", run(&input, crates, crate_mover_9001));

    Ok(())
}

type Crates = Vec<Vec<char>>;

fn run<L>(input: &str, crates: Crates, logic: L) -> String
where
    L: Fn(Crates, usize, usize, usize) -> Crates,
{
    input
        .lines()
        .skip(10)
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            (
                it.nth(1).unwrap().parse::<usize>().unwrap(),
                it.nth(1).unwrap().parse::<usize>().unwrap(),
                it.nth(1).unwrap().parse::<usize>().unwrap(),
            )
        })
        .fold(crates, |crates, (size, from, to)| {
            logic(crates, size, from - 1, to - 1)
        })
        .iter()
        .filter_map(|s| s.last())
        .collect()
}

fn crate_mover_9000(mut crates: Crates, size: usize, from: usize, to: usize) -> Crates {
    for _ in 0..size {
        let c = crates[from].pop().unwrap();
        crates[to].push(c);
    }

    crates
}

fn crate_mover_9001(mut crates: Crates, size: usize, from: usize, to: usize) -> Crates {
    let len = crates[from].len() - size;
    let mut o = crates[from].split_off(len);
    crates[to].append(&mut o);
    crates
}
