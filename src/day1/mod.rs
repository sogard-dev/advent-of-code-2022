pub fn main() {
    println!("Day1");
}

fn puzzle_1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut arr: Vec<usize> = vec![];

    let mut idx = 0;
    for s in lines {
        match s.parse::<usize>() {
            Ok(num) => {
                if arr.len() < idx + 1 {
                    arr.push(num);
                } else {
                    arr[idx] += num;
                }
            }
            Err(_) => {
                idx += 1;
            }
        }
    }

    *arr.iter().max().unwrap()
}

fn puzzle_2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut arr: Vec<usize> = vec![];

    let mut idx = 0;
    for s in lines {
        match s.parse::<usize>() {
            Ok(num) => {
                if arr.len() < idx + 1 {
                    arr.push(num);
                } else {
                    arr[idx] += num;
                }
            }
            Err(_) => {
                idx += 1;
            }
        }
    }

    arr.sort();
    arr.reverse();

    return arr[0] + arr[1] + arr[2];
}

#[cfg(test)]
mod tests {
    use crate::day1::*;

    #[test]
    fn it_works() {
        assert_eq!(24000, puzzle_1(include_str!("puzzle_1_test.txt")));
        assert_eq!(72511, puzzle_1(include_str!("puzzle_1.txt")));

        assert_eq!(45000, puzzle_2(include_str!("puzzle_1_test.txt")));
        assert_eq!(212117, puzzle_2(include_str!("puzzle_1.txt")));
    }
}
