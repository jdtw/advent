use std::{collections::HashMap, fmt::Display, str::FromStr};

const INPUT: &str = "input/day14_test.txt";

// NN: 1
// NC: 1
// CB: 1

// NC: 1
// CN: 1
// NB: 1
// BC: 1
// CH: 1
// HB: 1

// NNCB
// CH -> B
// HH -> N
// CB -> H
// NH -> C
// HB -> C
// HC -> B
// HN -> C
// NN -> C
// BH -> H
// NC -> B
// NB -> B
// BN -> B
// BB -> N
// BC -> B
// CC -> N
// CN -> C
// NCNBCHB
// NBCCNBBBCBHCB
// NBBBCNCCNBBNBNBBCHBHHBCHB
// NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
// NBBNBBNBBBNBBNBBCNCCNBBBCCNBCNCCNBBNBBNBBNBBNBBNBNBBNBBNBBNBBNBBCHBHHBCHBHHNHCNCHBCHBNBBCHBHHBCHB

pub fn solution() {
    let mut manual: Manual = input::parse(INPUT);
    for i in 0..10 {
        manual.step();
        println!("{}:", i);
        manual.count();
        //println!("{}: {}", i + 1, manual);
    }
    let (min, max) = manual.count();
    println!("Part1: {}", max - min);
}

struct Manual {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl Manual {
    fn step(&mut self) {
        let mut next: Vec<char> = Vec::new();
        for pair in self.template.windows(2) {
            let pair = (pair[0], pair[1]);
            let insert = self.rules.get(&pair).unwrap();
            next.extend_from_slice(&[pair.0, *insert]);
        }
        next.push(*self.template.last().unwrap());
        self.template = next;
    }

    fn count(&self) -> (usize, usize) {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in self.template.iter() {
            *counts.entry(*c).or_default() += 1;
        }
        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_by_key(|c| c.1);
        for (c, n) in counts.iter() {
            print!("{}:{} ", c, n);
        }
        println!();
        (counts[0].1, counts.last().unwrap().1)
    }
}

impl FromStr for Manual {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let template = lines.next().unwrap();
        let template = template.chars().collect();

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
        let s: String = self.template.iter().collect();
        write!(f, "{}", s)
    }
}
