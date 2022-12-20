use std::collections::{HashMap, HashSet};

type Input = Vec<Cube>;

pub fn main() {
    println!("18");
}

fn problem1(input: Input) -> usize {
    let mut sides = HashMap::new();
    for cube in input {
        *sides.entry((0, cube.x, cube.y - 1, cube.z - 1)).or_insert_with(|| 0) += 1;
        *sides.entry((0, cube.x - 1, cube.y - 1, cube.z - 1)).or_insert_with(|| 0) += 1;

        *sides.entry((1, cube.x - 1, cube.y, cube.z - 1)).or_insert_with(|| 0) += 1;
        *sides.entry((1, cube.x - 1, cube.y - 1, cube.z - 1)).or_insert_with(|| 0) += 1;

        *sides.entry((2, cube.x - 1, cube.y - 1, cube.z)).or_insert_with(|| 0) += 1;
        *sides.entry((2, cube.x - 1, cube.y - 1, cube.z - 1)).or_insert_with(|| 0) += 1;
    }
    sides.values().filter(|e| **e == 1).count()
}

fn problem2(input: Input) -> usize {
    let mut sides = HashMap::new();
    for cube in input.iter() {
        *sides.entry((0, cube.x, cube.y - 1, cube.z - 1)).or_insert_with(|| 0) += 1;
        *sides.entry((0, cube.x - 1, cube.y - 1, cube.z - 1)).or_insert_with(|| 0) += 1;

        *sides.entry((1, cube.x - 1, cube.y, cube.z - 1)).or_insert_with(|| 0) += 1;
        *sides.entry((1, cube.x - 1, cube.y - 1, cube.z - 1)).or_insert_with(|| 0) += 1;

        *sides.entry((2, cube.x - 1, cube.y - 1, cube.z)).or_insert_with(|| 0) += 1;
        *sides.entry((2, cube.x - 1, cube.y - 1, cube.z - 1)).or_insert_with(|| 0) += 1;
    }

    let min_x = input.iter().map(|cube| cube.x).min().unwrap();
    let max_x = input.iter().map(|cube| cube.x).max().unwrap();
    let min_y = input.iter().map(|cube| cube.y).min().unwrap();
    let max_y = input.iter().map(|cube| cube.y).max().unwrap();
    let min_z = input.iter().map(|cube| cube.z).min().unwrap();
    let max_z = input.iter().map(|cube| cube.z).max().unwrap();

    let mut touching_air = HashSet::new();

    touching_air.insert((min_x - 1, min_y - 1, min_z - 1));
    touching_air.insert((min_x - 1, min_y - 1, max_z + 1));
    touching_air.insert((min_x - 1, max_y + 1, min_z - 1));
    touching_air.insert((min_x - 1, max_y + 1, max_z + 1));

    let mut changed = true;
    while changed {
        changed = false;

        for x in min_x - 1..=max_x + 1 {
            for y in min_y - 1..=max_y + 1 {
                for z in min_z - 1..=max_z + 1 {
                    if !touching_air.contains(&(x, y, z)) {
                        if !input.contains(&Cube { x, y, z }) {
                            for (dx, dy, dz) in [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)] {
                                let neighbour = (x + dx, y + dy, z + dz);
                                if touching_air.contains(&neighbour) {
                                    touching_air.insert((x, y, z));
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            for z in min_z - 1..=max_z + 1 {
                if !touching_air.contains(&(x, y, z)) && !input.contains(&Cube { x, y, z }) {
                    sides.remove(&(0, x, y - 1, z - 1));
                    sides.remove(&(0, x - 1, y - 1, z - 1));

                    sides.remove(&(1, x - 1, y, z - 1));
                    sides.remove(&(1, x - 1, y - 1, z - 1));

                    sides.remove(&(2, x - 1, y - 1, z));
                    sides.remove(&(2, x - 1, y - 1, z - 1));
                }
            }
        }
    }

    sides.values().filter(|e| **e == 1).count()
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    #[test]
    fn test_problems_1() {
        assert_eq!(10, problem1(parse(include_str!("test_puzzle_1.txt"))));
        assert_eq!(64, problem1(parse(include_str!("test_puzzle_2.txt"))));
        assert_eq!(3662, problem1(parse(include_str!("puzzle.txt"))));
    }

    #[test]
    fn test_problems_2() {
        assert_eq!(58, problem2(parse(include_str!("test_puzzle_2.txt"))));
        assert_eq!(2060, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse(s: &str) -> Input {
        s.lines()
            .map(|line| {
                let numbers = util::parse_numbers(line);
                Cube { x: numbers[0], y: numbers[1], z: numbers[2] }
            })
            .collect()
    }
}
