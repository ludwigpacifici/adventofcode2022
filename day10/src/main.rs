use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;
    let signal = signal(input);

    println!("part 1: {:?}", part1(&signal));
    println!("part 2:\n{}", part2(signal));

    Ok(())
}

fn part2(signal: Vec<isize>) -> String {
    (0..6).fold(String::new(), |part2, i| {
        let mut part2 =
            signal[i * 40..(i + 1) * 40]
                .iter()
                .enumerate()
                .fold(part2, |mut s, (cycle, &x)| {
                    let cycle = cycle as isize + 1;
                    if x <= cycle && cycle < x + 3 {
                        s.push('█');
                    } else {
                        s.push(' ');
                    }
                    s
                });
        part2.push('\n');
        part2
    })
}

fn part1(signal: &[isize]) -> isize {
    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|i| i as isize * signal[i - 1])
        .sum::<isize>()
}

fn signal(input: String) -> Vec<isize> {
    input
        .lines()
        .map(|l| l.split_ascii_whitespace().nth(1))
        .map(|o| o.map(|n| n.parse::<isize>().unwrap()))
        .fold(vec![1], |mut history, i| {
            let &x = history.last().unwrap();
            history.push(x);
            if let Some(n) = i {
                history.push(x + n);
            }
            history
        })
}
