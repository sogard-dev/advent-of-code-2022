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

    for _ in 0..rounds {
        for iteration in 0..work.len() {
            let (move_from, value) = work.iter().find_position(|entry| entry.0 == iteration).unwrap();
            let (_, jump) = *value;

            let sign = jump.signum();
            if sign != 0 {
                let mut at = move_from as isize;

                let jumps = jump.abs() as usize;
                let jumps = jumps % (work.len() - 1);

                for _ in 0..jumps {
                    let swap_b = at + sign;


                    let roll_b: usize = {
                        if sign == 1 {
                            if at + 1 == work.len() as isize {
                                0 as usize
                            } else {
                                swap_b as usize
                            }
                        } else {
                            if at == 0 {
                                work.len() - 1
                            } else {
                                swap_b as usize
                            }
                        }
                    };

                    //println!("  Swap {} and {}", at, roll_b);

                    work.swap(at as usize, roll_b);

                    let new_at = swap_b;
                    if new_at == -1 {
                        at = (work.len() - 1) as isize;
                    } else if new_at == work.len() as isize {
                        at = 0;
                    } else {
                        at = new_at;
                    }
                }
            }

            //println!("After moving: {}", jump);
            //print(&work);
        }

        //println!("After {} round of mixing:", round + 1);
        //print(&work);
    }

    let (pos, v) = work.iter().find_position(|entry| entry.1 == 0).unwrap();

    println!("0 pos: {}, value: {:?}", pos, v);
    let a = work[(pos + 1000) % work.len()].1;
    let b = work[(pos + 2000) % work.len()].1;
    let c = work[(pos + 3000) % work.len()].1;

    println!("Numbers: {} {} {}", a, b, c);
    a + b + c
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
        assert_eq!(1623178306, problem1(10, parse(811589153, include_str!("test_puzzle.txt"))));
        assert_eq!(548634267428, problem1(10, parse(811589153, include_str!("puzzle.txt"))));
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
