use std::{collections::VecDeque, str::FromStr};

const INPUT: &str = "input/day11.txt";

pub fn solution() {
    let mut part1: Monkeys = input::parse(INPUT);
    let mut part2 = part1.clone();
    for _ in 0..20 {
        part1.round(Part::One);
    }
    println!("Part1: {}", part1.monkey_business());

    for _ in 0..10000 {
        part2.round(Part::Two(part2.gcd()));
    }
    println!("Part2: {}", part2.monkey_business());
}

#[derive(Clone, Debug)]
struct Monkeys(Vec<Monkey>);

#[derive(Copy, Clone)]
enum Part {
    One,
    Two(i64),
}

impl Monkeys {
    fn gcd(&self) -> i64 {
        self.0.iter().map(|m| m.divisible).product()
    }
    fn round(&mut self, part: Part) {
        for i in 0..self.0.len() {
            while let Some(worry) = self.0[i].items.pop_front() {
                self.0[i].inspections += 1;
                let mut worry = match self.0[i].operation {
                    Op::Double => worry * 2,
                    Op::Square => worry * worry,
                    Op::Add(n) => worry + n,
                    Op::Mul(n) => worry * n,
                };
                match part {
                    Part::One => worry /= 3,
                    Part::Two(gcd) => worry %= gcd,
                }
                let throw = if worry % self.0[i].divisible == 0 {
                    self.0[i].on_true
                } else {
                    self.0[i].on_false
                };
                self.0[throw].items.push_back(worry);
            }
        }
    }
    fn monkey_business(&self) -> usize {
        let mut inspections: Vec<usize> =
            self.0.iter().map(|m| m.inspections).collect();
        inspections.sort();
        inspections.reverse();
        inspections[0] * inspections[1]
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add(i64),
    Mul(i64),
    Square,
    Double,
}
#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Op,
    divisible: i64,
    on_true: usize,
    on_false: usize,
    inspections: usize,
}

impl FromStr for Monkeys {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkeys = Vec::new();
        let mut lines = s.lines();
        while let Some(monkey) = lines.next() {
            assert!(monkey.starts_with("Monkey"));
            let items = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Starting items: ")
                .unwrap();
            let items = input::split(items, ", ");
            let operation = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Operation: new = old ")
                .unwrap();
            let (op, num): (char, String) = input::split2(operation, " ");
            let operation = match (op, num.as_str()) {
                ('*', "old") => Op::Square,
                ('+', "old") => Op::Double,
                ('*', _) => Op::Mul(num.parse().unwrap()),
                ('+', _) => Op::Add(num.parse().unwrap()),
                _ => {
                    panic!("Unknown op, num {}, {}", op, num);
                }
            };
            let divisible = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();
            let on_true = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();
            let on_false = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();
            lines.next();
            let monkey = Monkey {
                items: VecDeque::from(items),
                operation,
                divisible,
                on_true,
                on_false,
                inspections: 0,
            };
            monkeys.push(monkey);
        }
        Ok(Monkeys(monkeys))
    }
}
