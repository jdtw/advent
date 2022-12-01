use std::str::FromStr;

use input::{input_path, parse_lines};

const INPUT: &str = "input/day1.txt";

#[derive(Debug)]
struct Calories(Option<u32>);

pub fn solution() {
    let cals: Vec<Calories> = parse_lines(input_path(INPUT));
    let mut elves = count_cals(cals);
    elves.sort();
    let elves = elves.into_iter().rev().collect::<Vec<_>>();
    println!("Part1: {}", elves[0]);
    println!("Part2: {}", elves[0] + elves[1] + elves[2]);
}

fn count_cals(cals: Vec<Calories>) -> Vec<u32> {
    let mut elves = Vec::new();
    let mut acc = 0;
    for Calories(c) in cals {
        match c {
            Some(c) => acc += c,
            None => {
                elves.push(acc);
                acc = 0;
            }
        }
    }
    elves
}

impl FromStr for Calories {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Calories(None))
        } else {
            let cals = u32::from_str(s)?;
            Ok(Calories(Some(cals)))
        }
    }
}
