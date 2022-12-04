use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("part 1: {:?}", run(&input, part1));
    println!("part 2: {:?}", run(&input, part2));

    Ok(())
}

fn part1((e1, e2): &((u32, u32), (u32, u32))) -> bool {
    e1.0 <= e2.0 && e2.1 <= e1.1 || e2.0 <= e1.0 && e1.1 <= e2.1
}

fn part2((e1, e2): &((u32, u32), (u32, u32))) -> bool {
    !(e1.1 < e2.0 || e2.1 < e1.0)
}

fn run<L>(input: &str, logic: L) -> usize
where
    L: Fn(&((u32, u32), (u32, u32))) -> bool,
{
    input
        .lines()
        .map(|l| {
            let mut it = l.split(',');
            let (Some(e1), Some(e2)) = (it.next(), it.next()) else { panic!("Cannot split coma: {:?}", l) };
            let mut it = e1.split('-');
            let (Some(e1l), Some(e1h)) = (it.next(), it.next()) else { panic!("Cannot split dash: {:?}", e1) };
            let mut it = e2.split('-');
            let (Some(e2l), Some(e2h)) = (it.next(), it.next()) else { panic!("Cannot split dash: {:?}", e2) };

            let e1l = e1l.parse::<u32>().expect("Cannot parse integer");
            let e1h = e1h.parse::<u32>().expect("Cannot parse integer");
            let e2l = e2l.parse::<u32>().expect("Cannot parse integer");
            let e2h = e2h.parse::<u32>().expect("Cannot parse integer");

            ((e1l, e1h), (e2l, e2h))
        })
        .filter(logic)
        .count()
}
