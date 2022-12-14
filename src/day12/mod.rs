use std::collections::HashMap;

use crate::grid::{Coordinate, Grid};

pub fn main() {
    println!("Day12");
}

fn bfs(grid: &Grid<GridModel>, from: Coordinate) -> HashMap<Coordinate, isize> {
    let mut distances = HashMap::new();

    grid.bfs(&from, |pos, distance| {
        distances.insert(pos, distance);
    });

    print(&grid, &distances);

    distances
}

fn print(grid: &Grid<GridModel>, distances: &HashMap<Coordinate, isize>) {
    grid.print(|pos| {         
        match distances.get(&pos) {
            Some(d) => format!("{:>4} ", d),
            None => "  ?  ".to_string(),
        }
    });
}

fn problem1(arg: (Grid<GridModel>, Coordinate, Coordinate)) -> isize {
    let (grid, start, end) = arg;
    let distances = bfs(&grid, end);
    *distances.get(&start).unwrap()
}

fn problem2(arg: (Grid<GridModel>, Coordinate, Coordinate)) -> isize {
    let (grid, _, end) = arg;

    let distances = bfs(&grid, end);

    let mut smallest = isize::MAX;
    grid.for_every(|pos, m| {
        match distances.get(&pos) {
            Some(d) => {
                if m.height == 'a' as isize {
                    smallest = smallest.min(*d);
                }
            }
            None => {}
        };
    });

    smallest
}

struct GridModel {
    height: isize,
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    use super::*;

    #[test]
    fn test_problems() {
        assert_eq!(31, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(425, problem1(parse(include_str!("puzzle.txt"))));

        assert_eq!(29, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(418, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse(s: &str) -> (Grid<GridModel>, Coordinate, Coordinate) {
        let mut start = None;
        let mut end = None;

        let mut grid = Grid::new(s, |pos, char| {
            if char == 'S' {
                start = Some(pos);
                GridModel { height: 'a' as isize }
            } else if char == 'E' {
                end = Some(pos);
                GridModel { height: 'z' as isize }
            } else {
                GridModel { height: char as isize }
            }
        });

        let deltas = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut connections = vec![];
        grid.for_every_delta(
            |from, m1, to, m2| {
                if m1.height - m2.height < 2 {
                    connections.push((*from, *to));
                }
            },
            deltas,
        );
        grid.add_connections(connections);

        (grid, start.unwrap(), end.unwrap())
    }
}
