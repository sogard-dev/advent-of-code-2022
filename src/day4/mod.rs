use std::str::FromStr;

pub fn main() {
    println!("Day4");
}

fn problem1(s: &str) -> usize {
    let pairs = parse(s);

    pairs
        .iter()
        .map(|p| if p.fully_overlap() { 1 } else { 0 })
        .sum()
}

fn problem2(s: &str) -> usize {
    let pairs = parse(s);

    pairs
        .iter()
        .map(|p| if p.partial_overlap() { 1 } else { 0 })
        .sum()
}

fn parse(s: &str) -> Vec<Pair> {
    s.split("\r\n")
        .map(|line| Pair::from_str(line).unwrap())
        .collect()
}

struct Section {
    from: usize,
    to: usize,
}

impl FromStr for Section {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl: Vec<&str> = s.split("-").collect();

        Ok(Section {
            from: spl[0].parse::<usize>().unwrap(),
            to: spl[1].parse::<usize>().unwrap(),
        })
    }
}

struct Pair {
    a: Section,
    b: Section,
}

impl Pair {
    fn partial_overlap(&self) -> bool {
        let a = self.a.from;
        let b = self.a.to;
        let c = self.b.from;
        let d = self.b.to;

        if b < c || d < a {
            return false;
        }

        true
    }

    fn fully_overlap(&self) -> bool {
        if self.a.from >= self.b.from && self.a.to <= self.b.to {
            return true;
        }
        if self.b.from >= self.a.from && self.b.to <= self.a.to {
            return true;
        }

        false
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl: Vec<&str> = s.split(",").collect();

        Ok(Pair {
            a: Section::from_str(spl[0]).unwrap(),
            b: Section::from_str(spl[1]).unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(2, problem1(include_str!("test_puzzle.txt")));
        assert_eq!(573, problem1(include_str!("puzzle.txt")));

        assert_eq!(4, problem2(include_str!("test_puzzle.txt")));
        assert_eq!(867, problem2(include_str!("puzzle.txt")));
    }
}
