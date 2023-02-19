use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs::read_to_string,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let lava = input
        .lines()
        .map(|l| {
            let mut it = l.split(',').map(|n| n.parse::<isize>().unwrap());
            (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
        })
        .collect::<HashSet<_>>();

    let part1 = lava.iter().fold(0, |part1, &(x, y, z)| {
        part1
            + [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ]
            .into_iter()
            .filter(|n| !lava.contains(n))
            .count()
    });

    println!("part 1: {part1}");

    let (xmin, xmax, ymin, ymax, zmin, zmax) = lava.iter().fold(
        (2, 2, 2, 2, 2, 2),
        |(xmin, xmax, ymin, ymax, zmin, zmax), &(x, y, z)| {
            (
                xmin.min(x),
                xmax.max(x),
                ymin.min(y),
                ymax.max(y),
                zmin.min(z),
                zmax.max(z),
            )
        },
    );

    let mut seen = HashSet::new();
    let mut q = VecDeque::from([(xmin, ymin, zmin)]);
    let mut part2 = 0;

    while let Some((x, y, z)) = q.pop_front() {
        if seen.contains(&(x, y, z)) {
            continue;
        } else {
            seen.insert((x, y, z));
        }

        for (xn, yn, zn) in [
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ] {
            if xn < xmin - 1
                || xmax + 1 < xn
                || yn < ymin - 1
                || ymax + 1 < yn
                || zn < zmin - 1
                || zmax + 1 < zn
            {
                continue;
            } else if lava.contains(&(xn, yn, zn)) {
                part2 += 1;
            } else {
                q.push_back((xn, yn, zn))
            }
        }
    }

    println!("part 2: {part2}");

    Ok(())
}
