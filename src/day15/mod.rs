use crate::{
    grid::Coordinate,
    util::{Interval, Intervals},
};

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

fn problem1(row: isize, input: Input) -> isize {
    let mut intervals = Intervals::new();

    for line in input.iter() {
        let sensor_pos = (line.sensor_x, line.sensor_y);
        let beacon_pos = (line.beacon_x, line.beacon_y);
        let sensor_to_beacon = manhatten_distance(beacon_pos, sensor_pos);
        let sensor_to_row = manhatten_distance((sensor_pos.0, row), sensor_pos);

        if sensor_to_row < sensor_to_beacon {
            let diff = sensor_to_beacon - sensor_to_row;
            intervals.add(Interval::new(sensor_pos.0 - diff, sensor_pos.0 + diff));
        }
    }

    intervals.vec().iter().fold(0, |acc, interval| acc + (interval.end - interval.start))
}

fn problem2(max: usize, input: Input) -> isize {
    let mut intervals = vec![];
    for _ in 0..=max {
        let mut this_intervals = Intervals::new();
        this_intervals.add(Interval::new(0, max as isize));
        intervals.push(this_intervals);
    }

    for line in input.iter() {
        let sensor_pos = (line.sensor_x, line.sensor_y);
        let beacon_pos = (line.beacon_x, line.beacon_y);
        let sensor_to_beacon = manhatten_distance(beacon_pos, sensor_pos);

        let from = (sensor_pos.1 - sensor_to_beacon).max(0);
        let to = (sensor_pos.1 + sensor_to_beacon).min(max as isize);

        for y in from..=to {
            let distance = (sensor_pos.1 - y).abs();
            let width = sensor_to_beacon - distance;
            let cover_start = sensor_pos.0 - width;
            let cover_end = sensor_pos.0 + width;

            intervals[y as usize].remove(Interval::new(cover_start, cover_end));
        }
    }
    for y in 0..=max {
        if let Some(interval) = intervals[y].get_end() {
            return interval.start * 4000000 + y as isize;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

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

    fn parse(s: &str) -> Input {
        s.lines()
            .map(|line| {
                let numbers = util::parse_numbers(line);
                Line { sensor_x: numbers[0], sensor_y: numbers[1], beacon_x: numbers[2], beacon_y: numbers[3] }
            })
            .collect()
    }
}
