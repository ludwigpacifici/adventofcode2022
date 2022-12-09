use std::{collections::HashSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("part 1: {:?}", run(&input, 2));
    println!("part 2: {:?}", run(&input, 10));

    Ok(())
}

fn run(input: &str, rope_len: usize) -> usize {
    let (_, visited) = input
        .lines()
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            let directory = it.next().unwrap();
            let len = it.next().unwrap().parse::<isize>().unwrap();
            (directory, len)
        })
        .map(|(d, l)| match d {
            "U" => ((0, 1), l),
            "D" => ((0, -1), l),
            "L" => ((-1, 0), l),
            "R" => ((1, 0), l),
            d => panic!("Unknown move: {:?}", d),
        })
        .fold(
            (vec![(0, 0); rope_len], HashSet::new()),
            |(rope, visited), (dxdy, l)| move_rope(dxdy, l, rope, visited),
        );
    visited.len()
}

fn move_rope(
    (dx, dy): (isize, isize),
    len: isize,
    mut rope: Vec<(isize, isize)>,
    mut visited: HashSet<(isize, isize)>,
) -> (Vec<(isize, isize)>, HashSet<(isize, isize)>) {
    for _ in 0..len {
        rope[0].0 += dx;
        rope[0].1 += dy;

        for i in 1..rope.len() {
            let dix = rope[i - 1].0 - rope[i].0;
            let diy = rope[i - 1].1 - rope[i].1;

            if dix.abs() == 2 && diy == 0 {
                rope[i].0 += if dix > 0 { 1 } else { -1 };
            } else if dix == 0 && diy.abs() == 2 {
                rope[i].1 += if diy > 0 { 1 } else { -1 };
            } else if dix.abs() >= 2 || diy.abs() >= 2 {
                rope[i].0 += if dix > 0 { 1 } else { -1 };
                rope[i].1 += if diy > 0 { 1 } else { -1 };
            }
        }

        visited.insert(*rope.last().unwrap());
    }

    (rope, visited)
}
