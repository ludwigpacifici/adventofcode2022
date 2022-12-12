use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read_to_string,
    str::FromStr,
};

const START: u8 = b'S';
const END: u8 = b'E';

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let heightmap = input.parse::<Matrix>().unwrap();
    let start = heightmap.find(START).unwrap();
    let end = heightmap.find(END).unwrap();

    println!("part 1: {}", dijkstra(&heightmap, start, end));
    println!("part 2: {}", part2(heightmap, end));

    Ok(())
}

fn part2(heightmap: Matrix, end: (usize, usize)) -> usize {
    (0..heightmap.l_len) // look at input, it's obvious the 'a' are first column
        .into_par_iter()
        .map(|s| dijkstra(&heightmap, (s, 0), end))
        .min()
        .unwrap()
}

const INFINITY: usize = 1_000_000;

// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
fn dijkstra(heightmap: &Matrix, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut prev = HashMap::new();
    let mut dist = HashMap::new();
    dist.insert(start, 0);

    let mut q = HashSet::new();
    for l in 0..heightmap.l_len {
        for c in 0..heightmap.c_len {
            q.insert((l, c));
        }
    }

    while !q.is_empty() {
        let u = q
            .iter()
            .cloned()
            .min_by_key(|v| dist.get(v).unwrap_or(&INFINITY))
            .unwrap();
        q.remove(&u);

        for v_i in neighbors(u).into_iter() {
            if let Some(neighbor_height) = heightmap.get_i(v_i) {
                let v = (v_i.0 as usize, v_i.1 as usize);
                let current_height = heightmap.get(u).unwrap();
                if is_accessible(current_height, neighbor_height) && q.contains(&v) {
                    let alt = dist.get(&u).unwrap_or(&INFINITY) + 1;
                    if alt < *dist.get(&v).unwrap_or(&INFINITY) {
                        dist.insert(v, alt);
                        prev.insert(v, u);
                    }
                }
            }
        }
    }

    dist[&end]
}

fn is_accessible(current_height: u8, neighbor_height: u8) -> bool {
    fn elevation(h: u8) -> u8 {
        match h {
            START => b'a',
            END => b'z',
            h => h,
        }
    }

    let current_elevation = elevation(current_height);
    let neighbor_elevation = elevation(neighbor_height);

    neighbor_elevation <= current_elevation + 1
}

fn neighbors((l, c): (usize, usize)) -> Vec<(isize, isize)> {
    let l = l as isize;
    let c = c as isize;
    vec![(l - 1, c), (l + 1, c), (l, c - 1), (l, c + 1)]
}

struct Matrix {
    data: Vec<Vec<u8>>,
    l_len: usize,
    c_len: usize,
}

impl Matrix {
    fn get(&self, (l, c): (usize, usize)) -> Option<u8> {
        if self.l_len <= l || self.c_len <= c {
            None
        } else {
            Some(self.data[l][c])
        }
    }

    fn get_i(&self, (l, c): (isize, isize)) -> Option<u8> {
        if l < 0 || c < 0 {
            None
        } else {
            self.get((l as usize, c as usize))
        }
    }

    fn find(&self, x: u8) -> Option<(usize, usize)> {
        for l in 0..self.l_len {
            for c in 0..self.c_len {
                if self.data[l][c] == x {
                    return Some((l, c));
                }
            }
        }
        None
    }
}

impl FromStr for Matrix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.lines().map(|l| l.as_bytes().to_vec()).collect::<Vec<_>>();

        let l_len = data.len();
        let c_len = data[0].len();

        Ok(Matrix { data, l_len, c_len })
    }
}
