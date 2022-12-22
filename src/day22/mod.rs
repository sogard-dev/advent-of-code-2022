use std::collections::{HashMap, HashSet};

type Input = Pair;

pub fn main() {
    println!("22");
}

fn problem1(input: Input) -> isize {
    0
}

fn problem2(mut input: Input) -> isize {
    0
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum Instruction {
    TurnLeft,
    TurnRight,
    Forward(usize),
}

struct Pair {
    instructions: Vec<Instruction>,
    walkable: HashSet<(usize, usize)>,
    blocked: HashSet<(usize, usize)>,
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::util;

    use super::*;

    #[test]
    fn test_problems_1() {
        assert_eq!(152, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(41857219607906, problem1(parse(include_str!("puzzle.txt"))));
    }

    #[test]
    fn test_problems_2() {
        assert_eq!(301, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(3916936880448, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse(s: &str) -> Input {
        let mut walkable = HashSet::new();
        let mut blocked = HashSet::new();
        let parts: Vec<&str> = s.split("\r\n\r\n").collect();
        parts[0].lines().enumerate().for_each(|(row, line)| line.chars().enumerate().for_each(|(column, char)| {
            match char {
                '#' => blocked.insert((row,column)),
                '.' => walkable.insert((row,column)),
                _ => false
            };
        }));

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
                    _ => panic!("Unknown")
                }
            }
        }

        Pair {
            instructions,
            walkable,
            blocked
        }
    }
}
