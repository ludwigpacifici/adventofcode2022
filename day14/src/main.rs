use std::{collections::HashSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (cave, y_max) = parse(input);

    let part1 = run(&cave, 100000, |(_, y)| y > y_max) - 1;
    println!("part 1: {}", part1);

    let part2 = run(&cave, y_max + 2, |sand| sand == (501, 1));
    println!("part 2: {}", part2);

    Ok(())
}

fn run<F>(cave: &HashSet<(usize, usize)>, floor: usize, stop: F) -> usize
where
    F: Fn((usize, usize)) -> bool,
{
    let mut sand_heap = HashSet::new();
    let mut sand = (500, 0);

    loop {
        let next = (sand.0, sand.1 + 1);
        if !cave.contains(&next) && !sand_heap.contains(&next) && next.1 < floor {
            sand = next;
            continue;
        }

        let next = (sand.0 - 1, sand.1 + 1);
        if !cave.contains(&next) && !sand_heap.contains(&next) && next.1 < floor {
            sand = next;
            continue;
        }

        let next = (sand.0 + 1, sand.1 + 1);
        if !cave.contains(&next) && !sand_heap.contains(&next) && next.1 < floor {
            sand = next;
            continue;
        }

        sand_heap.insert(sand);
        sand = (500, 0);

        if stop(next) {
            break;
        }
    }

    sand_heap.len()
}

fn parse(input: String) -> (HashSet<(usize, usize)>, usize) {
    let cave = input.lines().fold(HashSet::new(), |mut cave, l| {
        let bounds = l
            .split(" -> ")
            .map(|cs| {
                let (x, y) = cs.split_once(',').unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .collect::<Vec<_>>();

        for b in bounds.windows(2) {
            let (x0, y0) = b[0];
            let (x1, y1) = b[1];
            if x0 == x1 {
                for y in y0.min(y1)..=y0.max(y1) {
                    cave.insert((x0, y));
                }
            } else {
                for x in x0.min(x1)..=x0.max(x1) {
                    cave.insert((x, y0));
                }
            }
        }
        cave
    });

    let y_max = *cave.iter().map(|(_, y)| y).max().unwrap();

    (cave, y_max)
}
