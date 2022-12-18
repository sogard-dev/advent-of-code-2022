pub fn main() {
    println!("Day6");
}

fn problem1(s: &str) -> usize {
    get_start(s, 4)
}

fn problem2(s: &str) -> usize {
    get_start(s, 14)
}

fn get_start(s: &str, n: usize) -> usize {
    let mut last_n = vec![];

    for (index, c) in s.chars().enumerate() {
        last_n.push(c);

        if last_n.len() > n {
            last_n.remove(0);
        }

        let mut dup = last_n.clone();
        dup.sort();
        dup.dedup();

        if dup.len() == n {
            return index + 1;
        }
    }

    panic!("Did not find!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(7, problem1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, problem1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, problem1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, problem1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, problem1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
        assert_eq!(1833, problem1(include_str!("puzzle.txt")));

        assert_eq!(19, problem2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, problem2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, problem2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, problem2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, problem2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
        assert_eq!(3425, problem2(include_str!("puzzle.txt")));
    }
}
