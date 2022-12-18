use std::collections::{HashMap, HashSet};

use crate::grid::{Coordinate, Grid};

type Input = Grid<Valve>;

pub fn main() {
    println!("Day16");
}

fn problem1(grid: Input) -> usize {
    let mut distances = HashMap::new();
    let start = grid.find(|_, model| model.name.eq("AA")).unwrap();

    grid.for_every(|pos, model| {
        if model.rate > 0 || pos.eq(&start) {
            grid.bfs(pos, |other_pos, distance| {
                if distance > 0 && grid.get_model(other_pos).unwrap().rate > 0 {
                    distances.insert((*pos, *other_pos), distance as usize);
                }
            });
        }
    });

    def_1(&grid, 1, 0, &distances, &mut HashSet::new(), &start, 0)
}

fn def_1(grid: &Input, minute: usize, pressure: usize, distances: &HashMap<(Coordinate, Coordinate), usize>, visited: &mut HashSet<Coordinate>, my_position: &Coordinate, pressure_per_minute: usize) -> usize {
    if minute > 30 {
        return pressure;
    }

    visited.insert(*my_position);

    let to_visit: Vec<(Coordinate, usize)> = distances.iter().filter(|e| e.0 .0 == *my_position && !visited.contains(&e.0 .1)).filter(|e| e.1 + minute < 30).map(|e| (e.0 .1, *e.1)).collect();

    let mut result = pressure;

    for (other_pos, distance) in to_visit {
        let minute_of_opening = minute + distance;
        let new_pressure = (1 + distance) * pressure_per_minute + pressure;

        let rate = grid.get_model(&other_pos).unwrap().rate;
        let new_result = def_1(grid, minute_of_opening + 1, new_pressure, distances, visited, &other_pos, pressure_per_minute + rate);
        if new_result > result {
            if new_result >= 1820 {
                println!("Minute {}, opening {:?}", minute + distance, rate);
            }
            result = new_result;
        }
    }
    visited.remove(my_position);

    let minutes_left = 30 - minute + 1;
    let new_result = pressure + minutes_left * pressure_per_minute;
    if new_result > result {
        if new_result >= 1820 {
            println!("Do nothing at minute {}, releasing {:?}", minute, minutes_left * pressure_per_minute);
        }
        result = new_result;
    }

    result
}

fn problem2(grid: Input) -> usize {
    let mut distances = HashMap::new();
    let start = grid.find(|_, model| model.name.eq("AA")).unwrap();

    grid.for_every(|pos, model| {
        if model.rate > 0 || pos.eq(&start) {
            grid.bfs(pos, |other_pos, distance| {
                if distance > 0 && grid.get_model(other_pos).unwrap().rate > 0 {
                    distances.insert((*pos, *other_pos), distance as usize);
                }
            });
        }
    });

    def_2(&grid, 1, 0, &distances, &mut HashSet::new(), &mut HashSet::new(), &start, 0, &start, 0, 0, &mut 0)
}

