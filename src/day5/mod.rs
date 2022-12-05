use regex::Regex;

pub fn main() {
    println!("Day5");
}

fn problem1(s: &str) -> String {
    let results = parse(s);

    let mut stacks = results.stacks;


    for (amt, from, to) in results.movements {
        for _ in 0..amt {
            let take = stacks[from].pop().unwrap();
            stacks[to].push(take)
        }
    }

    let mut output = String::new();
    for stack in stacks {
        if let Some(letter) = stack.last() {
            output.push(*letter);
        }
    }

    output
}

fn problem2(s: &str) -> String {
    let results = parse(s);

    let mut stacks = results.stacks;

    for (amt, from, to) in results.movements {
        let mut tmp = vec![];
        for _ in 0..amt {
            let take = stacks[from].pop().unwrap();
            tmp.push(take);
        }

        tmp.reverse();
        for take in tmp {
            stacks[to].push(take);
        }
    }

    let mut output = String::new();
    for stack in stacks {
        if let Some(letter) = stack.last() {
            output.push(*letter);
        }
    }

    output
}

fn parse(s: &str) -> ParseResult {
    let mut stacks = vec![];
    stacks.push(vec![]);

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut movements = vec![];

    for line in s.lines() {
        if re.is_match(line) {
            let cap = re.captures(line).unwrap();

            movements.push((
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
            ));
        } else {
            for (column, c) in line.chars().enumerate() {
                if c.is_ascii_uppercase() {
                    let place = (column - 1) / 4 + 1;
                    for _ in stacks.len()..=place {
                        stacks.push(vec![]);
                    }

                    stacks[place].push(c);
                }
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    ParseResult {
        stacks: stacks,
        movements: movements,
    }
}

struct ParseResult {
    stacks: Vec<Vec<char>>,
    movements: Vec<(usize, usize, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!("CMZ", problem1(include_str!("test_puzzle.txt")));
        assert_eq!("TWSGQHNHL", problem1(include_str!("puzzle.txt")));

        assert_eq!("MCD", problem2(include_str!("test_puzzle.txt")));
        assert_eq!("JNRSCDWPP", problem2(include_str!("puzzle.txt")));
    }
}
