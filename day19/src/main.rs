use rayon::prelude::*;
use std::{collections::HashSet, collections::VecDeque, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let blueprints = input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            (
                it.nth(6).unwrap().parse::<usize>().unwrap(),
                it.nth(5).unwrap().parse::<usize>().unwrap(),
                it.nth(5).unwrap().parse::<usize>().unwrap(),
                it.nth(2).unwrap().parse::<usize>().unwrap(),
                it.nth(5).unwrap().parse::<usize>().unwrap(),
                it.nth(2).unwrap().parse::<usize>().unwrap(),
            )
        })
        .map(Blueprint::new)
        .collect::<Vec<_>>();

    let part1 = part1(&blueprints, 24);
    println!("part1: {part1}");

    let part2 = part2(&blueprints[..3], 32);
    println!("part1: {part2}");

    Ok(())
}

fn part1(blueprints: &[Blueprint], time: usize) -> usize {
    blueprints
        .into_par_iter()
        .enumerate()
        .map(|(i, blueprint)| (i + 1) * mine(blueprint, time).geode)
        .sum()
}

fn part2(blueprints: &[Blueprint], time: usize) -> usize {
    blueprints
        .into_par_iter()
        .map(|blueprint| mine(blueprint, time).geode)
        .product()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
struct Bank {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Robots {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Robots {
    fn new() -> Self {
        Robots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    r: Robots,
    b: Bank,
    t: usize,
}

fn mine(blueprint: &Blueprint, time: usize) -> Bank {
    let mut q = VecDeque::from([State {
        r: Robots::new(),
        b: Bank::default(),
        t: time,
    }]);
    let mut best_bank = Bank::default();
    let mut seen = HashSet::new();

    let max_ore_robots = [
        blueprint.ore_robot,
        blueprint.clay_robot,
        blueprint.obsidian_robot.0,
        blueprint.geode_robot.0,
    ]
    .into_iter()
    .max()
    .unwrap();
    let max_clay_robots = blueprint.obsidian_robot.1;
    let max_obsidian_robots = blueprint.geode_robot.1;

    while let Some(state) = q.pop_front() {
        if seen.contains(&state) {
            continue;
        } else {
            seen.insert(state.clone());
        }

        if best_bank.geode < state.b.geode {
            best_bank = state.b.clone();
        }

        if state.t == 0 {
            continue;
        }

        if state.b.geode + 1 < best_bank.geode {
            continue;
        }

        if blueprint.geode_robot.0 <= state.b.ore && blueprint.geode_robot.1 <= state.b.obsidian {
            let state = State {
                r: Robots {
                    geode: state.r.geode + 1,
                    ..state.r
                },
                b: Bank {
                    ore: state.b.ore + state.r.ore - blueprint.geode_robot.0,
                    clay: state.b.clay + state.r.clay,
                    obsidian: state.b.obsidian + state.r.obsidian - blueprint.geode_robot.1,
                    geode: state.b.geode + state.r.geode,
                },
                t: state.t - 1,
            };
            q.push_back(state);
        }

        if blueprint.obsidian_robot.0 <= state.b.ore
            && blueprint.obsidian_robot.1 <= state.b.clay
            && state.r.obsidian <= max_obsidian_robots
        {
            let state = State {
                r: Robots {
                    obsidian: state.r.obsidian + 1,
                    ..state.r
                },
                b: Bank {
                    ore: state.b.ore + state.r.ore - blueprint.obsidian_robot.0,
                    clay: state.b.clay + state.r.clay - blueprint.obsidian_robot.1,
                    obsidian: state.b.obsidian + state.r.obsidian,
                    geode: state.b.geode + state.r.geode,
                },
                t: state.t - 1,
            };
            q.push_back(state);
        }

        if blueprint.clay_robot <= state.b.ore && state.r.clay <= max_clay_robots {
            let state = State {
                r: Robots {
                    clay: state.r.clay + 1,
                    ..state.r
                },
                b: Bank {
                    ore: state.b.ore + state.r.ore - blueprint.clay_robot,
                    clay: state.b.clay + state.r.clay,
                    obsidian: state.b.obsidian + state.r.obsidian,
                    geode: state.b.geode + state.r.geode,
                },
                t: state.t - 1,
            };
            q.push_back(state);
        }

        if blueprint.ore_robot <= state.b.ore && state.r.ore <= max_ore_robots {
            let state = State {
                r: Robots {
                    ore: state.r.ore + 1,
                    ..state.r
                },
                b: Bank {
                    ore: state.b.ore + state.r.ore - blueprint.ore_robot,
                    clay: state.b.clay + state.r.clay,
                    obsidian: state.b.obsidian + state.r.obsidian,
                    geode: state.b.geode + state.r.geode,
                },
                t: state.t - 1,
            };
            q.push_back(state);
        }

        if 0 < state.t {
            let state = State {
                b: Bank {
                    ore: state.b.ore + state.r.ore,
                    clay: state.b.clay + state.r.clay,
                    obsidian: state.b.obsidian + state.r.obsidian,
                    geode: state.b.geode + state.r.geode,
                },
                t: state.t - 1,
                ..state
            };
            q.push_back(state);
        }
    }

    best_bank
}

#[derive(Debug)]
struct Blueprint {
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: (usize, usize),
    geode_robot: (usize, usize),
}

impl Blueprint {
    fn new((a, b, c, d, e, f): (usize, usize, usize, usize, usize, usize)) -> Self {
        Blueprint {
            ore_robot: a,
            clay_robot: b,
            obsidian_robot: (c, d),
            geode_robot: (e, f),
        }
    }
}
