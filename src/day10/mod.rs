use std::str::FromStr;

pub fn main() {
    println!("Day10");
}

enum Instruction {
    NOOP,
    AddX(i32),
}

impl Instruction {
    fn cycles(&self) -> i32 {
        match self {
            Instruction::NOOP => 0,
            Instruction::AddX(_) => 1,
        }
    }

    fn execute(&self, prev_value: i32) -> i32 {
        match self {
            Instruction::NOOP => prev_value,
            Instruction::AddX(v) => prev_value + v,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl: Vec<&str> = s.split(" ").collect();
        match spl[0] {
            "noop" => Ok(Instruction::NOOP),
            "addx" => Ok(Instruction::AddX(spl[1].parse::<i32>().unwrap())),
            _ => Err(()),
        }
    }
}

fn parse(s: &str) -> Vec<Instruction> {
    s.lines().map(|l| Instruction::from_str(l).unwrap()).collect()
}

fn problem1(s: &str) -> i32 {
    let instructions = parse(s);
    let snapshots = vec![20, 60, 100, 140, 180, 220];

    let mut register_x: i32 = 1;

    let mut sum_of_snapshots = 0;

    let mut current_cycle: i32 = 1;
    let mut execute_at_cycle: i32;

    for instruction in instructions {
        execute_at_cycle = instruction.cycles() + current_cycle;

        while execute_at_cycle != current_cycle {
            current_cycle += 1;
            if snapshots.contains(&current_cycle) {
                sum_of_snapshots += register_x * current_cycle;
            }
        }

        register_x = instruction.execute(register_x);
        current_cycle += 1;
        if snapshots.contains(&current_cycle) {
            sum_of_snapshots += register_x * current_cycle;
        }
    }

    sum_of_snapshots
}

fn problem2(s: &str) -> String {
    let instructions = parse(s);
    let snapshots = vec![41, 81, 121, 161, 201];

    let mut register_x: i32 = 1;

    let mut current_cycle: i32 = 1;
    let mut execute_at_cycle: i32;
    let mut y = 0;

    let mut display = [[0u8; 40]; 6];

    for instruction in instructions {
        execute_at_cycle = current_cycle + instruction.cycles();

        loop {
            if snapshots.contains(&current_cycle) {
                y += 1;
            }

            //Start cycle

            //During cycle
            let display_column = (current_cycle - 1) % 40;
            if (register_x - display_column).abs() <= 1 {
                display[y][(display_column) as usize] = 1;
            }

            //End of cycle
            if execute_at_cycle == current_cycle {
                register_x = instruction.execute(register_x);
                current_cycle += 1;
                break;
            }

            current_cycle += 1;
        }
    }

    let display_lines: Vec<String> = display
        .iter()
        .map(|line| {
            let line_display_items: Vec<&str> = line.iter().map(|p| if *p == 1 { "#" } else { "." }).collect();
            line_display_items.join("")
        })
        .collect();

    display_lines.join("\r\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problems() {
        assert_eq!(13140, problem1(include_str!("test_puzzle.txt")));
        assert_eq!(14560, problem1(include_str!("puzzle.txt")));

        assert_eq!(include_str!("test_puzzle_output.txt"), problem2(include_str!("test_puzzle.txt")));
        assert_eq!(include_str!("puzzle_output.txt"), problem2(include_str!("puzzle.txt")));
    }
}
