use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../../input/day_5.txt");

static MOVE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap());

static PARSE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\r?\n\r?\n").unwrap());

struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Command {
    fn from(input: &str) -> Self {
        let caps = MOVE_REGEX.captures(input).unwrap();
        Self {
            amount: caps.get(1).unwrap().as_str().parse().unwrap(),
            from: caps.get(2).unwrap().as_str().parse().unwrap(),
            to: caps.get(3).unwrap().as_str().parse().unwrap(),
        }
    }
}

struct Crates(Vec<VecDeque<char>>);

impl Crates {
    pub fn new(input: &str) -> Self {
        let mut rev_lines = input.lines().rev();
        let first_line = rev_lines.next().unwrap();
        let crates = (first_line.len() / 4) + 1;
        let mut stacks = vec![VecDeque::new(); crates];
        for line in rev_lines {
            Self::add_crates(line, &mut stacks);
        }

        Self(stacks)
    }

    fn add_crates(input: &str, stacks: &mut [VecDeque<char>]) {
        for i in (1..input.len()).step_by(4) {
            let c = input.chars().nth(i).unwrap();
            if c.is_ascii_uppercase() {
                stacks.get_mut(i / 4).unwrap().push_front(c);
            }
        }
    }

    pub fn move_crates_cratemover_9000(&mut self, commands: &str) {
        for command in commands.lines().map(Command::from) {
            for _ in 0..command.amount {
                if let Some(c) = self.0[command.from - 1].pop_front() {
                    self.0[command.to - 1].push_front(c);
                }
            }
        }
    }

    pub fn move_crates_cratemover_9001(&mut self, commands: &str) {
        for command in commands.lines().map(Command::from) {
            let mut v = VecDeque::new();
            for _ in 0..command.amount {
                if let Some(c) = self.0[command.from - 1].pop_front() {
                    v.push_front(c);
                }
            }
            while let Some(c) = v.pop_front() {
                self.0[command.to - 1].push_front(c);
            }
        }
    }

    pub fn top_crates(&self) -> String {
        self.0.iter().filter_map(|s| s.front()).collect()
    }
}

fn parse(input: &str) -> (&str, &str) {
    PARSE_REGEX.split(input).collect_tuple().unwrap()
}

fn main() {
    let (stacks, commands) = parse(INPUT);
    let mut crates = Crates::new(stacks);
    crates.move_crates_cratemover_9000(commands);
    println!("part 1: {:?}", crates.top_crates());

    crates = Crates::new(stacks);
    crates.move_crates_cratemover_9001(commands);
    println!("part 2: {:?}", crates.top_crates());
}

#[cfg(test)]
mod test {
    use crate::{parse, Crates};

    const TEST_INPUT: &str = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_part_1() {
        let (stacks, commands) = parse(TEST_INPUT);
        let mut crates = Crates::new(stacks);
        crates.move_crates_cratemover_9000(commands);
        assert_eq!("CMZ", crates.top_crates());
    }

    #[test]
    fn test_part_2() {
        let (stacks, commands) = parse(TEST_INPUT);
        let mut crates = Crates::new(stacks);
        crates.move_crates_cratemover_9001(commands);
        assert_eq!("MCD", crates.top_crates());
    }
}
