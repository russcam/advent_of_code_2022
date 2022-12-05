use std::cmp::Ordering;

const INPUT: &str = include_str!("../../input/day_4.txt");

pub struct SectionAssignment {
    from: i32,
    to: i32,
}

impl From<&str> for SectionAssignment {
    fn from(input: &str) -> Self {
        let mut parts = input.split('-');
        Self {
            from: parts.next().unwrap().parse().unwrap(),
            to: parts.next().unwrap().parse().unwrap(),
        }
    }
}

impl SectionAssignment {
    pub fn fully_overlaps(&self, other: &SectionAssignment) -> bool {
        match self.from.cmp(&other.from) {
            Ordering::Less => matches!(self.to.cmp(&other.to), Ordering::Greater | Ordering::Equal),
            Ordering::Greater => matches!(self.to.cmp(&other.to), Ordering::Less | Ordering::Equal),
            Ordering::Equal => true,
        }
    }

    pub fn partially_overlaps(&self, other: &SectionAssignment) -> bool {
        !(self.to < other.from || other.to < self.from)
    }
}

pub fn parse(input: &str) -> Vec<(SectionAssignment, SectionAssignment)> {
    input
        .lines()
        .map(|l| {
            let mut sections = l.split(',').map(SectionAssignment::from);
            (sections.next().unwrap(), sections.next().unwrap())
        })
        .collect()
}

fn main() {
    let pairs = parse(INPUT);

    let count = pairs
        .iter()
        .filter(|(fst, snd)| fst.fully_overlaps(snd))
        .count();

    println!("full overlap count: {}", count);

    let count = pairs
        .iter()
        .filter(|(fst, snd)| fst.partially_overlaps(snd))
        .count();

    println!("partial overlap count: {}", count);
}

#[cfg(test)]
mod test {
    use super::parse;

    const TEST_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_part_1() {
        let count = parse(TEST_INPUT)
            .iter()
            .filter(|(fst, snd)| fst.fully_overlaps(snd))
            .count();

        assert_eq!(count, 2);
    }

    #[test]
    fn test_part_2() {
        let count = parse(TEST_INPUT)
            .iter()
            .filter(|(fst, snd)| fst.partially_overlaps(snd))
            .count();

        assert_eq!(count, 4);
    }
}
