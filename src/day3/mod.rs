use std::{collections::HashSet, str::FromStr};

pub fn main() {
    println!("Day3");
}

fn problem1(vec: Vec<Rucksack>) -> usize {
    vec.iter().map(|r| r.union_value()).sum()
}

fn problem2(vec: Vec<Rucksack>) -> usize {
    let mut sum = 0;

    let mut i = 0;
    while i < vec.len() - 2 {
        let a: Vec<&Item> = vec[i].compartment_a.iter().chain(vec[i].compartment_b.iter()).collect();
        let b: Vec<&Item> = vec[i + 1].compartment_a.iter().chain(vec[i + 1].compartment_b.iter()).collect();
        let c: Vec<&Item> = vec[i + 2].compartment_a.iter().chain(vec[i + 2].compartment_b.iter()).collect();

        let mut seen = HashSet::new();

        for i1 in &a {
            for i2 in &b {
                for i3 in &c {
                    if i1.item_type.eq(&i2.item_type) && i2.item_type.eq(&i3.item_type) {
                        if seen.insert(i1.item_type.to_string()) {
                            println!("Found badge: {}", i1.item_type);
                            sum += i1.priority;
                        }
                    }
                }
            }
        }

        i += 3;
    }

    sum
}

struct Item {
    item_type: String,
    priority: usize,
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        let ascii = c as usize;
        let priority = if c.is_ascii_lowercase() { ascii - 97 + 1 } else { ascii - 65 + 27 };

        Item { item_type: c.to_string(), priority: priority }
    }
}

struct Rucksack {
    compartment_a: Vec<Item>,
    compartment_b: Vec<Item>,
}

impl Rucksack {
    fn union_value(&self) -> usize {
        let mut value = 0;

        let mut seen = HashSet::new();

        println!("Checking union: ");
        for a_item in &self.compartment_a {
            for b_item in &self.compartment_b {
                if a_item.item_type.eq(&b_item.item_type) {
                    if seen.insert(&a_item.item_type) {
                        println!(" {}, priority: {}", a_item.item_type, a_item.priority);
                        value += a_item.priority;
                    }

                    break;
                }
            }
        }

        value
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mid = line.len() / 2;
        let a = &line[0..mid];
        let b = &line[mid..line.len()];

        Ok(Rucksack { compartment_a: a.chars().map(|c| Item::from(c)).collect(), compartment_b: b.chars().map(|c| Item::from(c)).collect() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(157, problem1(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(8176, problem1(parse(include_str!("puzzle.txt"))));

        assert_eq!(70, problem2(parse(include_str!("test_puzzle.txt"))));
        assert_eq!(2689, problem2(parse(include_str!("puzzle.txt"))));
    }

    fn parse(str: &str) -> Vec<Rucksack> {
        str.split("\r\n").map(|line| Rucksack::from_str(line).unwrap()).collect()
    }
}
