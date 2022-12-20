use std::collections::VecDeque;

use itertools::Itertools;

type Input = VecDeque<isize>;

pub fn main() {
    println!("20");
}

fn print(input: &VecDeque<(usize, isize)>) {
    println!("{}", input.iter().map(|e| e.1).join(", "));
    println!("");
}

fn problem1(rounds: usize, input: Input) -> isize {
    let mut work: VecDeque<(usize, isize)> = input.iter().enumerate().map(|k| (k.0, *k.1)).collect();

    println!("Initial arrangement:");
    print(&work);

    for round in 0..rounds {
        for iteration in 0..work.len() {
            let (move_from, value) = work.iter().find_position(|entry| entry.0 == iteration).unwrap();
            
            
        }

        println!("After {} round of mixing:", round + 1);
        print(&work);
    }

    let (pos, v) = work.iter().find_position(|entry| entry.1 == 0).unwrap();

    println!("0 pos: {}, value: {:?}", pos, v);
    let a = work[(pos + 1000) % work.len()].1;
    let b = work[(pos + 2000) % work.len()].1;
    let c = work[(pos + 3000) % work.len()].1;

    println!("Numbers: {} {} {}", a, b, c);
    a + b + c
}

fn problem2(input: Input) -> usize {
    0
}

struct Entry {
    index: usize,
    value: isize,
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    #[test]
    fn test_problems_1() {
        assert_eq!(3, problem1(1, parse(1, include_str!("test_puzzle.txt"))));
        assert_eq!(7225, problem1(1, parse(1, include_str!("puzzle.txt")))); //Not -258
    }

    #[test]
    fn test_problems_2() {
        assert_eq!(0, problem1(1, parse(811589153, include_str!("test_puzzle.txt"))));
        assert_eq!(0, problem1(1, parse(811589153, include_str!("puzzle.txt"))));
    }

    fn parse(mult: isize, s: &str) -> Input {
        s.lines()
            .map(|line| {
                let numbers = util::parse_numbers(line);
                numbers[0] * mult
            })
            .collect()
    }
}
