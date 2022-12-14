use std::cmp::Ordering;

pub fn main() {
    println!("Day13");
}

fn process(pairs: &mut Vec<Pair>) {
    for (_, pair) in pairs.iter_mut().enumerate() {
        // println!("Repairing {}", pid + 1);
        let mut left_at = 0;
        let mut right_at = 0;

        loop {
            let left_option = pair.left.get(left_at);
            let right_option = pair.right.get(right_at);

            if let (Some(left), Some(right)) = (left_option, right_option) {
                // println!("inspecting: {:?} and {:?}", left, right);
                match (left, right) {
                    (Token::Open, Token::Num(_)) => {
                        pair.right.insert(right_at+1, Token::Close);
                        pair.right.insert(right_at, Token::Open);
                    },
                    (Token::Num(_), Token::Open) => {
                        pair.left.insert(left_at+1, Token::Close);
                        pair.left.insert(left_at, Token::Open);
                    },
                    _ => {}
                }
            } else {
                break;
            }

            left_at += 1;
            right_at += 1;
        }
    }
}

fn problem1(mut pairs: Vec<Pair>) -> usize {
    process(&mut pairs);

    let mut sum_of_right = 0;
    for (index, pair) in pairs.iter_mut().enumerate() {
        // println!("== Pair {} ==", (index + 1));
        if compare_pair(pair) {
            sum_of_right += index+1;
        }
    }

    sum_of_right
}

fn problem2(pairs: Vec<Pair>) -> usize {
    let mut all_lines = vec![];
    for pair in pairs {
        all_lines.push(pair.left);
        all_lines.push(pair.right);
    }

    let decoder_key_1 = vec![Token::Open, Token::Open, Token::Num(2), Token::Close, Token::Close];
    let decoder_key_2 = vec![Token::Open, Token::Open, Token::Num(6), Token::Close, Token::Close];

    all_lines.push(decoder_key_1.clone());
    all_lines.push(decoder_key_2.clone());

    all_lines.sort_by(|left,right| {
        let pair = Pair {left: left.clone(), right:right.clone()};
        let mut pairs = vec![pair];
        process(&mut pairs);

        if compare_pair(&pairs[0]) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    // println!("Organized:");
    // for packet in all_lines.iter() {
    //     println!("{:?}", packet);
    // }

    let key_1_index = all_lines.iter().position(|v| v.eq(&decoder_key_1)).unwrap() + 1;
    let key_2_index = all_lines.iter().position(|v| v.eq(&decoder_key_2)).unwrap() + 1;

    key_1_index * key_2_index
}

fn compare_pair(pair: &Pair) -> bool {
    let mut left_iter = pair.left.iter();
    let mut right_iter = pair.right.iter();

    loop {
        match (left_iter.next(), right_iter.next()) {
            (Some(left_token), Some(right_token)) => match (left_token, right_token) {
                (Token::Num(l), Token::Num(r)) => {
                    // println!("  - Compare {} vs {}", l, r);
                    if l < r {
                        // println!("    - Left side is smaller, so inputs are in the right order");
                        return true;
                    } else if l > r {
                        // println!("    - Right side is smaller, so inputs are not in the right order");
                        return false;
                    }
                }
                (Token::Close, Token::Num(_)) => {
                    // println!("    - Left side ran out of items, so inputs are in the right order");
                    return true;                    
                },
                (Token::Close, Token::Open) => {
                    // println!("    - Left side ran out of items, so inputs are in the right order");
                    return true;                    
                },

                (Token::Num(_), Token::Close) => {
                    // println!("    - Right side ran out of items, so inputs are not in the right order");
                    return false;                    
                },
                (Token::Open, Token::Close) => {
                    // println!("    - Right side ran out of items, so inputs are not in the right order");
                    return false;                    
                }
                (Token::Close, Token::Close) => {}
                (Token::Open, Token::Open) => {}
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

struct Pair {
    left: Vec<Token>,
    right: Vec<Token>,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Open,
    Close,
    Num(usize),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_problems() {
        assert_eq!(13, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(4734, problem1(parse(include_str!("puzzle.txt"))));

        assert_eq!(140, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(21836, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse_line(s: &str) -> Vec<Token> {
        let mut tokens = vec![];

        let bytes: Vec<u8> = s.bytes().collect();
        for i in 0..bytes.len() {
            match bytes[i] {
                b'[' => tokens.push(Token::Open),
                b']' => tokens.push(Token::Close),
                b if b.is_ascii_digit() => {
                    if bytes[i + 1].is_ascii_digit() {
                        tokens.push(Token::Num((b - 48_u8) as usize * 10 + (bytes[i + 1] - 48) as usize))
                    } else {
                        tokens.push(Token::Num((b - 48_u8) as usize))
                    }
                }
                b',' => {}
                b => panic!("Unknown token: {}", b),
            }
        }

        tokens
    }

    fn parse(s: &str) -> Vec<Pair> {
        let mut pairs = vec![];
        let mut lines = s.lines();

        loop {
            if let Some(left) = lines.next() {
                let left = parse_line(&left);
                let right = parse_line(&lines.next().unwrap());
                lines.next();
                pairs.push(Pair { left, right });
            } else {
                break;
            }
        }

        pairs
    }
}
