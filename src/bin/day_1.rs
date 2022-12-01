use std::cmp;

const INPUT: &str = include_str!("../../input/day_1.txt");

struct Inventory {
    items: Vec<usize>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn sum(&self) -> usize {
        self.items.iter().sum()
    }

    pub fn push(&mut self, line: &str) {
        self.items.push(line.parse().unwrap())
    }
}

fn parse_inventories(input: &str) -> Vec<Inventory> {
    let lines = input.lines();
    let mut inventories = vec![];
    let mut inventory = None;
    for line in lines {
        if line.is_empty() {
            if let Some(i) = inventory {
                inventories.push(i);
            }
            inventory = Some(Inventory::new());
            continue;
        }

        if let Some(i) = inventory.as_mut() {
            i.push(line);
        }
    }
    inventories
}

fn main() {
    let mut inventories = parse_inventories(INPUT);
    inventories.sort_by_key(|y| cmp::Reverse(y.sum()));
    println!("max calories: {}", inventories[0].sum());
    println!(
        "max three calories: {}",
        &inventories[..3].iter().map(|i| i.sum()).sum::<usize>()
    );
}


