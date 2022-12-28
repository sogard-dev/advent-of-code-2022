use std::{collections::HashSet, str::FromStr};

pub fn main() {
    println!("Day9");
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn to_pos(&self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::UpLeft => (-1, 1),
            Direction::UpRight => (1, 1),
            Direction::DownLeft => (-1, -1),
            Direction::DownRight => (1, -1),
        }
    }
}

struct Datastructure {
    head: (i32, i32),
    tails: Vec<(i32, i32)>,
    tail_visited: HashSet<(i32, i32)>,
}

impl Datastructure {
    fn move_head(&mut self, direction: &Direction) {
        let dir_pos = direction.to_pos();
        self.head = (self.head.0 + dir_pos.0, self.head.1 + dir_pos.1);
    }

    fn move_tail(&mut self, tail_index: usize, direction: &Direction) {
        let dir_pos = direction.to_pos();

        let current_tail = self.tails[tail_index];

        self.tails[tail_index] = (current_tail.0 + dir_pos.0, current_tail.1 + dir_pos.1);
        if tail_index == self.tails.len() - 1 {
            self.tail_visited.insert(current_tail);
            self.tail_visited.insert(self.tails[tail_index]);
        }
    }

    fn tail_index_follow(&mut self, tail_index: usize, following: (i32, i32)) {
        let tail = self.tails[tail_index];

        let same_x = following.0 == tail.0;
        let same_y = following.1 == tail.1;

        if same_x && same_y {
            return;
        }

        let vertical_diff = (following.1 - tail.1).abs();
        let horizontal_diff = (following.0 - tail.0).abs();

        if vertical_diff == 2 || horizontal_diff == 2 {
            let head_is_left = following.0 < tail.0;
            let head_is_up = following.1 > tail.1;

            if same_x {
                if head_is_up {
                    self.move_tail(tail_index, &Direction::Up);
                } else {
                    self.move_tail(tail_index, &Direction::Down);
                }
            } else if same_y {
                if head_is_left {
                    self.move_tail(tail_index, &Direction::Left);
                } else {
                    self.move_tail(tail_index, &Direction::Right);
                }
            } else if head_is_left && head_is_up {
                self.move_tail(tail_index, &Direction::UpLeft);
            } else if head_is_left && !head_is_up {
                self.move_tail(tail_index, &Direction::DownLeft);
            } else if !head_is_left && head_is_up {
                self.move_tail(tail_index, &Direction::UpRight);
            } else if !head_is_left && !head_is_up {
                self.move_tail(tail_index, &Direction::DownRight);
            } else {
                panic!("Should not go here");
            }
        }
    }

    fn tail_follow(&mut self) {
        self.tail_index_follow(0, self.head);
        for index in 1..self.tails.len() {
            self.tail_index_follow(index, self.tails[index - 1]);
        }
    }
}

fn print(structure: &Datastructure) {
    for y in (0..6).rev() {
        for x in 0..6 {
            if structure.head == (x, y) {
                print!("H");
            } else {
                let mut printed = false;
                for tail_index in 0..structure.tails.len() {
                    if (x, y) == structure.tails[tail_index] {
                        print!("{}", tail_index + 1);
                        printed = true;
                        break;
                    }
                }

                if !printed {
                    print!(".")
                }
            }
        }
        println!("");
    }

    println!("");
}

fn problem1(s: &str) -> usize {
    solve(s, 1)
}

fn problem2(s: &str) -> usize {
    solve(s, 9)
}

fn solve(s: &str, tail_count: usize) -> usize {
    let mut tails = vec![];
    for _ in 0..tail_count {
        tails.push((0, 0));
    }

    let mut structure = Datastructure { head: (0, 0), tails: tails, tail_visited: HashSet::new() };
    structure.tail_visited.insert((0, 0));

    //print(&structure);

    for line in s.lines() {
        let spl: Vec<&str> = line.split(" ").collect();
        let direction = Direction::from_str(spl[0]).unwrap();
        let times = spl[1].parse::<usize>().unwrap();

        //println!("== {} {} ==", spl[0], spl[1]);
        //println!();
        for _ in 0..times {
            structure.move_head(&direction);
            structure.tail_follow();
            //print(&structure);
        }
    }

    structure.tail_visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problems() {
        assert_eq!(13, problem1(include_str!("test_puzzle.txt")));
        assert_eq!(6256, problem1(include_str!("puzzle.txt")));

        assert_eq!(1, problem2(include_str!("test_puzzle.txt")));
        assert_eq!(36, problem2(include_str!("test_puzzle2.txt")));
        assert_eq!(2665, problem2(include_str!("puzzle.txt")));
    }
}
