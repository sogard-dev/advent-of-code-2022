pub fn main() {
    println!("Day8");
}

fn parse(s: &str) -> Vec<Vec<u32>> {
    return s.lines().into_iter().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
}

fn problem1(s: &str) -> usize {
    let forest = parse(s);
    let columns = forest.len();
    let rows = forest[0].len();

    //Forest: row,column

    let mut visible_trees = 0;

    for row in 0..rows {
        for column in 0..columns {
            let mut visible = false;

            if row == 0 || row == rows - 1 || column == 0 || column == columns - 1 {
                visible = true;
            } else {
                let my_height = forest[row][column];

                let mut is_blocked = 0;
                for r in 0..row {
                    if forest[r][column] >= my_height {
                        // println!(
                        //     "({},{}) {} blocked by ({},{}) {}",
                        //     row, column, my_height, r, column, forest[r][column]
                        // );
                        is_blocked += 1;
                        break;
                    }
                }

                for r in row + 1..rows {
                    if forest[r][column] >= my_height {
                        // println!(
                        //     "({},{}) {} blocked by ({},{}) {}",
                        //     row, column, my_height, r, column, forest[r][column]
                        // );
                        is_blocked += 1;
                        break;
                    }
                }

                for c in 0..column {
                    if forest[row][c] >= my_height {
                        // println!(
                        //     "({},{}) {} blocked by ({},{}) {}",
                        //     row, column, my_height, row, c, forest[row][c]
                        // );
                        is_blocked += 1;
                        break;
                    }
                }

                for c in column + 1..columns {
                    if forest[row][c] >= my_height {
                        // println!(
                        //     "({},{}) {} blocked by ({},{}) {}",
                        //     row, column, my_height, row, c, forest[row][c]
                        // );
                        is_blocked += 1;
                        break;
                    }
                }

                if is_blocked != 4 {
                    // println!("({},{}) {} is visible", row, column, my_height);
                    visible = true;
                }
            }

            if visible {
                visible_trees += 1;
            }
        }
    }

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

            println!("({},{}) {} => {},{},{},{}", row, column, my_height, up, left, right, down);

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
        assert_eq!(0, problem2(include_str!("puzzle.txt")));
    }
}
