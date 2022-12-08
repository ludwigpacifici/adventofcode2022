use std::{collections::HashSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let input = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let l_len = input.len();
    let c_len = input[0].len();

    println!("part 1: {:?}", part1(&input, l_len, c_len));
    println!("part 2: {:?}", part2(&input, l_len, c_len));

    Ok(())
}

fn part2(input: &Vec<Vec<u8>>, l_len: usize, c_len: usize) -> usize {
    (1..l_len - 1)
        .flat_map(|l| (1..c_len - 1).map(move |c| (l, c)))
        .map(|(l, c)| scenic_score(input, l, c))
        .max()
        .unwrap()
}

fn part1(input: &[Vec<u8>], l_len: usize, c_len: usize) -> usize {
    let mut visible = HashSet::new();

    let mut prev_max = 0;
    for l in 0..l_len {
        for c in 0..c_len {
            if input[l][c] > prev_max {
                visible.insert((l, c));
                prev_max = input[l][c];
            }
        }
        prev_max = 0;
    }

    let mut prev_max = 0;
    for l in 0..l_len {
        for c in 0..c_len {
            let c = c_len - c - 1;
            if input[l][c] > prev_max {
                visible.insert((l, c));
                prev_max = input[l][c];
            }
        }
        prev_max = 0;
    }

    let mut prev_max = 0;
    for c in 0..c_len {
        for l in 0..l_len {
            if input[l][c] > prev_max {
                visible.insert((l, c));
                prev_max = input[l][c];
            }
        }
        prev_max = 0;
    }

    let mut prev_max = 0;
    for c in 0..c_len {
        for l in 0..l_len {
            let l = l_len - l - 1;
            if input[l][c] > prev_max {
                visible.insert((l, c));
                prev_max = input[l][c];
            }
        }
        prev_max = 0;
    }

    visible.len()
}

fn scenic_score(input: &Vec<Vec<u8>>, l_origin: usize, c_origin: usize) -> usize {
    let l_len = input.len();
    let c_len = input[0].len();
    let tree_house_size = input[l_origin][c_origin];

    let mut left: isize = c_origin as isize;
    while left > 0 {
        left -= 1;
        if tree_house_size <= input[l_origin][left as usize] {
            break;
        }
    }

    let mut right = c_origin;
    while right < c_len - 1 {
        right += 1;
        if tree_house_size <= input[l_origin][right] {
            break;
        }
    }

    let mut top: isize = l_origin as isize;
    while top > 0 {
        top -= 1;
        if tree_house_size <= input[top as usize][c_origin] {
            break;
        }
    }

    let mut bottom = l_origin;
    while bottom < l_len - 1 {
        bottom += 1;
        if tree_house_size <= input[bottom as usize][c_origin] {
            break;
        }
    }

    (c_origin - left as usize)
        * (right - c_origin)
        * (l_origin - top as usize)
        * (bottom - l_origin)
}
