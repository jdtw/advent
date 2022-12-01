use anyhow::{anyhow, Error, Result};
use input::{input_path, parse_lines};
use once_cell::unsync::Lazy;
use regex::Regex;
use std::{
  collections::{HashMap, HashSet},
  str::FromStr,
};

const INPUT: &str = "input/day07.txt";

pub fn solution() {
  let rules: Vec<Rule> = parse_lines(input_path(INPUT));

  // Build a map of bag -> bags that can hold that bag...
  let mut contains: HashMap<&str, Vec<&str>> = HashMap::new();
  // and a map of bag -> bags that bag holds.
  let mut holds: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
  for r in rules.iter() {
    let holds_entry = holds.entry(&r.bag).or_insert(Vec::new());
    for (n, bag) in r.contains.iter() {
      contains.entry(bag).or_insert(Vec::new()).push(&r.bag);
      holds_entry.push((*n, bag))
    }
  }

  // Part 1
  {
    let mut stack = contains["shiny gold"].clone();
    let mut result = HashSet::new();
    while let Some(bag) = stack.pop() {
      if result.contains(bag) {
        continue;
      }
      result.insert(bag);
      if let Some(bags) = contains.get(bag) {
        for b in bags {
          stack.push(b);
        }
      }
    }
    println!("part1: {}", result.len());
  }

  // Part 2
  {
    let mut stack = holds["shiny gold"].clone();
    let mut result = HashMap::new();
    while let Some((n, bag)) = stack.pop() {
      // Add 'n' number of bags...
      *result.entry(bag).or_insert(0) += n;
      if let Some(bags) = holds.get(bag) {
        // And then add n times what those bags hold...
        for _ in 0..n {
          for b in bags {
            stack.push(*b);
          }
        }
      }
      println!("part2: {}", result.values().sum::<usize>())
    }
  }
}

#[derive(Debug)]
struct Rule {
  bag: String,
  contains: Vec<(usize, String)>,
}

impl FromStr for Rule {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // posh crimson bags contain 2 mirrored tan bags, 1 faded red bag, 1 striped gray bag.
    let rule_re = Lazy::new(|| {
      Regex::new(r"^(?P<bag>.+) bags contain (?P<contents>.*).$").unwrap()
    });
    let contents_re =
      Lazy::new(|| Regex::new(r"(?P<num>\d+) (?P<bag>[^\d]+) bag").unwrap());
    let caps = rule_re
      .captures(s)
      .ok_or_else(|| anyhow!("malformed input"))?;
    let mut rule = Rule {
      bag: String::from(&caps["bag"]),
      contains: Vec::new(),
    };
    for contents in contents_re.captures_iter(&caps["contents"]) {
      let n = contents["num"].parse()?;
      rule.contains.push((n, contents["bag"].into()))
    }
    Ok(rule)
  }
}
