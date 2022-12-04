use std::{collections::HashMap, fmt::Display, str::FromStr};

const INPUT: &str = "input/day14.txt";

pub fn solution() {
    let mut manual: Manual = input::parse(INPUT);
    for _i in 0..10 {
        manual.step();
    }
    let (min, max) = manual.count();
    let part1 = max - min;
    for _ in 10..40 {
        manual.step();
    }
    let (min, max) = manual.count();
    let part2 = max - min;
    println!("Part1: {}\nPart2: {}", part1, part2);
}

struct Manual {
    template: HashMap<(char, char), usize>,
    rules: HashMap<(char, char), char>,
}

impl Manual {
    fn step(&mut self) {
        let mut next: HashMap<(char, char), usize> = HashMap::new();
        for (pair, count) in self.template.iter() {
            let insert = self.rules.get(pair).unwrap();
            *next.entry((pair.0, *insert)).or_default() += count;
            *next.entry((*insert, pair.1)).or_default() += count;
        }
        self.template = next;
    }

    fn count(&self) -> (usize, usize) {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for ((a, b), count) in self.template.iter() {
            *counts.entry(*a).or_default() += count;
            *counts.entry(*b).or_default() += count;
        }
        let mut counts = counts
            .into_iter()
            .map(|(p, c)| (p, (c + (c % 2)) / 2))
            .collect::<Vec<_>>();
        counts.sort_by_key(|c| c.1);
        (counts[0].1, counts.last().unwrap().1)
    }
}

impl FromStr for Manual {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let line = lines.next().unwrap();
        let chars: Vec<char> = line.chars().collect();
        let mut template = HashMap::new();
        for pair in chars.windows(2) {
            *template.entry((pair[0], pair[1])).or_default() += 1;
        }

        // skip newline
        lines.next().unwrap();

        let mut rules = HashMap::new();
        for rule in lines {
            let (from, to) = rule.split_once(" -> ").unwrap();
            let from = from.chars().collect::<Vec<_>>();
            let to = to.chars().collect::<Vec<_>>();
            rules.insert((from[0], from[1]), to[0]);
        }
        Ok(Manual { template, rules })
    }
}

impl Display for Manual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (p, c) in self.template.iter() {
            write!(f, "{}{}: {} ", p.0, p.1, c)?;
        }
        Ok(())
    }
}
