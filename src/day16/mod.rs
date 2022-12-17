use itertools::Itertools;
use std::collections::{HashMap};

use crate::{grid::Grid, util};



type Input = Grid<Valve>;

pub fn main() {
    println!("Day16");
    problem1(parse(include_str!("puzzle.txt")));
}
fn problem1(grid: Input) -> usize {
    let mut jumps = vec![];
    let mut pos_to_idx = HashMap::new();
    let mut idx_to_rate = vec![];
    let mut distances = vec![];

    grid.for_every(|pos, model| {
        let my_index = pos_to_idx.len();
        pos_to_idx.insert(*pos, my_index);
        idx_to_rate.push(model.rate);
        distances.push(vec![]);

        if model.rate > 0 {
            println!("Idx: {}, rate:{}", my_index, model.rate);
            jumps.push(my_index);
        }
    });

    grid.for_every(|pos, _| {
        let my_idx = pos_to_idx.get(pos).unwrap();
        for _ in 0..distances.len() {
            distances[*my_idx].push(0);
        }

        grid.bfs(pos, |other_pos, dist| {
            let his_idx = pos_to_idx.get(other_pos).unwrap();
            distances[*my_idx][*his_idx] = dist as usize;
        });
    });

    let mut best = 0;

    let permutations = factorial(15);
    let mut counter = 0_u128;
    println!("  Starting, permutations: {}", permutations);
    for prm in jumps.iter().permutations(jumps.len()).unique() {
        let cand = run_faster(prm, &distances, &idx_to_rate);
        if cand > best {
            best = cand;
            println!("  Found better: {}", best);
        }
        if counter % 100000000 == 0 {
            println!("{} / {} done", counter, permutations);
        }
        counter += 1;
    }

    best
}

fn factorial(num: u128) -> u128 {
    match num {
        0 => 1,
        1.. => (1..num + 1).product(),
    }
}

fn run_faster(list: Vec<&usize>, distances: &Vec<Vec<usize>>, rates: &Vec<usize>) -> usize {
    let mut minute = 1_isize;
    let max_round = 30;
    let mut pressure_per_round = 0_usize;
    let mut released_pressure = 0;

    let mut current = 0;

    let mut at = 0;

    while minute <= max_round {
        let minutes_remaining = (max_round - minute) as usize;
        if let Some(idx_s) = list.get(at) {
            let idx = *idx_s;
            at += 1;

            let distance = distances[current][*idx];
            let rate = rates[*idx];

            if distance >= minutes_remaining {
                continue;
            }

            let gaps = distance + 1;
            minute += gaps as isize;

            // println!("Took {} on minute {}", rate, (minute - 1));

            released_pressure += pressure_per_round * gaps;
            current = *idx;
            pressure_per_round += rate;
            continue;
        }

        released_pressure += pressure_per_round;
        minute += 1;
    }

    released_pressure
}

fn problem2(input: Input) -> isize {
    0
}

#[derive(Debug, PartialEq, Clone)]
struct Valve {
    rate: usize,
    name: String,
    potential: usize,
    closed: bool,
}

fn parse(s: &str) -> Input {
    let mut grid = Grid::new_from_list(s, |_, line| {
        let spl: Vec<&str> = line.split_whitespace().collect();
        let valve_name = spl[1];

        let rate = util::parse_numbers(line)[0] as usize;

        Valve { name: valve_name.to_string(), rate, potential: 0, closed: false }
    });

    let mut valve_to_position = HashMap::new();
    grid.for_every(|pos, valve| {
        valve_to_position.insert(valve.name.clone(), *pos);
    });

    s.lines().for_each(|line| {
        let spl: Vec<&str> = line.split_whitespace().collect();
        let from = valve_to_position.get(spl[1]).unwrap();

        for i in 9..spl.len() {
            let to = valve_to_position.get(&spl[i].replace(",", "")).unwrap();
            grid.add_connections(vec![(*from, *to)]);
        }
    });

    grid
}

#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_problem_1() {
        assert_eq!(1651, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(0, problem1(parse(include_str!("puzzle.txt"))));
    }

    #[test]
    fn test_problem_2() {
        assert_eq!(0, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(0, problem2(parse(include_str!("puzzle.txt"))));
    }
}
