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

    let start = get_first_from_left(&map, &min_row, &min_column, &max_column);
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

    let r = position.0;
    let c = position.1;

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

#[derive(Debug, PartialEq, Clone, Eq)]
enum Side {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Side {
    fn get_offset_row(&self) -> usize {
        match self {
            Side::One => 0,
            Side::Two => 0,
            Side::Three => 50,
            Side::Four => 100,
            Side::Five => 100,
            Side::Six => 150,
        }
    }

    fn get_offset_column(&self) -> usize {
        match self {
            Side::One => 100,
            Side::Two => 50,
            Side::Three => 50,
            Side::Four => 50,
            Side::Five => 0,
            Side::Six => 0,
        }
    }
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

fn problem2(map: Input) -> usize {
    let min_row = map.fields.iter().min_by_key(|entry| entry.0 .0).unwrap().0 .0;
    let max_row = map.fields.iter().max_by_key(|entry| entry.0 .0).unwrap().0 .0;
    let min_column = map.fields.iter().min_by_key(|entry| entry.0 .1).unwrap().0 .1;
    let max_column = map.fields.iter().max_by_key(|entry| entry.0 .1).unwrap().0 .1;

    let start = get_first_from_left(&map, &min_row, &min_column, &max_column);
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
                    let (d, pos) = get_next_2(&position, &direction, &map);
                    direction = d;
                    position = pos;
                    //print(&min_row, &max_row, &min_column, &max_column, &position, &map);
                }
            }
        }
    }

    let r = position.0;
    let c = position.1;

    1000 * r
        + 4 * c
        + match direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
}

fn get_next_2(position: &(usize, usize), direction: &Direction, map: &Map) -> (Direction, (usize, usize)) {
    let next_to_check = match direction {
        Direction::Left => (position.0, position.1 - 1),
        Direction::Right => (position.0, position.1 + 1),
        Direction::Up => (position.0 - 1, position.1),
        Direction::Down => (position.0 + 1, position.1),
    };

    let current_side = get_side(position);
    let continue_side = get_side(&next_to_check);
    println!("Current  Position: {:?}, side: {:?}", position, current_side);
    println!("Checking Position: {:?}, side: {:?}", next_to_check, continue_side);

    if current_side == get_side(&next_to_check) {
        if let Some(x) = map.fields.get(&next_to_check) {
            if x.eq(&Entry::Walkable) {
                return (direction.clone(), next_to_check);
            } else {
                return (direction.clone(), *position);
            }
        }
    }

    let next_side = get_next_side(&current_side, direction);
    println!("  Going {:?}, we should roll to {:?}", direction, next_side);

    let (dir, rolled_to_check) = roll(&current_side, &next_side, position);

    println!("  New direction and position to check {:?},  {:?}", dir, rolled_to_check);
    if let Some(x) = map.fields.get(&rolled_to_check) {
        if x.eq(&Entry::Walkable) {
            return (dir, rolled_to_check);
        } else {
            return (direction.clone(), *position);
        }
    }

    panic!("Should not go here");
}

