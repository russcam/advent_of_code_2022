use std::collections::{HashMap, HashSet};
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day_3.txt");

fn priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        (c as usize) - 96
    } else {
        (c as usize) - 38
    }
}

fn part_1(input: &str) -> usize {
    input.lines()
        .map(|l| { l.split_at(l.len() / 2) })
        .map(|(fst, snd)| {
            let map = fst.chars().map(|c| (c, priority(c))).collect::<HashMap<_, _>>();
            for c in snd.chars() {
                if let Some(p) = map.get(&c) {
                    return *p;
                }
            }
            0usize
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input.lines()
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let (fst, snd, thd) =  chunk.next_tuple().unwrap();
            let fst_set = fst.chars().collect::<HashSet<_>>();
            let thd_map = thd.chars().map(|c| (c, priority(c))).collect::<HashMap<_, _>>();
            for c in snd.chars() {
                if fst_set.contains(&c) {
                    if let Some(p) = thd_map.get(&c) {
                        return *p;
                    }
                }
            }
            0usize
        })
        .sum()
}


fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const TEST_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 70);
    }
}