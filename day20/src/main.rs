use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mixing = mix(&input, 1, 1);
    let part1 = score(mixing);
    println!("part1: {part1}");

    let mixing = mix(&input, 811589153, 10);
    let part2 = score(mixing);
    println!("part2: {part2}");
}

fn mix(input: &str, decryption_key: isize, repeat_n: usize) -> VecDeque<(usize, isize)> {
    let mut mixing = input
        .lines()
        .map(|l| l.parse::<isize>().unwrap() * decryption_key)
        .enumerate()
        .collect::<VecDeque<_>>();

    for _ in 0..repeat_n {
        for i in 0..mixing.len() {
            let (from, v) = mixing
                .iter()
                .enumerate()
                .find(|(_, x)| x.0 == i)
                .map(|(im, x)| (im, x.1))
                .unwrap();

            if v == 0 {
                continue;
            }

            mixing.rotate_left(from);
            let popped = mixing.pop_front().unwrap();
            let to = popped.1.rem_euclid(mixing.len() as isize) as usize;
            mixing.rotate_left(to);
            mixing.push_front(popped);
        }
    }

    mixing
}

fn score(mixing: VecDeque<(usize, isize)>) -> isize {
    let i_0 = mixing.iter().position(|x| x.1 == 0).unwrap();

    [1000, 2000, 3000]
        .into_iter()
        .map(|n| mixing[(i_0 + n) % mixing.len()].1)
        .sum()
}
