use std::{collections::HashMap, str::FromStr};

const INPUT: &str = "input/day02.txt";

pub fn solution() {
  let p = input::input_path(INPUT);
  let passwords: Vec<Entry> = input::parse_lines(p);
  let mut part1 = 0;
  let mut part2 = 0;
  for p in passwords {
    if p.is_valid() {
      part1 += 1;
    }
    if p.part2_is_valid() {
      part2 += 1;
    }
  }
  println!("part1: {}", part1);
  println!("part2: {}", part2);
}

struct Policy {
  lo: i32,
  hi: i32,
  c: char,
}
struct Entry {
  policy: Policy,
  password: String,
}

impl Entry {
  fn is_valid(&self) -> bool {
    let mut counts = HashMap::new();
    for c in self.password.chars() {
      *counts.entry(c).or_insert(0) += 1;
    }
    let count = counts.get(&self.policy.c).copied().unwrap_or(0);
    count >= self.policy.lo && count <= self.policy.hi
  }

  fn part2_is_valid(&self) -> bool {
    let password: Vec<_> = self.password.chars().collect();
    let i = self.policy.hi as usize - 1;
    let j = self.policy.lo as usize - 1;
    let c = self.policy.c;
    (password[i] == c) ^ (password[j] == c)
  }
}

impl FromStr for Entry {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> anyhow::Result<Self> {
    // Split into (policy, password)
    let split: Vec<&str> = s.split(':').collect();
    let password = split[1].trim().into();

    // Split the policy: (lo-hi, c)
    let split: Vec<&str> = split[0].split(' ').collect();
    let c = split[1].trim();
    let c = c.chars().next().unwrap();

    // Split lo-hi: (lo, hi)
    let split: Vec<&str> = split[0].split('-').collect();
    let lo: i32 = split[0].parse().unwrap();
    let hi: i32 = split[1].parse().unwrap();
    Ok(Entry {
      policy: Policy { hi, lo, c },
      password,
    })
  }
}