fn def_2(grid: &Input, minute: usize, pressure: usize, distances: &HashMap<(Coordinate, Coordinate), usize>, reserved: &mut HashSet<Coordinate>, opened: &mut HashSet<Coordinate>, a_pos: &Coordinate, pressure_per_minute: usize, b_pos: &Coordinate, a_opening: usize, b_opening: usize, max_seen: &mut usize) -> usize {
    let rounds = 26;

    if minute > rounds {
        return pressure;
    }

    if pressure + 218 * (rounds - minute + 1) < *max_seen {
        return 0;
    }

    if a_pos == b_pos && minute > 1 {
        panic!("They should not");
    }

    let mut this_minute = minute;
    let mut new_pressure = pressure + pressure_per_minute;

    while a_opening > this_minute && b_opening > this_minute && this_minute < rounds {
        this_minute += 1;
        new_pressure += pressure_per_minute;
    }

    let next_minute = this_minute + 1;
    let mut best_result = new_pressure;
    let mut new_pressure_per_minute = pressure_per_minute;

    if new_pressure > *max_seen {
        *max_seen = new_pressure;
        println!("New max seen: {}", max_seen);
    }

    reserved.insert(*a_pos);
    reserved.insert(*b_pos);

    let mut a_opened = false;
    if a_opening == this_minute && !opened.contains(a_pos) {
        opened.insert(*a_pos);
        let rate = grid.get_model(&a_pos).unwrap().rate;
        new_pressure_per_minute += rate;
        //println!("Human opening {:?} at minute {}", a_pos, this_minute);
        //println!("  Release: {}", new_pressure_per_minute);
        a_opened = true;
    }

    let mut b_opened = false;
    if b_opening == this_minute && !opened.contains(b_pos) {
        opened.insert(*b_pos);
        let rate = grid.get_model(&b_pos).unwrap().rate;
        new_pressure_per_minute += rate;
        //println!("Elaphant opening {:?} at minute {}", b_pos, this_minute);
        //println!("  Release: {}", new_pressure_per_minute);
        b_opened = true;
    }

    let mut to_visit_singles = Vec::new();

    let mut a_will_visit = false;
    if a_opening < this_minute {
        let a_to_visit: Vec<(Coordinate, usize)> = distances.iter().filter(|e| e.0 .0 == *a_pos && !reserved.contains(&e.0 .1)).filter(|e| e.1 + this_minute < rounds).map(|e| (e.0 .1, *e.1)).collect();
        for (other_pos, distance) in a_to_visit {
            to_visit_singles.push((Some((other_pos, distance)), None));
            a_will_visit = true;
        }
    }

    let mut b_will_visit = false;
    if b_opening < this_minute {
        let b_to_visit: Vec<(Coordinate, usize)> = distances.iter().filter(|e| e.0 .0 == *b_pos && !reserved.contains(&e.0 .1)).filter(|e| e.1 + this_minute < rounds).map(|e| (e.0 .1, *e.1)).collect();
        for (other_pos, distance) in b_to_visit {
            to_visit_singles.push((None, Some((other_pos, distance))));
            b_will_visit = true;
        }
    }

    let mut to_visit = Vec::new();
    for (a1, b1) in to_visit_singles.iter() {
        if a_will_visit != b_will_visit {
            to_visit.push((a1, b1));
        }

        for (a2, b2) in to_visit_singles.iter() {
            if a1.is_some() && b2.is_some() {
                if !a1.unwrap().0.eq(&b2.unwrap().0) {
                    to_visit.push((a1, b2));
                }
            }
            if a2.is_some() && b1.is_some() {
                if !a2.unwrap().0.eq(&b1.unwrap().0) {
                    to_visit.push((a2, b1));
                }
            }
        }
    }

    for (a, b) in to_visit {
        let mut new_a_pos = a_pos;
        let mut new_a_can_move = a_opening;
        let mut new_b_pos = b_pos;
        let mut new_b_can_move = b_opening;

        if let Some((pos, distance)) = a {
            new_a_pos = pos;
            new_a_can_move = this_minute + distance;
        }

        if let Some((pos, distance)) = b {
            new_b_pos = pos;
            new_b_can_move = this_minute + distance;
        }

        if new_a_pos != new_b_pos {
            let new_result = def_2(grid, next_minute, new_pressure, distances, reserved, opened, new_a_pos, new_pressure_per_minute, new_b_pos, new_a_can_move, new_b_can_move, max_seen);
            if new_result > best_result {
                best_result = new_result;
            }
        }
    }

    if best_result == new_pressure {
        let new_result = def_2(grid, next_minute, new_pressure, distances, reserved, opened, a_pos, new_pressure_per_minute, b_pos, a_opening, b_opening, max_seen);
        if new_result > best_result {
            best_result = new_result;
        }
    }

    if a_opened {
        opened.remove(&a_pos);
        //println!("Human close {:?} at {}", a_pos, this_minute);
    }

    if b_opened {
        opened.remove(&b_pos);
        //println!("Elephant close {:?} at {}", b_pos, this_minute);
    }

    reserved.remove(&a_pos);
    reserved.remove(&b_pos);

    best_result
}

#[derive(Debug, PartialEq, Clone)]
struct Valve {
    rate: usize,
    name: String,
    potential: usize,
    closed: bool,
}

#[cfg(test)]
mod tests {

    use crate::util;

    use super::*;

    #[test]
    fn test_problem_1() {
        assert_eq!(1651, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(1947, problem1(parse(include_str!("puzzle.txt"))));
    }

    #[test]
    fn test_problem_2() {
        assert_eq!(1707, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(15, problem2(parse(include_str!("puzzle.txt"))));
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
                let other = spl[i].replace(",", "");
                //println!("{} leads to {}", spl[1], other);
                let to = valve_to_position.get(&other).unwrap();
                grid.add_connections(vec![(*from, *to)]);
            }
        });

        grid
    }
}
