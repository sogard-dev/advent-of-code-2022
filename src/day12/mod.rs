use std::collections::{HashSet, HashMap, VecDeque};
type Coordinate = (isize, isize);

pub fn main() {
    println!("Day12");
}


fn bfs(grid: &Grid) -> HashMap<Coordinate, isize> {
    let init = grid.end;

    let mut to_visit = VecDeque::new();
    let mut added_to_visit = HashSet::new();

    to_visit.push_back(init);
    added_to_visit.insert(init);

    let mut distances = HashMap::new();

    let mut distance = -1;
    while !to_visit.is_empty() {
        distance += 1;

        let mut seen = vec![];

        while let Some(pos_to_check) = to_visit.pop_front() {
            distances.insert(pos_to_check, distance);

            for neighbour_pos in grid.connections.get(&pos_to_check).unwrap() {
                seen.push(*neighbour_pos);
            }
        }       

        for pos in seen {
            if !added_to_visit.contains(&pos)   {
                added_to_visit.insert(pos);
                to_visit.push_back(pos);
            }
        }
    }

    distances
}

fn print(grid: &Grid, distances: &HashMap<Coordinate, isize>) {
    println!("Distances:");
    for row_index in 0..grid.height {
        for column_index in 0..grid.width {
            match distances.get(&(row_index, column_index)) {
                Some(d) => print!("{:>4} ", d),
                None => print!("  ?  ")
            };
        }

        println!();
    }
    println!();
}

fn problem1(grid: Grid) -> isize {
    let distances = bfs(&grid);
    // print(&grid, &distances);
    *distances.get(&grid.start).unwrap()
}

fn problem2(grid: Grid) -> isize {
    let distances = bfs(&grid);

    let mut smallest = isize::MAX;

    for row_index in 0..grid.height {
        for column_index in 0..grid.width {
            let pos = (row_index, column_index);
            match distances.get(&pos) {
                Some(d) => {
                    if *grid.get_height(&pos).unwrap() == 'a' as isize {
                        smallest = smallest.min(*d);
                    }
                },
                None => {}
            };
        }
    }

    smallest
}

struct Grid {
    connections: HashMap<Coordinate, HashSet<Coordinate>>,
    nodes: HashMap<Coordinate, isize>,
    width: isize,
    height: isize,
    start: Coordinate,
    end: Coordinate,
}

impl Grid {
    fn new(width: isize, height: isize) -> Grid {
        Grid { width, height, connections: HashMap::new(), nodes: HashMap::new(), start: (0,0), end:(0,0) }
    }

    fn set_height(&mut self, row: isize, column: isize, height: isize) {
        self.nodes.insert((row, column), height);
    }

    fn get_height(&self, pos: &Coordinate) -> Option<&isize> {
        self.nodes.get(pos)
    }

    fn add_directional(&mut self, pos_1: Coordinate, pos_2: Coordinate) {
        self.connections.entry(pos_1).or_insert_with(|| HashSet::new()).insert(pos_2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problems() {
        assert_eq!(31, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(425, problem1(parse(include_str!("puzzle.txt"))));

        assert_eq!(29, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(418, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse(s: &str) -> Grid {
        let height = s.lines().count();
        let width = s.lines().into_iter().next().unwrap().len();

        let mut grid =  Grid::new(width as isize, height as isize);

        let lines: Vec<&str> = s.lines().collect();
        for (row_index, line) in lines.iter().enumerate() {
            for (column_index, char) in line.chars().enumerate() {
                if char.eq(&'S') {
                    grid.set_height(row_index as isize, column_index as isize, 'a' as isize);
                    grid.start = (row_index as isize, column_index as isize);
                } else if char.eq(&'E') {
                    grid.set_height(row_index as isize, column_index as isize, 'z' as isize);
                    grid.end = (row_index as isize, column_index as isize);
                } else {
                    grid.set_height(row_index as isize, column_index as isize, char as isize);
                }
            }
        }

        for row_index in 0..height {
            for column_index in 0..width {
                let my_pos = (row_index as isize, column_index as isize);
                let my_height = *grid.get_height(&my_pos).unwrap();

                for neighbour in [(-1,0), (1,0), (0,-1), (0,1)] {
                    let neighbour_pos = (my_pos.0 - neighbour.0, my_pos.1 - neighbour.1);

                    if let Some(n_height) = grid.get_height(&neighbour_pos) {
                        if my_height - *n_height < 2 {
                            grid.add_directional(my_pos, neighbour_pos);
                        }
                    }
                }
            }
        }

        grid
    }
}
