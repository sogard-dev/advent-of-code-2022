use std::collections::{HashMap, HashSet, VecDeque};

type Input = HashSet<(isize, isize)>;

pub fn main() {
    println!("23");
}

fn print(elfs: &Input) {
    let min_row = elfs.iter().min_by_key(|e| e.0).unwrap().0 - 1;
    let max_row = elfs.iter().max_by_key(|e| e.0).unwrap().0 + 1;
    let min_column = elfs.iter().min_by_key(|e| e.1).unwrap().1 - 1;
    let max_column = elfs.iter().max_by_key(|e| e.1).unwrap().1 + 1;

    for r in min_row..=max_row {
        for c in min_column..=max_column {
            print!("{}", if elfs.contains(&(r, c)) { "#" } else { "." });
        }
        println!("");
    }
    println!("");
}

fn problem1(mut elfs: Input) -> usize {
    let mut directions = VecDeque::new();
    directions.push_back(Direction::N);
    directions.push_back(Direction::S);
    directions.push_back(Direction::W);
    directions.push_back(Direction::E);

    for round in 1..=10 {
        let mut wish_to_go = HashMap::new();
        let mut position_count = HashMap::new();

        for elf in elfs.iter() {
            if let Some(new_pos) = want_to_go(elf, &directions, &elfs) {
                wish_to_go.insert(new_pos, *elf);
                *position_count.entry(new_pos).or_insert_with(|| 0) += 1
            }
        }

        for (k, v) in position_count {
            if v == 1 {
                elfs.remove(&wish_to_go.get(&k).unwrap());
                elfs.insert(k);
            }
        }

        println!("After round {}", round);
        print(&elfs);

        let dir = directions.pop_front().unwrap();
        directions.push_back(dir);
    }

    let min_row = elfs.iter().min_by_key(|e| e.0).unwrap().0;
    let max_row = elfs.iter().max_by_key(|e| e.0).unwrap().0;
    let min_column = elfs.iter().min_by_key(|e| e.1).unwrap().1;
    let max_column = elfs.iter().max_by_key(|e| e.1).unwrap().1;

    let rows = 1 + (max_row - min_row) as usize;
    let columns = 1 + (max_column - min_column) as usize;
    rows * columns - elfs.len()
}

fn want_to_go(elf: &(isize, isize), directions: &VecDeque<Direction>, elfs: &Input) -> Option<(isize, isize)> {
    let has_company = elfs.contains(&(elf.0 - 1, elf.1 - 1)) || elfs.contains(&(elf.0 - 1, elf.1)) || elfs.contains(&(elf.0 - 1, elf.1 + 1)) || elfs.contains(&(elf.0 + 1, elf.1 - 1)) || elfs.contains(&(elf.0 + 1, elf.1)) || elfs.contains(&(elf.0 + 1, elf.1 + 1)) || elfs.contains(&(elf.0 - 1, elf.1 - 1)) || elfs.contains(&(elf.0, elf.1 - 1)) || elfs.contains(&(elf.0 + 1, elf.1 - 1)) || elfs.contains(&(elf.0 - 1, elf.1 + 1)) || elfs.contains(&(elf.0, elf.1 + 1)) || elfs.contains(&(elf.0 + 1, elf.1 + 1));
    if !has_company {
        return None;
    }

    for direction in directions.iter() {
        let should_not_go = match direction {
            Direction::N => elfs.contains(&(elf.0 - 1, elf.1 - 1)) || elfs.contains(&(elf.0 - 1, elf.1)) || elfs.contains(&(elf.0 - 1, elf.1 + 1)),
            Direction::S => elfs.contains(&(elf.0 + 1, elf.1 - 1)) || elfs.contains(&(elf.0 + 1, elf.1)) || elfs.contains(&(elf.0 + 1, elf.1 + 1)),
            Direction::W => elfs.contains(&(elf.0 - 1, elf.1 - 1)) || elfs.contains(&(elf.0, elf.1 - 1)) || elfs.contains(&(elf.0 + 1, elf.1 - 1)),
            Direction::E => elfs.contains(&(elf.0 - 1, elf.1 + 1)) || elfs.contains(&(elf.0, elf.1 + 1)) || elfs.contains(&(elf.0 + 1, elf.1 + 1)),
            _ => panic!("Should not be"),
        };

        if !should_not_go {
            return Some(match direction {
                Direction::N => (elf.0 - 1, elf.1),
                Direction::S => (elf.0 + 1, elf.1),
                Direction::W => (elf.0, elf.1 - 1),
                Direction::E => (elf.0, elf.1 + 1),
                _ => panic!("nope"),
            });
        }
    }
    None
}

fn problem2(mut elfs: Input) -> usize {
    let mut directions = VecDeque::new();
    directions.push_back(Direction::N);
    directions.push_back(Direction::S);
    directions.push_back(Direction::W);
    directions.push_back(Direction::E);

    for round in 1..=1000000 {
        let mut wish_to_go = HashMap::new();
        let mut position_count = HashMap::new();

        for elf in elfs.iter() {
            if let Some(new_pos) = want_to_go(elf, &directions, &elfs) {
                wish_to_go.insert(new_pos, *elf);
                *position_count.entry(new_pos).or_insert_with(|| 0) += 1
            }
        }

        let mut moved = false;
        for (k, v) in position_count {
            if v == 1 {
                elfs.remove(&wish_to_go.get(&k).unwrap());
                elfs.insert(k);
                moved = true;
            }
        }

        if !moved {
            return round;
        }

        //println!("After round {}", round);
        //print(&elfs);

        let dir = directions.pop_front().unwrap();
        directions.push_back(dir);
    }
    panic!("should not happen");
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
    SW,
    SE,
    NW,
    NE,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problems_1() {
        assert_eq!(110, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(4208, problem1(parse(include_str!("puzzle.txt"))));
    }

    #[test]
    fn test_problems_2() {
        assert_eq!(20, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(0, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse(s: &str) -> Input {
        let mut elves = HashSet::new();
        s.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(column, c)| {
                match c {
                    '#' => elves.insert((row as isize, column as isize)),
                    _ => true,
                };
                ()
            })
        });

        elves
    }
}
