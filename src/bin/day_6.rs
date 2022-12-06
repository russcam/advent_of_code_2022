use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day_6.txt");

struct DataStream<'a>(&'a str);

impl<'a> DataStream<'a> {
    pub fn first_marker(&self, distinct_chars: usize) -> usize {
        let mut min = 0;
        let mut max = distinct_chars - 1;
        while max < self.0.len() {
            let set: HashSet<u8> = HashSet::from_iter(self.0[min..=max].bytes());
            if set.len() == distinct_chars {
                return max + 1;
            }
            min += 1;
            max += 1;
        }

        unreachable!("stream must contain a first marker");
    }
}

impl<'a> From<&'a str> for DataStream<'a> {
    fn from(input: &'a str) -> Self {
        Self(input)
    }
}

fn main() {
    let buf = DataStream::from(INPUT);
    println!("start-of-packet marker: {}", buf.first_marker(4));
    println!("start-of-message marker: {}", buf.first_marker(14));
}

#[cfg(test)]
mod test {
    use crate::DataStream;
    use std::collections::HashSet;

    const TEST_INPUT: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;

    #[test]
    fn test_part_1() {
        let buf = DataStream::from(TEST_INPUT);
        assert_eq!(7, buf.first_marker(4));
    }

    #[test]
    fn test_part_2() {
        let buf = DataStream::from(TEST_INPUT);
        assert_eq!(19, buf.first_marker(14));
    }
}
