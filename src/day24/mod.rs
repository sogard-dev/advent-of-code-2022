use std::collections::{HashMap, HashSet};

type Input = Vec<Storm>;

pub fn main() {
    println!("23");
}

fn print(storms: &Input, round: usize, start: &(usize, usize), end: &(usize, usize)) {
    let min_row = 0;
    let min_column = 0;
    let max_row = storms.iter().next().unwrap().rows;
    let max_column = storms.iter().next().unwrap().columns;

    let mut map = HashMap::new();

    for storm in storms.iter() {
        map.entry(storm.position_in_round(round)).or_insert_with(|| vec![]).push(storm.direction.clone());
    }

    println!("Minute {}", round);
    for r in min_row..=max_row {
        for c in min_column..=max_column {
            let pos = (r, c);
            if pos.eq(start) {
                print!("S");
            } else if pos.eq(end) {
                print!("E");
            } else {
                if let Some(storms) = map.get(&(r, c)) {
                    if storms.len() == 1 {
                        print!("{}", storms[0].to_map_char());
                    } else {
                        print!("{}", storms.len());
                    }
                } else {
                    if r == 0 || r == max_row || c == 0 || c == max_column {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
        }
        println!("");
    }
    println!("");
}

fn problem1(storms: Input) -> usize {
    let max_row = storms.iter().next().unwrap().rows;
    let max_column = storms.iter().next().unwrap().columns;

    let start = (0, 1);
    let end = (max_row, max_column - 1);

    for round in 0..2 {
        print(&storms, round, &start, &end);
    }

    bfs(start, end, &storms, 0) + 1
}

fn bfs(start: (usize, usize), end: (usize, usize), storms: &Input, start_round: usize) -> usize {
    let max_row = storms.iter().next().unwrap().rows;
    let max_column = storms.iter().next().unwrap().columns;

    let mut open_set = HashSet::new();
    open_set.insert(start);

    for round in start_round..1000000000 {
        if open_set.len() == 0 {
            break;
        }

        //println!("Round {}, positions: {}", round, open_set.len());

        let mut new_open_set = HashSet::new();
        let storm_map = storm_map(storms, round + 1);

        for pos in open_set {
            for diff in [(1, 0), (0, 1), (0, -1), (-1, 0), (0, 0)] {
                let new_row = ((pos.0 as isize) + diff.0) as usize;
                let new_column = ((pos.1 as isize) + diff.1) as usize;
                let new_pos = (new_row, new_column);

                if new_pos.eq(&end) {
                    return round;
                }

                if (new_pos.0 <= 0 || new_pos.1 <= 0 || new_pos.0 >= max_row || new_pos.1 >= max_column) && !new_pos.eq(&start) {
                    //println!("  Throwing away {:?}", new_pos);
                    continue;
                }

                if !storm_map.contains(&new_pos) {
                    //println!("  Adding {:?}", new_pos);
                    new_open_set.insert(new_pos);
                }
            }
        }

        open_set = new_open_set;
    }

    0
}

fn storm_map(storms: &[Storm], round: usize) -> HashSet<(usize, usize)> {
    let mut map = HashSet::new();
    storms.iter().for_each(|s| {
        map.insert(s.position_in_round(round));
    });
    map
}

fn problem2(storms: Input) -> usize {
    let max_row = storms.iter().next().unwrap().rows;
    let max_column = storms.iter().next().unwrap().columns;

    let start = (0, 1);
    let end = (max_row, max_column - 1);

    for round in 0..2 {
        print(&storms, round, &start, &end);
    }

    let trip_1 = bfs(start, end, &storms, 0) + 1;
    let trip_2 = bfs(end, start, &storms, trip_1) + 1;
    let trip_3 = bfs(start, end, &storms, trip_2) + 1;

    println!("Trips: {}, {}, {}", trip_1, trip_2, trip_3);

    trip_3
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn to_map_char(&self) -> &str {
        match self {
            Direction::N => "^",
            Direction::S => "v",
            Direction::W => "<",
            Direction::E => ">",
        }
    }
}

struct Storm {
    row: usize,
    column: usize,
    direction: Direction,
    rows: usize,
    columns: usize,
}

impl Storm {
    fn new(row: usize, column: usize, direction: Direction, rows: usize, columns: usize) -> Storm {
        Storm { row, column, direction, rows, columns }
    }

    fn position_in_round(&self, round: usize) -> (usize, usize) {
        let overflow_row = round * (self.rows - 1);
        let overflow_column = round * (self.columns - 1);
        match self.direction {
            Direction::N => (((overflow_row + self.row - 1 - round) % (self.rows - 1)) + 1, self.column),
            Direction::S => (((overflow_row + self.row - 1 + round) % (self.rows - 1)) + 1, self.column),
            Direction::W => (self.row, ((overflow_column + self.column - 1 - round) % (self.columns - 1)) + 1),
            Direction::E => (self.row, ((overflow_column + self.column - 1 + round) % (self.columns - 1)) + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problems_1() {
        assert_eq!(18, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(247, problem1(parse(include_str!("puzzle.txt"))));
    }

    #[test]
    fn test_problems_2() {
        assert_eq!(54, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(728, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse(s: &str) -> Input {
        let rows = s.lines().count() - 1;
        let columns = s.lines().next().unwrap().len() - 1;

        let mut storms = vec![];
        s.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(column, c)| {
                match c {
                    '<' => storms.push(Storm::new(row, column, Direction::W, rows, columns)),
                    '>' => storms.push(Storm::new(row, column, Direction::E, rows, columns)),
                    '^' => storms.push(Storm::new(row, column, Direction::N, rows, columns)),
                    'v' => storms.push(Storm::new(row, column, Direction::S, rows, columns)),
                    _ => {}
                };
                ()
            })
        });

        storms
    }
}
