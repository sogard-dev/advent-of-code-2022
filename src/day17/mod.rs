use std::{collections::HashSet, borrow::BorrowMut};

use crate::grid::Coordinate;

type Input = Champer;

pub fn main() {
    println!("18");
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct Rock {
    width: usize,
    height: usize,
    pattern: HashSet<(usize, usize)>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct Champer {
    jets: Vec<isize>,
    rocks: Vec<Rock>,
}

fn can_rock_move_here(rock: &Rock, pos: Coordinate, stable_rocks: &HashSet<Coordinate>) -> bool {
    let x = pos.0;
    let y = pos.1;

    //Sides
    if x < 0 || x + rock.width as isize > 7 {
        return false;
    }

    //Floor
    if y - (rock.height as isize) + 1 < 0 {
        return false;
    }

    //Stable rocks
    for p in rock.pattern.iter() {
        let row = y - (p.1 as isize);
        if stable_rocks.contains(&((p.0 as isize) + x, row)) {
            return false;
        }
    }
    return true;
}

fn clean_if_possible(stable_rocks: &mut HashSet<Coordinate>, highest_rock: isize) {
    let remove_under = highest_rock - 100;

    let to_remove: HashSet<Coordinate> = stable_rocks.iter().filter(|(_, y)| *y < remove_under).map(|e| *e).collect();

    for coord in to_remove.iter() {
        stable_rocks.remove(coord);
    }
}

fn move_to_top(stable_rocks: &mut HashSet<Coordinate>, increase: isize) {
    
    let moved: Vec<Coordinate> = stable_rocks.iter().map(|e| (e.0, e.1+increase)).collect();

    stable_rocks.clear();

    for item in moved {
        stable_rocks.insert(item);
    }

}

fn problem1(rocks: usize, input: Input) -> isize {
    let mut stable_rock = HashSet::new();
    let mut highest_rock = -1;
    let mut jet_index = 0;
    let mut rock_index = 0;

    let print_every = input.rocks.len() * input.jets.len();

    let mut snapshot = None;
    let mut last_snapshot_cycle = 0;
    let mut last_snapshot_highest = 0;

    let mut rock_no = 0;
    while rock_no < rocks {
        let rock = &input.rocks[rock_index];
        let mut x = 2;
        let mut y = highest_rock + 3 + rock.height as isize;

        loop {
            let jet = &input.jets[jet_index];
            jet_index = (jet_index + 1) % input.jets.len();

            let new_x = x + jet;

            if can_rock_move_here(rock, (new_x, y), &stable_rock) {
                x = new_x;
            }

            let new_y = y - 1;
            if can_rock_move_here(rock, (x, new_y), &stable_rock) {
                y = new_y;
            } else {
                highest_rock = highest_rock.max(y);

                for p in rock.pattern.iter() {
                    stable_rock.insert((p.0 as isize + x, y - p.1 as isize));
                }
                if stable_rock.len() > 200 {
                    clean_if_possible(&mut stable_rock, highest_rock);
                }
                break;
            }
        }

        rock_index = (rock_index + 1) % input.rocks.len();

        if rock_no % print_every == 0 {
            println!("Height at {} is {}", rock_no, highest_rock);

            if rock_no == print_every {
                last_snapshot_cycle = rock_no;
                snapshot = Some(get_snap(&stable_rock, highest_rock));
                last_snapshot_highest = highest_rock;
            }
        }

        if let Some(snap) = &snapshot {
            if snap.eq(&get_snap(&stable_rock, highest_rock)) {
                let diff_high = highest_rock - last_snapshot_highest;
                let diff = rock_no - last_snapshot_cycle;
                println!("Found snapshot at {}, highest: {}, since: {}, high diff: {}", rock_no, highest_rock, diff, diff_high);
                last_snapshot_cycle = rock_no;
                last_snapshot_highest = highest_rock;

                if diff > 0 {
                    let remaining = rocks - rock_no;
                    let steps_to_take = remaining / diff;

                    rock_no += steps_to_take * diff;
                    highest_rock += steps_to_take as isize * diff_high;


                    let moved_up = steps_to_take as isize * diff_high;

                    move_to_top(&mut stable_rock, moved_up as isize);
                    snapshot = None;
                    println!("Done, zooomed up to {}", rock_no);
                }
            }
        }

        //println!("Rock no: {}", rock_no);

        rock_no += 1;
    }

    stable_rock.iter().map(|k| k.1).max().unwrap() + 1
}

fn get_snap(stable_rocks: &HashSet<Coordinate>, highest_rock: isize) -> String {
    let mut str = String::new();
    for height in (highest_rock - 50..=highest_rock).rev() {
        str.push('|');

        for i in 0..7 {
            if stable_rocks.contains(&(i, height)) {
                str.push('#');
            } else {
                str.push('.');
            }
        }
        str.push('|');

        str.push('\n');
    }

    str
}

fn problem2(rocks: Input) -> isize {
    problem1(1000000000000, rocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problems_1() {
        assert_eq!(3068, problem1(2022, parse(include_str!("test_puzzle.txt"))));
        assert_eq!(3209, problem1(2022, parse(include_str!("puzzle.txt"))));
    }

    #[test]
    fn test_problems_2() {
        assert_eq!(1514285714288, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(1580758017509, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse_rock(s: &str) -> Rock {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();

        let mut pattern = HashSet::new();
        for (row, line) in s.lines().enumerate() {
            for (column, char) in line.chars().enumerate() {
                if char == '#' {
                    pattern.insert((column, row));
                }
            }
        }
        Rock { width, height, pattern }
    }

    fn get_rocks() -> Vec<Rock> {
        vec![parse_rock("####"), parse_rock(".#.\n###\n.#."), parse_rock("..#\n..#\n###"), parse_rock("#\n#\n#\n#"), parse_rock("##\n##")]
    }

    fn parse(s: &str) -> Input {
        Champer { jets: s.chars().map(|v| if v == '<' { -1 } else { 1 }).collect(), rocks: get_rocks() }
    }
}
