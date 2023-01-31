use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs::read_to_string,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (valves, rates, next_valves) = parse(&input);

    let index = valves
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();

    let start = "AA";
    let reach = HashMap::new();

    let total_time = 30;
    let score = find_rates_1(&index, &next_valves, &rates, reach, start, total_time);
    println!("part1: {score}");

    let reach = HashMap::new();
    let total_time = 26;
    let score = find_rates_2(
        &index,
        &next_valves,
        &rates,
        reach,
        start,
        [total_time, total_time],
    );
    println!("part2: {score:?}");

    Ok(())
}

type Reach<'a> = HashMap<&'a str, Vec<(&'a str, u64)>>;
type Time = u64;
type Score = u64;
type Opened = usize;
type Valve = usize;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct State2<'a> {
    position: [&'a str; 2],
    time_left: [Time; 2],
    opened: Opened,
    score: Score,
}

fn find_rates_2<'a>(
    index: &HashMap<&str, usize>,
    next_valves: &HashMap<&'a str, Vec<&'a str>>,
    rates: &HashMap<&str, u64>,
    mut reach: Reach<'a>,
    start: &'a str,
    total_time: [Time; 2],
) -> Score {
    let state = State2 {
        position: [start, start],
        time_left: [total_time[0] + 1, total_time[1] + 1], // because AA will consume one for nothing
        opened: 0,
        score: 0,
    };
    let mut best = state;
    let mut to_visit = VecDeque::from([state]);
    let mut seen = HashSet::new();

    while let Some(mut s) = to_visit.pop_front() {
        if seen.contains(&s) {
            continue;
        } else {
            let sym = State2 {
                position: [s.position[1], s.position[0]],
                time_left: [s.time_left[1], s.time_left[0]], // because AA will consume one for nothing
                ..s
            };
            seen.insert(s);
            seen.insert(sym);
        }

        let opened = open(s.opened, index[s.position[0]]);
        if s.time_left[0] > 0 {
            s = State2 {
                time_left: [s.time_left[0] - 1, s.time_left[1]],
                opened,
                score: s.score + rates[s.position[0]] * (s.time_left[0] - 1),
                ..s
            };
        }

        let opened = open(opened, index[s.position[1]]);
        if s.time_left[1] > 0 {
            s = State2 {
                time_left: [s.time_left[0], s.time_left[1] - 1],
                opened,
                score: s.score + rates[s.position[1]] * (s.time_left[1] - 1),
                ..s
            };
        }

        if best.score < s.score {
            best = s;
        }

        if s.time_left[0] <= 1 && s.time_left[1] <= 1 {
            continue;
        }

        reach = can_reach(index, next_valves, rates, reach, s.position[0]);
        reach = can_reach(index, next_valves, rates, reach, s.position[1]);

        let destinations1 = reach[s.position[0]]
            .iter()
            .filter(|(v, t)| *t < s.time_left[0] && !is_opened(s.opened, index[v]) && *t < 7);

        let destinations2 = reach[s.position[1]]
            .iter()
            .filter(|(v, t)| *t < s.time_left[1] && !is_opened(s.opened, index[v]) && *t < 7)
            .collect::<Vec<_>>();

        let mut no_sym = HashSet::new();

        for (next_v1, t1) in destinations1 {
            for (next_v2, t2) in destinations2.iter() {
                if next_v1 == next_v2 {
                    continue;
                }

                if no_sym.contains(&(next_v1, t1, next_v2, t2))
                    || no_sym.contains(&(next_v2, t2, next_v1, t1))
                {
                    continue;
                } else {
                    no_sym.insert((next_v1, t1, next_v2, t2));
                    no_sym.insert((next_v2, t2, next_v1, t1));
                }

                let with_valve = State2 {
                    position: [*next_v1, *next_v2],
                    time_left: [s.time_left[0] - t1, s.time_left[1] - t2],
                    opened,
                    ..s
                };
                to_visit.push_back(with_valve);
            }
        }
    }

    best.score
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct State1<'a> {
    position: &'a str,
    opened: Opened,
    time_left: Time,
    score: Score,
}

fn find_rates_1<'a>(
    index: &HashMap<&str, usize>,
    next_valves: &HashMap<&'a str, Vec<&'a str>>,
    rates: &HashMap<&str, u64>,
    mut reach: Reach<'a>,
    start: &'a str,

    total_time: Time,
) -> Score {
    let state = State1 {
        position: start,
        opened: 0,
        time_left: total_time + 1, // because AA will consume one for nothing
        score: 0,
    };
    let mut best = state;
    let mut to_visit = VecDeque::from([state]);
    let mut seen = HashSet::new();

    while let Some(mut s) = to_visit.pop_front() {
        if seen.contains(&s) {
            continue;
        } else {
            seen.insert(s);
        }

        let opened = open(s.opened, index[s.position]);

        if s.time_left > 0 {
            s = State1 {
                time_left: s.time_left - 1,
                opened,
                score: s.score + rates[s.position] * (s.time_left - 1),
                ..s
            };
        }

        if best.score < s.score {
            best = s;
        }

        if s.time_left <= 1 {
            continue;
        }

        reach = can_reach(index, next_valves, rates, reach, s.position);

        let destinations = reach[s.position]
            .iter()
            .filter(|(v, t)| *t < s.time_left && !is_opened(s.opened, index[v]));

        for (next_v, t) in destinations {
            let with_valve = State1 {
                time_left: s.time_left - t,
                position: next_v,
                opened,
                ..s
            };
            to_visit.push_back(with_valve);
        }
    }

    best.score
}

fn can_reach<'a>(
    index: &HashMap<&str, usize>,
    next_valves: &HashMap<&'a str, Vec<&'a str>>,
    rates: &HashMap<&str, u64>,
    mut reach: Reach<'a>,
    start: &'a str,
) -> Reach<'a> {
    if reach.contains_key(start) {
        return reach;
    }
    let mut to_visit = VecDeque::from([(start, 0)]);
    let mut opened = 0;

    while let Some((v, t)) = to_visit.pop_front() {
        if is_opened(opened, index[v]) {
            continue;
        }

        opened = open(opened, index[v]);

        if t > 0 && rates[v] > 0 {
            reach.entry(start).or_insert_with(Vec::new).push((v, t));
        }

        for next_valve in next_valves[v]
            .iter()
            .filter(|x| !is_opened(opened, index[**x]))
        {
            to_visit.push_back((next_valve, t + 1));
        }
    }

    reach
}

fn is_opened(valves: Opened, v: Valve) -> bool {
    (valves >> v) & 1 == 1
}

fn open(valves: Opened, v: Valve) -> Opened {
    (1 << v) | valves
}

fn parse(input: &str) -> (Vec<&str>, HashMap<&str, u64>, HashMap<&str, Vec<&str>>) {
    let (valves, rates, next_valves) = input
        .lines()
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            let valve = it.nth(1).unwrap();
            let rate = it
                .nth(2)
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .trim_end_matches(';')
                .parse::<u64>()
                .unwrap();

            let next_valves = it
                .skip(4)
                .map(|v| v.trim_end_matches(','))
                .collect::<Vec<_>>();

            (valve, rate, next_valves)
        })
        .fold(
            (Vec::new(), HashMap::new(), HashMap::new()),
            |(mut valves, mut rates, mut next_valves), (valve, rate, next)| {
                valves.push(valve);
                rates.insert(valve, rate);
                next_valves.insert(valve, next);
                (valves, rates, next_valves)
            },
        );
    (valves, rates, next_valves)
}
