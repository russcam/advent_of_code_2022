const INPUT: &str = include_str!("../../input/day_2.txt");

#[derive(Eq, PartialEq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn parse_opp(input: &str) -> Self {
        match input {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!("unknown opp value"),
        }
    }

    pub fn wins_against(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    pub fn loses_against(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

struct Game((Shape, Shape));

impl Game {
    pub fn score(&self) -> usize {
        self.score_shape() + self.score_win()
    }

    fn score_shape(&self) -> usize {
        match self.0 .1 {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn score_win(&self) -> usize {
        match &self.0 {
            (Shape::Rock, Shape::Scissors) => 0,
            (Shape::Rock, Shape::Paper) => 6,
            (Shape::Scissors, Shape::Paper) => 0,
            (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Paper, Shape::Rock) => 0,
            (Shape::Paper, Shape::Scissors) => 6,
            _ => 3,
        }
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut inputs = line.split(' ');
            let opp = Shape::parse_opp(inputs.next().unwrap());
            let me = match inputs.next().unwrap() {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissors,
                _ => panic!("unknown me value"),
            };
            Game((opp, me)).score()
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut inputs = line.split(' ');
            let opp = Shape::parse_opp(inputs.next().unwrap());
            let me = match inputs.next().unwrap() {
                "X" => opp.wins_against(),
                "Y" => opp,
                "Z" => opp.loses_against(),
                _ => panic!("unknown outcome value"),
            };
            Game((opp, me)).score()
        })
        .sum()
}

fn main() {
    println!("part 1 score: {}", part_1(INPUT));
    println!("part 2 score: {}", part_2(INPUT));
}

#[cfg(test)]
mod test {
    use super::{part_1, part_2};

    const TEST_INPUT: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 12);
    }
}
