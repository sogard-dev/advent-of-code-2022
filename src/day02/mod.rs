use std::str::FromStr;

pub fn main() {
    println!("Day2");
}

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(input: &str) -> Result<Hand, Self::Err> {
        match input {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            "X" => Ok(Hand::Rock),
            "Y" => Ok(Hand::Paper),
            "Z" => Ok(Hand::Scissors),
            _ => Err(()),
        }
    }
}

fn game(them: &Hand, us: &Hand) -> Outcome {
    match them {
        Hand::Paper => match us {
            Hand::Rock => Outcome::Lose,
            Hand::Paper => Outcome::Draw,
            Hand::Scissors => Outcome::Win,
        },
        Hand::Rock => match us {
            Hand::Rock => Outcome::Draw,
            Hand::Paper => Outcome::Win,
            Hand::Scissors => Outcome::Lose,
        },
        Hand::Scissors => match us {
            Hand::Rock => Outcome::Win,
            Hand::Paper => Outcome::Lose,
            Hand::Scissors => Outcome::Draw,
        },
    }
}

fn points(us: &Hand, outcome: &Outcome) -> usize {
    let hand_points = match us {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    };

    let outcome_points = match outcome {
        Outcome::Win => 6,
        Outcome::Lose => 0,
        Outcome::Draw => 3,
    };

    hand_points + outcome_points
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

fn pick_hand(them: &Hand, outcome: &Outcome) -> Hand {
    if game(them, &Hand::Paper) == *outcome {
        return Hand::Paper;
    }
    if game(them, &Hand::Scissors) == *outcome {
        return Hand::Scissors;
    }
    if game(them, &Hand::Rock) == *outcome {
        return Hand::Rock;
    }
    panic!("Shit");
}

fn puzzle_1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut sum = 0;
    for s in lines {
        let spl: Vec<&str> = s.split(" ").collect();

        let us = Hand::from_str(spl[1]).unwrap();
        let them = Hand::from_str(spl[0]).unwrap();
        let outcome = game(&them, &us);
        let points = points(&us, &outcome);
        sum += points;
    }

    sum
}

fn puzzle_2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut sum = 0;
    for s in lines {
        let spl: Vec<&str> = s.split(" ").collect();

        let them = Hand::from_str(spl[0]).unwrap();
        let expected_outcome = Outcome::from_str(spl[1]).unwrap();

        let my_hand = pick_hand(&them, &expected_outcome);

        let outcome = game(&them, &my_hand);
        let points = points(&my_hand, &outcome);

        sum += points;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day02::*;

    #[test]
    fn it_works() {
        assert_eq!(15, puzzle_1(include_str!("puzzle_1_test.txt")));
        assert_eq!(9177, puzzle_1(include_str!("puzzle_1.txt")));

        assert_eq!(12, puzzle_2(include_str!("puzzle_1_test.txt")));
        assert_eq!(12111, puzzle_2(include_str!("puzzle_1.txt")));
    }
}
