use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    fs::read_to_string,
};

const CHAMBER_WIDTH: usize = 7;
const CHAMBER_LINE: u8 = 0b1111111;
const BLOCK_COUNT: usize = 5;
const BLOCK_WIDTH: [usize; BLOCK_COUNT] = [4, 3, 3, 1, 2];
const BLOCK_HEIGHT: [usize; BLOCK_COUNT] = [1, 3, 3, 4, 2];
const BLOCK_HEIGHT_MAX: usize = 4;
const BLOCK_SHAPES: [[u8; BLOCK_HEIGHT_MAX]; BLOCK_COUNT] = [
    [0b1111, 0, 0, 0],
    [0b010, 0b111, 0b010, 0],
    [0b111, 0b001, 0b001, 0],
    [0b1, 0b1, 0b1, 0b1],
    [0b11, 0b11, 0, 0],
];

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (pushes, pushes_cycle_len) = parse(&input);

    println!("part 1: {}", run(2022, pushes.clone(), pushes_cycle_len));
    println!(
        "part 2: {}",
        run(1_000_000_000_000, pushes, pushes_cycle_len)
    );

    Ok(())
}

fn run(
    rock_wanted: usize,
    mut pushes: std::iter::Cycle<std::str::Chars>,
    pushes_cycle_len: usize,
) -> usize {
    let mut chamber = vec![CHAMBER_LINE];
    let mut tall = chamber.len();
    let mut current_position = new_position(chamber.len());
    let mut landed_rock_count = 0;
    let mut cycles = HashMap::new();
    let mut tick = 0;

    while landed_rock_count < rock_wanted {
        let block_id = block_id(landed_rock_count);
        let push_direction = pushes.next().unwrap();
        let next_position = jet_push(push_direction, block_id, current_position);

        if next_position != current_position && !collides(block_id, &next_position, &chamber) {
            current_position = next_position;
        }

        let next_position = down(current_position);

        if collides(block_id, &next_position, &chamber) {
            (chamber, tall) = rest(chamber, tall, block_id, current_position);
            landed_rock_count += 1;

            if *chamber.last().unwrap() == CHAMBER_LINE {
                // Last rock rested ends creating a full line at the top.
                // It's equivalent to re-initialise the chamber to floor only.
                chamber.clear();
                chamber.push(CHAMBER_LINE);

                match cycles.entry((block_id, tick % pushes_cycle_len)) {
                    Entry::Vacant(e) => {
                        e.insert((tall, landed_rock_count));
                    }
                    Entry::Occupied(e) => {
                        let (memo_tall, memo_landed_rock_count) = e.get();
                        let cycle_tall = tall - memo_tall;
                        let cycle_landed_rock_count = landed_rock_count - memo_landed_rock_count;
                        let jump_count =
                            (rock_wanted - landed_rock_count) / cycle_landed_rock_count;
                        landed_rock_count += jump_count * cycle_landed_rock_count;
                        tall += jump_count * cycle_tall;
                    }
                };
            }
            current_position = new_position(chamber.len());
        } else {
            current_position = next_position;
        }

        tick += 1;
    }

    // Remove the fake floor introduced before the loop
    tall - 1
}

fn parse(input: &str) -> (std::iter::Cycle<std::str::Chars>, usize) {
    let pushes = input.trim().chars().cycle();
    let pushes_cycle_len = input.trim().chars().count();
    (pushes, pushes_cycle_len)
}

fn rest(
    chamber: Vec<u8>,
    tall: usize,
    block_id: usize,
    current_position: (usize, usize),
) -> (Vec<u8>, usize) {
    (0..BLOCK_HEIGHT[block_id]).fold((chamber, tall), |(mut chamber, mut tall), i| {
        let y = current_position.1 + i;
        let shift = CHAMBER_WIDTH - current_position.0 - BLOCK_WIDTH[block_id];
        let block_y = BLOCK_SHAPES[block_id][i] << shift;
        while chamber.len() - 1 < y {
            chamber.push(0);
            tall += 1;
        }
        chamber[y] |= block_y;
        (chamber, tall)
    })
}

fn collides(block_id: usize, current_position: &(usize, usize), chamber: &[u8]) -> bool {
    (0..BLOCK_HEIGHT_MAX).any(|i| {
        let y = current_position.1 + i;
        let chamber_y = chamber.get(y).cloned().unwrap_or_default();
        let shift = CHAMBER_WIDTH - current_position.0 - BLOCK_WIDTH[block_id];
        let block_y = BLOCK_SHAPES[block_id][i] << shift;
        block_y & chamber_y != 0
    })
}

fn new_position(tall: usize) -> (usize, usize) {
    (2, tall + 3)
}

fn jet_push(direction: char, block_id: usize, (x, y): (usize, usize)) -> (usize, usize) {
    match direction {
        '>' => ((x + 1).min(CHAMBER_WIDTH - BLOCK_WIDTH[block_id]), y),
        '<' if x == 0 => (0, y),
        '<' => (x - 1, y),
        d => panic!("Unknown jet direction: {}", d),
    }
}

fn down((x, y): (usize, usize)) -> (usize, usize) {
    (x, y - 1)
}

fn block_id(landed_rock_count: usize) -> usize {
    landed_rock_count % BLOCK_COUNT
}
