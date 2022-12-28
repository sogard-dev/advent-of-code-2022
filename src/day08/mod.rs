use std::collections::{HashMap, HashSet};

use crate::grid::{self, Coordinate, Grid};

pub fn main() {
    println!("Day8");
}

fn parse(s: &str) -> Vec<Vec<u32>> {
    return s.lines().into_iter().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
}

fn problem1(s: &str) -> usize {
    let grid = Grid::new(s, |_, c| c.to_digit(10).unwrap());

    let a: Vec<Coordinate> = (1..=grid.height).map(|i| (1 * i, 0)).collect();
    let b = (1..=grid.height).map(|i| (-1 * i, 0)).collect();
    let c = (1..=grid.width).map(|i| (0, 1 * i)).collect();
    let d = (1..=grid.width).map(|i| (0, -1 * i)).collect();

    let deltas = [a, b, c, d].concat();

    let mut blocked = HashMap::new();

    grid.for_every_delta(
        |me, my_height, them, their_height| {
            if my_height <= their_height {
                let dir = grid::get_direction(me, them);
                blocked.entry(*me).or_insert_with(|| HashSet::new()).insert(dir);
            }
        },
        deltas,
    );

    let mut visible_trees = 0;
    grid.for_every(|pos, _| match blocked.get(&pos) {
        Some(set) => {
            if set.len() != 4 {
                visible_trees += 1
            }
        }
        None => visible_trees += 1,
    });

    visible_trees
}

fn problem2(s: &str) -> usize {
    let forest = parse(s);
    let columns = forest.len();
    let rows = forest[0].len();

    let mut best_score = 0;

    for row in 0..rows {
        for column in 0..columns {
            let my_height = forest[row][column];

            let mut up = 0;
            for r in (0..row).rev() {
                up += 1;
                if forest[r][column] >= my_height {
                    break;
                }
            }

            let mut down = 0;
            for r in row + 1..rows {
                down += 1;
                if forest[r][column] >= my_height {
                    break;
                }
            }

            let mut right = 0;
            for c in column + 1..columns {
                right += 1;
                if forest[row][c] >= my_height {
                    break;
                }
            }

            let mut left = 0;
            for c in (0..column).rev() {
                left += 1;
                if forest[row][c] >= my_height {
                    break;
                }
            }

            //println!("({},{}) {} => {},{},{},{}", row, column, my_height, up, left, right, down);

            let score = up * down * left * right;
            if score > best_score {
                best_score = score;
            }
        }
    }

    best_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(21, problem1(include_str!("test_puzzle.txt")));
        assert_eq!(1870, problem1(include_str!("puzzle.txt")));

        assert_eq!(8, problem2(include_str!("test_puzzle.txt")));
        assert_eq!(517440, problem2(include_str!("puzzle.txt")));
    }
}
