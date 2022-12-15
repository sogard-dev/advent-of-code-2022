use crate::grid::{Grid, Coordinate};

type Input = Grid<GridModel>;

pub fn main() {
    println!("Day14");
}

fn print_grid(grid: &Grid<GridModel>) {
    grid.print(|pos| {
        match grid.get_model(&pos).unwrap().t {
            Type::Air => ".",
            Type::Sand => "O",
            Type::Rock => "#",
            Type::SandSource => "+",
        }.to_string()
    })
}

fn problem1(mut grid: Input) -> isize {
    let sand_spawner = grid.find(|_, m| m.t.eq(&Type::SandSource)).unwrap();

    println!("Sand spawner: {:?}", sand_spawner);

    while spawn_and_trickle(&mut grid, &sand_spawner){}

    let sum = grid.sum(|_, m| {
        match m.t {
            Type::Sand => 1,
            _ => 0
        }
    });

    print_grid(&grid);

    sum
}

fn spawn_and_trickle(grid: &mut Grid<GridModel>, sand_position: &Coordinate) -> bool {
    grid.get_model_mut(&sand_position).unwrap().t = Type::Sand;

    let mut current_position = sand_position.clone();
    loop {
        let under = (current_position.0 + 1, current_position.1);
        let under_left = (current_position.0 + 1, current_position.1 - 1);
        let under_right = (current_position.0+1, current_position.1 + 1);

        let mut moved = false;

        for pos in [under, under_left, under_right] {
            moved = grid.swap_if(&current_position, &pos, |_, candi| candi.t.eq(&Type::Air));
            if moved {
                current_position = pos;
                break;
            }

            if grid.get_model_mut(&pos).is_none() {
                grid.get_model_mut(&current_position).unwrap().t = Type::Air;
                return false;
            }
        }

        if !moved {
            break;
        }
    }

    true
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum Type {
    Air, Sand, Rock, SandSource
}

struct GridModel {
    t: Type,
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_problems() {
        assert_eq!(24, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(696, problem1(parse(include_str!("puzzle.txt"))));

        assert_eq!(93, problem1(parse_2(include_str!("test_puzzle.txt"))));
        assert_eq!(23610, problem1(parse_2(include_str!("puzzle.txt"))));
    }

    fn parse(s: &str) -> Input {
        let done: Vec<Vec<Vec<isize>>> = s.lines().map(|line| line.split(" -> ").map(|entry| entry.split(",").map(|v| v.parse::<isize>().unwrap()).collect()).collect()).collect();

        let mut min_right = isize::MAX;
        let mut min_down = 0;
        let mut max_right = isize::MIN;
        let mut max_down = isize::MIN;

        let mut set_of_rocks = HashSet::new();

        for line in done {
            for i in 0..line.len() - 1 {
                let from = &line[i];
                let to = &line[i + 1];

                // println!("{:?} -> {:?}", from, to);

                let inc_right = (to[0] - from[0]).signum();
                let inc_down = (to[1] - from[1]).signum();

                let mut rock = from.clone();
                set_of_rocks.insert(rock.clone());

                // println!("Diff: {:?}, {:?}", inc_right, inc_down);

                while rock[0] != to[0] || rock[1] != to[1] {
                    rock = vec![rock[0] + inc_right, rock[1] + inc_down];
                    // println!("Rock: {:?}", rock);

                    set_of_rocks.insert(rock.clone());
                }
            }

            for entry in line {
                min_right = entry[0].min(min_right);
                max_right = entry[0].max(max_right);

                min_down = entry[1].min(min_down);
                max_down = entry[1].max(max_down);
            }
        }

        let mut scan = vec![];
        for down in min_down..=max_down {
            let mut line = String::new();
            for right in min_right..=max_right {
                if right == 500 && down == 0 {
                    line.push('+');
                } else if set_of_rocks.contains(&vec![right, down]) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            scan.push(line);
        }

        Grid::new(&scan.join("\n"), |_, c| match c {
            '#' => GridModel {t: Type::Rock},
            '.' => GridModel {t: Type::Air},
            '+' => GridModel {t: Type::SandSource},
            _ => panic!("Unknown type: {}", c)
        })
    }

    fn parse_2(s: &str) -> Input {
        let done: Vec<Vec<Vec<isize>>> = s.lines().map(|line| line.split(" -> ").map(|entry| entry.split(",").map(|v| v.parse::<isize>().unwrap()).collect()).collect()).collect();

        let mut min_right = isize::MAX;
        let mut min_down = 0;
        let mut max_right = isize::MIN;
        let mut max_down = isize::MIN;

        let mut set_of_rocks = HashSet::new();

        for line in done {
            for i in 0..line.len() - 1 {
                let from = &line[i];
                let to = &line[i + 1];

                // println!("{:?} -> {:?}", from, to);

                let diff_right = to[0] - from[0];
                let diff_down = to[1] - from[1];

                let inc_right = if diff_right > 0 {
                    1
                } else if diff_right < 0 {
                    -1
                } else {
                    0
                };
                let inc_down = if diff_down > 0 {
                    1
                } else if diff_down < 0 {
                    -1
                } else {
                    0
                };

                let mut rock = from.clone();
                set_of_rocks.insert(rock.clone());

                // println!("Diff: {:?}, {:?}", inc_right, inc_down);

                while rock[0] != to[0] || rock[1] != to[1] {
                    rock = vec![rock[0] + inc_right, rock[1] + inc_down];
                    // println!("Rock: {:?}", rock);

                    set_of_rocks.insert(rock.clone());
                }
            }

            for entry in line {
                min_right = entry[0].min(min_right);
                max_right = entry[0].max(max_right);

                min_down = entry[1].min(min_down);
                max_down = entry[1].max(max_down);
            }
        }

        max_down = max_down + 2;
        min_right = min_right - (max_down - min_down);
        max_right = max_right + (max_down - min_down);

        for i in min_right..=max_right {
            set_of_rocks.insert(vec![i, max_down]);
        }

        let mut scan = vec![];
        for down in min_down..=max_down {
            let mut line = String::new();
            for right in min_right..=max_right {
                if right == 500 && down == 0 {
                    line.push('+');
                } else if set_of_rocks.contains(&vec![right, down]) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            scan.push(line);
        }

        Grid::new(&scan.join("\n"), |_, c| match c {
            '#' => GridModel {t: Type::Rock},
            '.' => GridModel {t: Type::Air},
            '+' => GridModel {t: Type::SandSource},
            _ => panic!("Unknown type: {}", c)
        })
    }
}
