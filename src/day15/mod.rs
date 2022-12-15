use std::collections::HashSet;

use crate::grid::Coordinate;

type Input = Vec<Line>;

#[derive(Debug, PartialEq, Clone, Eq)]
struct Line {
    sensor_x: isize,
    sensor_y: isize,
    beacon_x: isize,
    beacon_y: isize,
}

pub fn main() {
    println!("Day15");
}

fn manhatten_distance(a: Coordinate, b: Coordinate) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn problem1(row: isize, mut input: Input) -> usize {
    let mut covered_in_row = HashSet::new();

    for line in input.iter() {
        let sensor_pos = (line.sensor_x, line.sensor_y);
        let beacon_pos = (line.beacon_x, line.beacon_y);
        let distance_to_beacon = manhatten_distance(beacon_pos, sensor_pos);
        // println!("Sensor: {:?} with beacon {:?} and distance: {}", sensor_pos, beacon_pos, distance_to_beacon);

        for diff in -distance_to_beacon..=distance_to_beacon {
            let to_check = (line.sensor_x + diff, row);

            let dist = manhatten_distance(sensor_pos, to_check);
            // println!("  Checking: {:?} with distance: {}", to_check, dist);

            if dist <= distance_to_beacon {
                // println!("    Adding: {:?} with distance: {}", to_check, dist);

                covered_in_row.insert(to_check);
            }
        }
    }

    for line in input.iter() {
        covered_in_row.remove(&(line.beacon_x, line.beacon_y));
    }

    covered_in_row.len()
}

fn problem2(max: isize, mut input: Input) -> isize {
    let mut y = 0;
    let mut x = 0;

    while y <= max {
        let mut new_x = x;
        for line in input.iter() {
            let sensor_dist = manhatten_distance((line.beacon_x, line.beacon_y), (line.sensor_x, line.sensor_y));
            let sensor_dist_to_this = manhatten_distance((x,y), (line.sensor_x, line.sensor_y));

            if sensor_dist >= sensor_dist_to_this{
                let diff = (sensor_dist_to_this - sensor_dist).abs();
                new_x = x + diff + 1;
                // println!("  Was close to {:?}, Sensor dist: {}, Distance: {}, Difference: {}, Jumping to: {}", (line.sensor_x, line.sensor_y), sensor_dist, sensor_dist_to_this, diff, new_x);

                break;
            }
        }

        if new_x == x {
            return x * 4000000 + y;
        }

        x = new_x;
        if x >= max {
            x = 0;
            y += 1;

            println!("Checking {:?}", (x,y));

        }
    }

    0
}

#[cfg(test)]
mod tests {

    use super::*;
    use regex::Regex;

    #[test]
    fn test_problems_1() {
        assert_eq!(26, problem1(10, parse(include_str!("test_puzzle.txt"))));
        assert_eq!(4737567, problem1(2000000, parse(include_str!("puzzle.txt"))));
    }

    #[test]
    fn test_problems_2() {
        assert_eq!(56000011, problem2(20, parse(include_str!("test_puzzle.txt"))));
        assert_eq!(13267474686239, problem2(4000000, parse(include_str!("puzzle.txt"))));
    }

    fn get_numbers(s: &str) -> Vec<isize> {
        let mut vec = Vec::new();
        let re = Regex::new(r"([-\d]+)").unwrap();
        for capture in re.captures_iter(&s) {
            for i in 1..capture.len() {
                if let Ok(num) = capture[i].parse::<isize>() {
                    vec.push(num);
                }
            }
        }

        vec
    }

    fn parse(s: &str) -> Input {
        s.lines()
            .map(|line| {
                let numbers = get_numbers(line);
                Line { sensor_x: numbers[0], sensor_y: numbers[1], beacon_x: numbers[2], beacon_y: numbers[3] }
            })
            .collect()
    }
}
