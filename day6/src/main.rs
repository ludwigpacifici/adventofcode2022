use std::{collections::HashSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("part 1: {:?}", run(&input, 4));
    println!("part 2: {:?}", run(&input, 14));

    Ok(())
}

fn run(input: &str, window_len: usize) -> usize {
    input
        .as_bytes()
        .windows(window_len)
        .enumerate()
        .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == window_len)
        .map(|(i, _)| i + window_len)
        .expect("No window with unique elements")
}
