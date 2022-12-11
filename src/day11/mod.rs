pub fn main() {
    println!("Day11");
}

type Worry = usize;

fn problem1(mut monkeys: Vec<Monkey>) -> usize {
    solve(monkeys, 20, 3)
}

fn problem2(mut monkeys: Vec<Monkey>) -> usize {
    solve(monkeys, 10000, 1)
}

fn solve(mut monkeys: Vec<Monkey>, rounds: usize, worry_division: Worry) -> usize {
    let divisible = monkeys.iter().map(|m| m.test_divisible).fold(1, |acum, item| acum * item);

    for round in 1..=rounds {
        //println!("Round {}", round);

        for index in 0..monkeys.len() {
            //println!("  Monkey {}:", index);

            let mut throw_to: Vec<(usize, Worry)> = vec![];

            loop {
                let monkey = &mut monkeys[index];
                if let Some(worry) = monkey.starting_items.first() {
                    //println!("    Monkey inspects an item with a worry level of {}.", worry);

                    let worry_after_operation = monkey.operation.as_ref()(*worry);
                    //println!("      Worry level is changed to {}.", worry_after_operation);

                    let worry_after_bored = (worry_after_operation / worry_division) % divisible;
                    //println!("      Monkey gets bored with item. Worry level is divided by 3 to {}.", worry_after_bored);

                    let throw_boolean = worry_after_bored % monkey.test_divisible == 0;
                    if throw_boolean {
                        //println!("      Current worry level is divisible by {}", monkey.test_divisible);
                        throw_to.push((monkey.throw_true, worry_after_bored));
                    } else {
                        //println!("      Current worry level is not divisible by {}", monkey.test_divisible);
                        throw_to.push((monkey.throw_false, worry_after_bored));
                    }

                    monkey.inspected += 1;

                    monkey.starting_items.remove(0);
                } else {
                    break;
                }
            }

            for (monkey_index, worry) in throw_to {
                monkeys[monkey_index].starting_items.push(worry);
            }
        }

        for (index, monkey) in monkeys.iter().enumerate() {
            //println!("Monkey {}: {:?}", index, monkey.starting_items);
        }

        //println!("");
    }

    monkeys.iter().enumerate().for_each(|(idx, monkey)| {
        //println!("Monkey {} inspected items {} times.", idx, monkey.inspected);
    });

    let mut inspected: Vec<usize> = monkeys.iter().map(|m| m.inspected).collect();
    inspected.sort();
    inspected.reverse();

    inspected[0] * inspected[1]
}

struct Monkey {
    starting_items: Vec<Worry>,
    operation: Box<dyn Fn(Worry) -> Worry>,
    test_divisible: Worry,
    throw_true: usize,
    throw_false: usize,
    inspected: usize
}

impl Monkey {
    fn new(starting_items: Vec<Worry>, operation: impl Fn(Worry) -> Worry + 'static, test_divisible: Worry, throw_true: usize, throw_false: usize) -> Monkey {
        Monkey { starting_items, operation: Box::new(operation), test_divisible, throw_true, throw_false, inspected: 0 }
    }
}

enum Operator {
    Multiply,
    Plus
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_problems() {          
        assert_eq!(10605, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(113220, problem1(parse(include_str!("puzzle.txt"))));

        assert_eq!(2713310158, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(30599555965, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse(s: &str) -> Vec<Monkey> {
        let s = s.replace("\n", "");
        let s = s.replace("\r", "");
        let s = s.replace(" ", "");
        
        let re = Regex::new(r"Monkey\d:Startingitems:([\d,]+)Operation:new=([old\d]+)([+*])([old\d]+)+Test:divisibleby(\d+)Iftrue:throwtomonkey(\d+)Iffalse:throwtomonkey(\d+)").unwrap();

        let mut monkeys = vec![];

        for capture in re.captures_iter(&s) {
            let starting_items = capture[1].split(",").map(|v| v.parse::<Worry>().unwrap()).collect();
            let test_divisible = capture[5].parse::<Worry>().unwrap();
            let throw_true = capture[6].parse::<usize>().unwrap();
            let throw_false = capture[7].parse::<usize>().unwrap();

            let operand_1 = capture[2].to_string();
            let operator_string = &capture[3];
            let operand_2 = capture[4].to_string();

            let operator = {
                if operator_string.contains("*") {
                    Operator::Multiply
                } else if operator_string.contains("+") {
                    Operator::Plus
                } else {
                    panic!("Shit");
                }
            };


            let operation = move |old: Worry| {
                let value_1 = old;
                let value_2 = {
                    if operand_2.contains("old") {
                        old
                    } else {
                        operand_2.parse::<Worry>().unwrap()
                    }
                };
                match operator {
                    Operator::Multiply => value_1 * value_2,
                    Operator::Plus => value_1 + value_2,
                }
            };

            
            let monkey = Monkey::new(starting_items, operation, test_divisible, throw_true, throw_false);
            monkeys.push(monkey);
        }

        monkeys
    }
}
