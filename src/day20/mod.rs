use std::collections::HashMap;

type Input = HashMap<String, Node>;

pub fn main() {
    println!("21");
}

fn problem1(mut input: Input) -> isize {
    let root = input.get("root").unwrap();
    println!("Root: {}", root.name);
    println!("Children: {:?} {:?}", root.left_child, root.right_child);
    println!("Operator: {:?}", root.operator);

    recurse_1(&input, "root")
}

fn recurse_1(input: &Input, node: &str) -> isize {
    let node = input.get(node).unwrap();
    if let Some(value) = node.value {
        return value;
    }

    if let (Some(lhs), Some(rhs), Some(op)) = (&node.left_child, &node.right_child, &node.operator) {
        let lval = recurse_1(input, lhs);
        let rval = recurse_1(input, rhs);
        return match op {
            Operator::Plus => lval + rval,
            Operator::Minus => lval - rval,
            Operator::Divide => lval / rval,
            Operator::Multiply => lval * rval,
        };
    }

    panic!("What is this?");
}

fn problem2(mut input: Input) -> isize {
    let root = input.get("root").unwrap();
    println!("Root: {}", root.name);
    println!("Children: {:?} {:?}", root.left_child, root.right_child);
    println!("Operator: {:?}", root.operator);

    recurse_2(&input, "root")
}

fn recurse_2(input: &Input, node: &str) -> isize {
    let node = input.get(node).unwrap();
    if let Some(value) = node.value {
        return value;
    }

    if let (Some(lhs), Some(rhs), Some(op)) = (&node.left_child, &node.right_child, &node.operator) {
        let lval = recurse_2(input, lhs);
        let rval = recurse_2(input, rhs);
        return match op {
            Operator::Plus => lval + rval,
            Operator::Minus => lval - rval,
            Operator::Divide => lval / rval,
            Operator::Multiply => lval * rval,
        };
    }

    panic!("What is this?");
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct Node {
    name: String,
    value: Option<isize>,
    left_child: Option<String>,
    right_child: Option<String>,
    operator: Option<Operator>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    #[test]
    fn test_problems_1() {
        assert_eq!(152, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(41857219607906, problem1(parse(include_str!("puzzle.txt"))));
    }

    #[test]
    fn test_problems_2() {
        assert_eq!(301, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(-1, problem2(parse(include_str!("puzzle.txt"))));
    }


    fn parse(s: &str) -> Input {
        let mut map = HashMap::new();
        s.lines().for_each(|line| {
            let init = line[0..4].to_string();
            let numbers = util::parse_numbers(line);
            if numbers.len() > 0 {
                map.insert(init.clone(), Node { name: init.clone(), value: Some(numbers[0]), left_child: None, right_child: None, operator: None });
            } else {
                let first = line[6..10].to_string();
                let second = line[13..17].to_string();

                map.insert(
                    init.clone(),
                    Node {
                        name: init.clone(),
                        value: None,
                        left_child: Some(first),
                        right_child: Some(second),
                        operator: Some(match &line[11..12] {
                            "-" => Operator::Minus,
                            "+" => Operator::Plus,
                            "/" => Operator::Divide,
                            "*" => Operator::Multiply,
                            _ => panic!("Unknown operator"),
                        }),
                    },
                );
            };
        });

        map
    }
}
