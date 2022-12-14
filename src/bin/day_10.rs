use std::collections::{HashMap, VecDeque};
use std::fmt::Write;

const INPUT: &str = include_str!("../../input/day_10.txt");

enum Instruction {
    Noop,
    Addx(i32),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        if input == "noop" {
            Instruction::Noop
        } else {
            Instruction::Addx(input.strip_prefix("addx ").unwrap().parse().unwrap())
        }
    }
}

struct Program {
    x: i32,
    stack: VecDeque<i32>,
    record_cycles: HashMap<i32, i32>,
    cycle: i32,
    screens: Vec<Vec<char>>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            x: 1,
            stack: VecDeque::new(),
            record_cycles: HashMap::new(),
            cycle: 0,
            screens: vec![],
        }
    }

    pub fn execute(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.record();
            match instruction {
                Instruction::Noop => {
                    self.stack.push_back(0);
                }
                Instruction::Addx(addx) => {
                    self.stack.push_back(0);
                    self.stack.push_back(*addx);
                }
            }
            self.cycle();
        }

        while !self.stack.is_empty() {
            self.record();
            self.cycle();
        }
    }

    fn record(&mut self) {
        if self.cycle + 1 == 20 || (self.cycle + 1 - 20) % 40 == 0 {
            self.record_cycles
                .insert(self.cycle + 1, self.signal_strength());
        }

        if self.cycle % 40 == 0 {
            self.screens.push(Vec::with_capacity(40));
        }
        let screen_multiplier = (self.screens.len() as i32 - 1) * 40;
        let row = self.screens.last_mut().unwrap();
        match self.cycle {
            c if c == self.x - 1 + screen_multiplier
                || c == self.x + screen_multiplier
                || c == self.x + 1 + screen_multiplier =>
            {
                row.push('#')
            }
            _ => row.push('.'),
        }
    }

    fn cycle(&mut self) {
        self.cycle += 1;
        self.x += self.stack.pop_front().unwrap();
    }

    fn signal_strength(&self) -> i32 {
        self.x * (self.cycle + 1)
    }

    pub fn sum_signal_strength(&self) -> i32 {
        self.record_cycles.values().sum()
    }

    pub fn screen(&self) -> String {
        let mut screen = String::new();
        for line in &self.screens {
            writeln!(&mut screen, "{}", String::from_iter(line)).unwrap();
        }
        screen
    }
}

fn main() {
    let instructions = INPUT.lines().map(Instruction::from).collect::<Vec<_>>();
    let mut program = Program::new();
    program.execute(&instructions);
    println!("part 1: {}", program.sum_signal_strength());
    println!("part 2:");
    println!("{}", program.screen());
}

#[cfg(test)]
mod test {
    use crate::{Instruction, Program};

    const TEST_INPUT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn test_part_1() {
        let instructions = TEST_INPUT
            .lines()
            .map(Instruction::from)
            .collect::<Vec<_>>();
        let mut program = Program::new();
        program.execute(&instructions);
        assert_eq!(13140, program.sum_signal_strength())
    }

    #[test]
    fn test_part_2() {
        let instructions = TEST_INPUT
            .lines()
            .map(Instruction::from)
            .collect::<Vec<_>>();
        let mut program = Program::new();
        program.execute(&instructions);
        assert_eq!(
            r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#,
            program.screen()
        );
    }
}
