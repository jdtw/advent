use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const INPUT: &str = "input/day8.txt";

pub fn solution() {
    let entries: Vec<Entry> = input::parse_lines(INPUT);
    let mut part1 = 0;
    for e in entries.iter() {
        part1 += e
            .outputs
            .iter()
            .filter(|o| matches!(o.len(), 2 | 4 | 3 | 7))
            .count();
    }
    println!("Part1: {}", part1);

    let part2: u64 = entries.iter().map(Entry::solve).sum();
    println!("Part2: {}", part2);
}

impl Entry {
    fn solve(&self) -> u64 {
        let nums = HashMap::from([
            ("abcefg", '0'),
            ("cf", '1'),
            ("acdeg", '2'),
            ("acdfg", '3'),
            ("bcdf", '4'),
            ("abdfg", '5'),
            ("abdefg", '6'),
            ("acf", '7'),
            ("abcdefg", '8'),
            ("abcdfg", '9'),
        ]);

        // Count all segments
        let mut counts: HashMap<char, u8> = HashMap::new();
        for u in self.signals.iter() {
            for c in u.chars() {
                *counts.entry(c).or_default() += 1;
            }
        }

        // b, e, and f have unique counts.
        let mut b = '\0';
        let mut e = '\0';
        let mut f = '\0';
        for (c, count) in counts {
            match count {
                6 => b = c,
                4 => e = c,
                9 => f = c,
                _ => (),
            }
        }

        let mut one = "";
        let mut four = "";
        let mut seven = "";
        let mut eight = "";
        for s in self.signals.iter() {
            match s.len() {
                2 => one = s.as_str(),
                4 => four = s.as_str(),
                3 => seven = s.as_str(),
                7 => eight = s.as_str(),
                _ => (),
            }
        }

        let mut known = HashSet::from([b, e, f]);
        // Segment one is 'cf', and we know 'f'.
        let c = one.chars().find(|c| !known.contains(c)).unwrap();
        known.insert(c);

        // Segment seven is 'acf', and we know 'cf'
        let a = seven.chars().find(|a| !known.contains(a)).unwrap();
        known.insert(a);

        // Segment four is 'bcdf' and we know 'bcf'
        let d = four.chars().find(|d| !known.contains(d)).unwrap();
        known.insert(d);

        // Segment eight is 'abcdefg' and we know 'abcdef'
        let g = eight.chars().find(|g| !known.contains(g)).unwrap();

        let solve = HashMap::from([
            (a, 'a'),
            (b, 'b'),
            (c, 'c'),
            (d, 'd'),
            (e, 'e'),
            (f, 'f'),
            (g, 'g'),
        ]);

        self.outputs
            .iter()
            .map(|s| {
                let mut chars = s
                    .chars()
                    .map(|c| *solve.get(&c).unwrap())
                    .collect::<Vec<char>>();
                chars.sort();
                let s = chars.iter().collect::<String>();
                nums.get(s.as_str()).unwrap()
            })
            .collect::<String>()
            .parse()
            .unwrap()
    }
}

#[derive(Debug)]
struct Entry {
    signals: Vec<String>,
    outputs: Vec<String>,
}

impl FromStr for Entry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signals, outputs) = s.split_once(" | ").unwrap();
        let signals =
            signals.split_ascii_whitespace().map(String::from).collect();
        let outputs =
            outputs.split_ascii_whitespace().map(String::from).collect();
        Ok(Entry { signals, outputs })
    }
}
