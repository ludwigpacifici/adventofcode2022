use std::{error::Error, fs::read_to_string, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(input));

    Ok(())
}

fn part2(mut input: String) -> usize {
    input.push_str("[[2]]\n[[6]]\n");

    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, l)| (i, l.parse::<Packet>()))
        .collect::<Vec<_>>();

    packets.sort_unstable_by(|(_, l), (_, r)| l.cmp(r));

    let len = packets.len();

    packets
        .into_iter()
        .enumerate()
        .filter_map(|(postion, (initial_position, _))| {
            if initial_position == len - 1 || initial_position == len - 2 {
                Some(postion + 1)
            } else {
                None
            }
        })
        .product::<usize>()
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|ll| {
            ll.split_ascii_whitespace()
                .map(|l| l.parse::<Packet>())
                .collect::<Vec<_>>()
        })
        .enumerate()
        .filter_map(|(i, ps)| if ps[0] <= ps[1] { Some(i + 1) } else { None })
        .sum::<usize>()
}

fn parse_digit(s: &[u8], i: usize) -> (usize, u32) {
    let mut j = i;
    let mut n = 0;

    while j < s.len() && s[j].is_ascii_digit() {
        n = n * 10 + (s[j] as u32 - b'0' as u32);
        j += 1;
    }

    (j, n)
}

#[derive(Debug, Eq)]
enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn to_list(v: u32) -> Packet {
        Packet::List(vec![Packet::Value(v)])
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Value(s), Packet::Value(o)) => s.partial_cmp(o),
            (Packet::List(s), Packet::List(o)) => s
                .iter()
                .zip(o.iter())
                .find(|(s, o)| *s != *o)
                .map_or_else(|| s.len().partial_cmp(&o.len()), |(s, o)| s.partial_cmp(o)),
            (s, Packet::Value(o)) => s.partial_cmp(&Packet::to_list(*o)),
            (Packet::Value(s), o) => Packet::to_list(*s).partial_cmp(o),
        }
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_inner(s: &[u8], mut i: usize) -> Result<(usize, Packet), String> {
            let mut ret = Vec::new();

            while i < s.len() {
                match s[i] {
                    b']' => {
                        return Ok((i + 1, Packet::List(ret)));
                    }
                    b'[' => {
                        let (j, inner) = parse_inner(s, i + 1).unwrap();
                        i = j;
                        ret.push(inner);
                    }
                    c if c.is_ascii_digit() => {
                        let (j, n) = parse_digit(s, i);
                        i = j;
                        ret.push(Packet::Value(n));
                    }
                    c if c == b',' => {
                        i += 1;
                    }
                    c => {
                        return Err(format!("Unknown char {}", c as char));
                    }
                }
            }

            Ok((i, Packet::List(ret)))
        }

        parse_inner(s.as_bytes(), 0).map(|x| x.1)
    }
}
