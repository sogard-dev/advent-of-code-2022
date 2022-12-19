type Input = Vec<Blueprint>;
use std::time::Instant;

pub fn main() {
    println!("19");
}

fn print(blueprint: &Blueprint) {
    println!("Blueprint {}:", blueprint.id);
    println!("  Each ore robot costs {} ore.", blueprint.ore_robot_cost_ore);
    println!("  Each clay robot costs {} ore.", blueprint.clay_robot_cost_ore);
    println!("  Each obsidian robot costs {} ore and {} clay.", blueprint.obsidian_robot_cost_ore, blueprint.obsidian_robot_cost_clay);
    println!("  Each geode robot costs {} ore and {} obsidian.", blueprint.geode_robot_cost_ore, blueprint.geode_robot_cost_obsidian);
    println!("");
}

fn problem1(input: Input) -> usize {
    input
        .iter()
        .map(|b| {
            let start = Instant::now();
            let res = solve(b, &24, 1, 1, 0, 0, 0, 0, 0, 0, 0, &mut 0);
            let duration = start.elapsed();
            println!("Blueprint {}: {} in time: {}ms", b.id, res, duration.as_millis());
            res * b.id
        })
        .sum::<usize>()
}

fn problem2(input: Input) -> usize {
    input
        .iter()
        .take(3)
        .map(|b| {
            let start = Instant::now();
            let res = solve(b, &32, 1, 1, 0, 0, 0, 0, 0, 0, 0, &mut 0);
            let duration = start.elapsed();
            println!("Blueprint {}: {} in time: {}ms", b.id, res, duration.as_millis());
            res
        })
        .fold(1, |acc, val| acc * val)
}

fn solve(input: &Blueprint, stop_at: &usize, minute: usize, ore_robots: usize, clay_robots: usize, obsidian_robots: usize, geode_robots: usize, ore: usize, clay: usize, obsidian: usize, geode: usize, record_geode: &mut usize) -> usize {
    //Produce
    let new_ore = ore + ore_robots;
    let new_clay = clay + clay_robots;
    let new_obsidian = obsidian + obsidian_robots;
    let new_geode = geode + geode_robots;

    if minute == *stop_at {
        return new_geode;
    }

    let remaining_minutes = *stop_at - minute;
    let can_produce = remaining_minutes * (remaining_minutes + 1) / 2 + remaining_minutes * geode_robots;
    if *record_geode > new_geode + can_produce {
        return 0;
    }

    let mut best = new_geode;

    if input.geode_robot_cost_ore <= ore && input.geode_robot_cost_obsidian <= obsidian {
        let result = solve(input, stop_at, minute + 1, ore_robots, clay_robots, obsidian_robots, geode_robots + 1, new_ore - input.geode_robot_cost_ore, new_clay, new_obsidian - input.geode_robot_cost_obsidian, new_geode, record_geode);
        if result > best {
            best = result;
        }
    }
    if input.obsidian_robot_cost_ore <= ore && input.obsidian_robot_cost_clay <= clay {
        let result = solve(input, stop_at, minute + 1, ore_robots, clay_robots, obsidian_robots + 1, geode_robots, new_ore - input.obsidian_robot_cost_ore, new_clay - input.obsidian_robot_cost_clay, new_obsidian, new_geode, record_geode);
        if result > best {
            best = result;
        }
    }
    if input.clay_robot_cost_ore <= ore {
        let result = solve(input, stop_at, minute + 1, ore_robots, clay_robots + 1, obsidian_robots, geode_robots, new_ore - input.clay_robot_cost_ore, new_clay, new_obsidian, new_geode, record_geode);
        if result > best {
            best = result;
        }
    }
    if input.ore_robot_cost_ore <= ore {
        let result = solve(input, stop_at, minute + 1, ore_robots + 1, clay_robots, obsidian_robots, geode_robots, new_ore - input.ore_robot_cost_ore, new_clay, new_obsidian, new_geode, record_geode);
        if result > best {
            best = result;
        }
    }

    let result = solve(input, stop_at, minute + 1, ore_robots, clay_robots, obsidian_robots, geode_robots, new_ore, new_clay, new_obsidian, new_geode, record_geode);
    if result > best {
        best = result;
    }

    if best > *record_geode {
        *record_geode = best;
        println!(" Found new record: {}", record_geode)
    }

    best
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct Blueprint {
    id: usize,
    ore_robot_cost_ore: usize,
    clay_robot_cost_ore: usize,
    obsidian_robot_cost_ore: usize,
    obsidian_robot_cost_clay: usize,
    geode_robot_cost_ore: usize,
    geode_robot_cost_obsidian: usize,
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    #[test]
    fn test_problems_1() {
        assert_eq!(33, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(1382, problem1(parse(include_str!("puzzle.txt"))));
    }

    #[test]
    fn test_problems_2() {
        assert_eq!(3472, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(31740, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse(s: &str) -> Input {
        s.lines()
            .map(|line| {
                let numbers = util::parse_numbers(line);

                Blueprint {
                    id: numbers[0] as usize,
                    ore_robot_cost_ore: numbers[1] as usize,
                    clay_robot_cost_ore: numbers[2] as usize,
                    obsidian_robot_cost_ore: numbers[3] as usize,
                    obsidian_robot_cost_clay: numbers[4] as usize,
                    geode_robot_cost_ore: numbers[5] as usize,
                    geode_robot_cost_obsidian: numbers[6] as usize,
                }
            })
            .collect()
    }
}
