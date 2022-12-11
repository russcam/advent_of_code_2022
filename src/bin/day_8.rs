use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day_8.txt");

struct Grid(Vec<Vec<u32>>);

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
                .collect(),
        )
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Grid {
    pub fn visible_from_outside(&self) -> usize {
        let mut count = 0;
        for (x, y) in self.coords() {
            if x == 0 || y == 0 || x - 1 == self.column_len(y) || y - 1 == self.row_len() {
                count += 1;
                continue;
            }
            let height = self.cell_height(x, y);
            if [
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ]
            .into_iter()
            .any(|d| self.values(x, y, d).all(|other| other < height))
            {
                count += 1;
            }
        }
        count
    }

    fn values(
        &self,
        x: usize,
        y: usize,
        direction: Direction,
    ) -> Box<dyn Iterator<Item = u32> + '_> {
        match direction {
            Direction::Left => Box::new((0..x).rev().map(move |xx| self.cell_height(xx, y))),
            Direction::Right => {
                Box::new((x + 1..self.0[y].len()).map(move |xx| self.cell_height(xx, y)))
            }
            Direction::Up => Box::new((0..y).rev().map(move |yy| self.cell_height(x, yy))),
            Direction::Down => {
                Box::new((y + 1..self.0.len()).map(move |yy| self.cell_height(x, yy)))
            }
        }
    }

    fn row_len(&self) -> usize {
        self.0.len()
    }

    fn column_len(&self, y: usize) -> usize {
        self.0[y].len()
    }

    fn cell_height(&self, x: usize, y: usize) -> u32 {
        self.0[y][x]
    }

    fn coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.row_len()).flat_map(|y| {
            let columns = self.column_len(y);
            (0..columns).map(move |x| (x, y))
        })
    }

    pub fn score(&self) -> usize {
        let mut max_score = 0;
        for (x, y) in self.coords() {
            if x == 0 || y == 0 || x - 1 == self.column_len(y) || y - 1 == self.row_len() {
                continue;
            }
            let height = self.cell_height(x, y);
            let score = [
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ]
            .map(|d| {
                self.values(x, y, d)
                    .fold_while(0, |acc, other| {
                        if height <= other {
                            Done(acc + 1)
                        } else {
                            Continue(acc + 1)
                        }
                    })
                    .into_inner()
            })
            .iter()
            .product();

            if score > max_score {
                max_score = score;
            }
        }

        max_score
    }
}

fn main() {
    let grid = Grid::from(INPUT);
    println!("part 1: {}", grid.visible_from_outside());
    println!("part 2: {}", grid.score());
}

#[cfg(test)]
mod test {
    use crate::Grid;

    const TEST_INPUT: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_part_1() {
        let grid = Grid::from(TEST_INPUT);
        assert_eq!(21, grid.visible_from_outside());
    }

    #[test]
    fn test_part_2() {
        let grid = Grid::from(TEST_INPUT);
        assert_eq!(8, grid.score());
    }
}
