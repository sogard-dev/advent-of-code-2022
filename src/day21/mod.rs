use std::collections::HashMap;

type Input = HashMap<String, Node>;

pub fn main() {
    println!("21");
}

fn problem1(input: Input) -> isize {
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
    let mut value_map = HashMap::new();

    input.insert("humn".to_string(), Node { name: "humn".to_string(), value: None, left_child: None, right_child: None, operator: None });

    recurse_2(&input, "root", &mut value_map);

    let root = input.get("root").unwrap();
    if let Some(v) = value_map.get(root.left_child.as_ref().unwrap()) {
        value_map.insert("root".to_string(), *v);
    }
    if let Some(v) = value_map.get(root.right_child.as_ref().unwrap()) {
        value_map.insert("root".to_string(), *v);
    }

    let root_value = *value_map.get("root").unwrap();

    print(&input, &value_map);

    recurse_3(&input, "root", &mut value_map, root_value);
    print(&input, &value_map);

    *value_map.get("humn").unwrap()
}

fn print(input: &Input, value_map: &HashMap<String, isize>) {
    for (k,v) in input.iter() {
        if let Some(num) = value_map.get(k) {
            println!("Key: {}, value_map: {}", k, num);
        } else if v.left_child.is_some() {
            println!("Key: {}, exp: {} {:?} {}", k, v.left_child.as_ref().unwrap(), v.operator.as_ref().unwrap(), v.right_child.as_ref().unwrap());
        } else if v.value.is_some(){
            println!("Key: {}, value: {}", k, v.value.unwrap());
        } else {
            println!("Key: {}, value: ?", k);
        }
    }
    println!("");
}

fn recurse_2(input: &Input, node_str: &str, value_map: &mut HashMap<String, isize>) -> Option<isize> {
    if node_str.eq("humn") {
        return None;
    }

    let node = input.get(node_str).unwrap();
    if let Some(value) = node.value {
        value_map.insert(node_str.to_string(), value);
        return Some(value);
    }

    if let (Some(lhs), Some(rhs), Some(op)) = (&node.left_child, &node.right_child, &node.operator) {
        if let (Some(lval), Some(rval)) = (recurse_2(input, lhs, value_map), recurse_2(input, rhs, value_map)) {
            let value = match op {
                Operator::Plus => lval + rval,
                Operator::Minus => lval - rval,
                Operator::Divide => lval / rval,
                Operator::Multiply => lval * rval,
            };
    
            value_map.insert(node_str.to_string(), value);
            return Some(value);
        }
    }

    None
}

fn recurse_3(input: &Input, node_str: &str, value_map: &mut HashMap<String, isize>, parent_value: isize) {
    let node = input.get(node_str).unwrap();

    println!("Looking at {}", node_str);

    if let (Some(lhs), Some(rhs), Some(op)) = (&node.left_child, &node.right_child, &node.operator) {
        let (lopt, ropt) = (value_map.get(lhs), value_map.get(rhs));

        let value = {
            if node_str.eq("root") {
                parent_value
            } else {
                match (lopt, ropt) {
                    (None, Some(rval)) => {
                        match op {
                            Operator::Plus => parent_value - rval,
                            Operator::Minus => parent_value + rval,
                            Operator::Divide => parent_value * rval,
                            Operator::Multiply => parent_value / rval,
                        }
                    },
                    (Some(lval), None) => match op {
                        Operator::Plus => parent_value - lval,
                        Operator::Minus => lval - parent_value,
                        Operator::Divide => lval / parent_value,
                        Operator::Multiply => parent_value / lval,
                    },
                    _ => return
                }
            }
        };
                
        if lopt.is_some() {
            println!("  Calculating {} to be {}", rhs, value);
            value_map.insert(rhs.to_string(), value);
        } else {
            println!("  Calculating {} to be {}", lhs, value);
            value_map.insert(lhs.to_string(), value);
        }

        recurse_3(&input, lhs, value_map, value);
        recurse_3(&input, rhs, value_map, value);
        
    }
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
        assert_eq!(3916936880448, problem2(parse(include_str!("puzzle.txt"))));
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