fn roll(current_side: &Side, next_side: &Side, position: &(usize, usize)) -> (Direction, (usize, usize)) {
    let side_relative_row = ((position.0 - 1) % 50) as isize;
    let side_relative_column = ((position.1 - 1) % 50) as isize;

    let relative = match (current_side, next_side) {
        (Side::Two, Side::Five) => (Direction::Right, (49 - side_relative_row, 0)),
        (Side::Five, Side::Two) => (Direction::Right, (49 - side_relative_row, 0)),

        (Side::One, Side::Four) => (Direction::Left, (49 - side_relative_row, 49)),
        (Side::Four, Side::One) => (Direction::Left, (49 - side_relative_row, 49)),

        (Side::One, Side::Two) => (Direction::Left, (side_relative_row, 49)),
        (Side::Four, Side::Five) => (Direction::Left, (side_relative_row, 49)),

        (Side::One, Side::Three) => (Direction::Left, (side_relative_column, 49)),
        (Side::Four, Side::Six) => (Direction::Left, (side_relative_column, 49)),

        (Side::Two, Side::One) => (Direction::Right, (side_relative_row, 0)),
        (Side::Five, Side::Four) => (Direction::Right, (side_relative_row, 0)),

        (Side::One, Side::Six) => (Direction::Up, (49, side_relative_column)),
        (Side::Three, Side::Two) => (Direction::Up, (49, side_relative_column)),
        (Side::Four, Side::Three) => (Direction::Up, (49, side_relative_column)),
        (Side::Six, Side::Five) => (Direction::Up, (49, side_relative_column)),
        
        (Side::Two, Side::Six) => (Direction::Right, (side_relative_column, 0)),
        (Side::Five, Side::Three) => (Direction::Right, (side_relative_column, 0)),

        (Side::Two, Side::Three) => (Direction::Down, (0, side_relative_column)),
        (Side::Three, Side::Four) => (Direction::Down, (0, side_relative_column)),
        (Side::Five, Side::Six) => (Direction::Down, (0, side_relative_column)),
        (Side::Six, Side::One) => (Direction::Down, (0, side_relative_column)),

        (Side::Three, Side::Five) => (Direction::Down, (0, side_relative_row)),
        (Side::Six, Side::Two) => (Direction::Down, (0, side_relative_row)),

        (Side::Three, Side::One) => (Direction::Up, (49, side_relative_row)),
        (Side::Six, Side::Four) => (Direction::Up, (49, side_relative_row)),
        (a, b) => panic!("{:?} should not roll onto {:?}", a, b),
    };

    let new_row = 1 + relative.1.0 + next_side.get_offset_row() as isize;
    let new_column = 1 + relative.1.1 + next_side.get_offset_column() as isize;

    (relative.0, (new_row as usize , new_column as usize))
}

fn get_next_side(side: &Side, direction: &Direction) -> Side {
    match (side, direction) {
        (Side::One, Direction::Left) => Side::Two,
        (Side::One, Direction::Up) => Side::Six,
        (Side::One, Direction::Right) => Side::Four,
        (Side::One, Direction::Down) => Side::Three,
        (Side::Two, Direction::Left) => Side::Five,
        (Side::Two, Direction::Up) => Side::Six,
        (Side::Two, Direction::Right) => Side::One,
        (Side::Two, Direction::Down) => Side::Three,
        (Side::Three, Direction::Left) => Side::Five,
        (Side::Three, Direction::Up) => Side::Two,
        (Side::Three, Direction::Right) => Side::One,
        (Side::Three, Direction::Down) => Side::Four,
        (Side::Four, Direction::Left) => Side::Five,
        (Side::Four, Direction::Up) => Side::Three,
        (Side::Four, Direction::Right) => Side::One,
        (Side::Four, Direction::Down) => Side::Six,
        (Side::Five, Direction::Left) => Side::Two,
        (Side::Five, Direction::Up) => Side::Three,
        (Side::Five, Direction::Right) => Side::Four,
        (Side::Five, Direction::Down) => Side::Six,
        (Side::Six, Direction::Left) => Side::Two,
        (Side::Six, Direction::Up) => Side::Five,
        (Side::Six, Direction::Right) => Side::Four,
        (Side::Six, Direction::Down) => Side::One,
    }
}

fn get_side(pos: &(usize, usize)) -> Side {
    let row = pos.0;
    let column = pos.1;

    if column > 100 {
        Side::One
    } else if row <= 50 {
        Side::Two
    } else if row <= 100 {
        Side::Three
    } else if column > 50 {
        Side::Four
    } else if row > 150 {
        Side::Six
    } else {
        Side::Five
    }
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
        //assert_eq!(301, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(127012, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse(s: &str) -> Input {
        let mut fields = HashMap::new();
        let parts: Vec<&str> = s.split("\r\n\r\n").collect();
        parts[0].lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(column, char)| {
                match char {
                    '#' => fields.insert((row + 1, column + 1), Entry::Blocked),
                    '.' => fields.insert((row + 1, column + 1), Entry::Walkable),
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
