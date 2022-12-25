type Input = Vec<SNAFU>;

pub fn main() {
    println!("25");
}

fn problem1(snafus: Input) -> SNAFU {
    let sum = snafus.iter().map(|s| s.to_decimal()).fold(0, |acc, v| acc + v);
    SNAFU::from_decimal(sum)
}

fn problem2(storms: Input) -> usize {
    0
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct SNAFU {
    parts: Vec<isize>,
}

impl SNAFU {
    fn from_str(s: &str) -> SNAFU {
        let mut parts = vec![];
        for c in s.chars() {
            parts.push(match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("Unknown"),
            });
        }

        SNAFU { parts }
    }

    fn to_decimal(&self) -> isize {
        let mut sum = 0;
        let mut mult = 1;
        for num in self.parts.iter().rev() {
            sum += num * mult;
            mult = mult * 5;
        }

        sum
    }
    fn to_string(&self) -> String {
        let mut o = String::new();
        for p in self.parts.iter() {
            o.push(match p {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => panic!("Shit"),
            })
        }

        o
    }

    fn increment(&self) -> SNAFU {
        let mut new_parts = self.parts.clone();
        for i in (0..new_parts.len()).rev() {
            new_parts[i] += 1;

            if new_parts[i] == 3 {
                new_parts[i] = -2;
            } else {
                return SNAFU { parts: new_parts };
            }
        }

        new_parts.insert(0, 1);

        SNAFU { parts: new_parts }
    }

    fn from_decimal(dec: isize) -> SNAFU {
        let mut snafu = SNAFU { parts: vec![] };

        while snafu.to_decimal() < dec {
            snafu.parts.push(2);
        }

        for i in (0..snafu.parts.len()) {
            for new_val in [1, 0, -1, -2] {
                snafu.parts[i] = new_val;
                if snafu.to_decimal() < dec {
                    snafu.parts[i] = new_val + 1;
                    break;
                }
            }
        }

        if snafu.to_decimal() != dec {
            panic!("no no no");
        }

        snafu
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_1() {
        assert_eq!("2=-1=0", problem1(parse(include_str!("test_puzzle.txt"))).to_string());
        assert_eq!("2=1-=02-21===-21=200", problem1(parse(include_str!("puzzle.txt"))).to_string());
    }

    #[test]
    fn test_to_decimal() {
        assert_eq!(1, SNAFU::from_str("1").to_decimal());
        assert_eq!(2, SNAFU::from_str("2").to_decimal());
        assert_eq!(3, SNAFU::from_str("1=").to_decimal());
        assert_eq!(4, SNAFU::from_str("1-").to_decimal());
        assert_eq!(5, SNAFU::from_str("10").to_decimal());
        assert_eq!(6, SNAFU::from_str("11").to_decimal());
        assert_eq!(7, SNAFU::from_str("12").to_decimal());
        assert_eq!(8, SNAFU::from_str("2=").to_decimal());
        assert_eq!(9, SNAFU::from_str("2-").to_decimal());
        assert_eq!(10, SNAFU::from_str("20").to_decimal());
        assert_eq!(15, SNAFU::from_str("1=0").to_decimal());
        assert_eq!(20, SNAFU::from_str("1-0").to_decimal());
        assert_eq!(2022, SNAFU::from_str("1=11-2").to_decimal());
        assert_eq!(12345, SNAFU::from_str("1-0---0").to_decimal());
        assert_eq!(314159265, SNAFU::from_str("1121-1110-1=0").to_decimal());
    }

    #[test]
    fn test_increment() {
        assert_eq!(1, SNAFU::from_str("0").increment().to_decimal());
        assert_eq!(11, SNAFU::from_str("20").increment().to_decimal());
        assert_eq!(2023, SNAFU::from_str("1=11-2").increment().to_decimal());
    }

    fn parse(s: &str) -> Vec<SNAFU> {
        s.lines().map(|line| SNAFU::from_str(line)).collect()
    }
}
