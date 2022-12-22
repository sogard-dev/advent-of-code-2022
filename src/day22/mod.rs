use std::collections::HashMap;

type Input = Map;

pub fn main() {
    println!("22");
}

fn problem1(map: Input) -> usize {
    let min_row = map.fields.iter().min_by_key(|entry| entry.0 .0).unwrap().0 .0;
    let max_row = map.fields.iter().max_by_key(|entry| entry.0 .0).unwrap().0 .0;
    let min_column = map.fields.iter().min_by_key(|entry| entry.0 .1).unwrap().0 .1;
    let max_column = map.fields.iter().max_by_key(|entry| entry.0 .1).unwrap().0 .1;

    let start = get_first_from_left(&map, &0, &min_column, &max_column);
    let mut direction = Direction::Right;
    let mut position = start;

    print(&min_row, &max_row, &min_column, &max_column, &position, &map);

    for ins in &map.instructions {
        match ins {
            Instruction::TurnLeft => {
                direction = match direction {
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                };
                //print(&min_row, &max_row, &min_column, &max_column, &position, &map);
            }
            Instruction::TurnRight => {
                direction = match direction {
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                };
                //print(&min_row, &max_row, &min_column, &max_column, &position, &map);
            }
            Instruction::Forward(steps) => {
                for _ in 0..*steps {
                    position = get_next(&position, &direction, &map, &min_row, &max_row, &min_column, &max_column);
                    //print(&min_row, &max_row, &min_column, &max_column, &position, &map);
                }
            }
        }
    }

    let r = (position.0 + 1);
    let c = (position.1 + 1);

    1000 * r
        + 4 * c
        + match direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
}

fn print(min_row: &usize, max_row: &usize, min_column: &usize, max_column: &usize, position: &(usize, usize), map: &Input) {
    for row in *min_row..=*max_row {
        for column in *min_column..=*max_column {
            if (row, column).eq(&position) {
                print!("H");
            } else {
                match map.fields.get(&(row, column)) {
                    Some(entry) => print!(
                        "{}",
                        match entry {
                            Entry::Walkable => ".",
                            Entry::Blocked => "#",
                        }
                    ),
                    None => print!(" "),
                }
            }
        }
        println!("");
    }
    println!("");
}

fn get_next(position: &(usize, usize), direction: &Direction, map: &Map, min_row: &usize, max_row: &usize, min_column: &usize, max_column: &usize) -> (usize, usize) {
    let is_boundary = match direction {
        Direction::Left => position.1 == *min_column,
        Direction::Right => position.1 == *max_column,
        Direction::Up => position.0 == *min_row,
        Direction::Down => position.0 == *max_column,
    };

    if !is_boundary {
        let next_to_check = match direction {
            Direction::Left => (position.0, position.1 - 1),
            Direction::Right => (position.0, position.1 + 1),
            Direction::Up => (position.0 - 1, position.1),
            Direction::Down => (position.0 + 1, position.1),
        };

        if let Some(x) = map.fields.get(&next_to_check) {
            if x.eq(&Entry::Walkable) {
                return next_to_check;
            }
            if x.eq(&Entry::Blocked) {
                return *position;
            }
        }
    }

    let rolled_to_check = match direction {
        Direction::Right => get_first_from_left(&map, &position.0, min_column, max_column),
        Direction::Left => get_first_from_right(&map, &position.0, min_column, max_column),
        Direction::Down => get_first_from_top(&map, &position.1, min_row, max_row),
        Direction::Up => get_first_from_bottom(&map, &position.1, min_row, max_row),
    };

    if let Some(x) = map.fields.get(&rolled_to_check) {
        if x.eq(&Entry::Walkable) {
            return rolled_to_check;
        }
        if x.eq(&Entry::Blocked) {
            return *position;
        }
    }

    panic!("Should not go here");
}

fn get_first_from_left(map: &Input, row: &usize, min_column: &usize, max_column: &usize) -> (usize, usize) {
    for i in *min_column..=*max_column {
        if map.fields.get(&(*row, i)).is_some() {
            return (*row, i);
        }
    }
    panic!("Shit");
}

fn get_first_from_right(map: &Input, row: &usize, min_column: &usize, max_column: &usize) -> (usize, usize) {
    for i in (*min_column..=*max_column).rev() {
        if map.fields.get(&(*row, i)).is_some() {
            return (*row, i);
        }
    }
    panic!("Shit");
}

fn get_first_from_top(map: &Input, column: &usize, min_row: &usize, max_row: &usize) -> (usize, usize) {
    for i in *min_row..=*max_row {
        if map.fields.get(&(i, *column)).is_some() {
            return (i, *column);
        }
    }
    panic!("Shit");
}

fn get_first_from_bottom(map: &Input, column: &usize, min_row: &usize, max_row: &usize) -> (usize, usize) {
    for i in (*min_row..=*max_row).rev() {
        if map.fields.get(&(i, *column)).is_some() {
            return (i, *column);
        }
    }
    panic!("Shit");
}

fn problem2(input: Input) -> isize {
    0
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum Instruction {
    TurnLeft,
    TurnRight,
    Forward(usize),
}

struct Map {
    instructions: Vec<Instruction>,
    fields: HashMap<(usize, usize), Entry>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum Entry {
    Walkable,
    Blocked,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use regex::Regex;

    use crate::util;

    use super::*;

    #[test]
    fn test_problems_1() {
        assert_eq!(6032, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(66292, problem1(parse(include_str!("puzzle.txt"))));
    }

    #[test]
    fn test_problems_2() {
        assert_eq!(301, problem2(parse(include_str!("test_puzzle.txt"))));
        //assert_eq!(3916936880448, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse(s: &str) -> Input {
        let mut fields = HashMap::new();
        let parts: Vec<&str> = s.split("\r\n\r\n").collect();
        parts[0].lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(column, char)| {
                match char {
                    '#' => fields.insert((row, column), Entry::Blocked),
                    '.' => fields.insert((row, column), Entry::Walkable),
                    _ => None,
                };
            })
        });

        let mut instructions = vec![];

        let re = Regex::new(r"([\d]+)+|([RL])+").unwrap();
        for capture in re.captures_iter(&s) {
            let numbers = util::parse_numbers(&capture[0]);
            if numbers.len() > 0 {
                instructions.push(Instruction::Forward(numbers[0] as usize));
            } else {
                match &capture[0] {
                    "R" => instructions.push(Instruction::TurnRight),
                    "L" => instructions.push(Instruction::TurnLeft),
                    _ => panic!("Unknown"),
                }
            }
        }

        Map { instructions, fields }
    }
}
