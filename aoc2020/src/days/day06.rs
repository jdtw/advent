use input::input_path;
use std::{
  collections::HashMap,
  fs::File,
  io::{BufRead, BufReader},
};

const INPUT: &str = "input/day06.txt";
//const INPUT: &str = "input/day06_test.txt";

pub fn solution() {
  let p = input_path(INPUT);
  let f = File::open(p).unwrap();
  let f = BufReader::new(f);
  let mut part1 = 0;
  let mut part2 = 0;
  let mut group = HashMap::new();
  let mut members = 0;
  for line in f.lines() {
    let line = line.unwrap();
    if line.is_empty() {
      part1 += group.len();
      part2 += group.values().filter(|c| **c == members).count();
      // reset;
      group = HashMap::new();
      members = 0;
      continue;
    }
    members += 1;
    for c in line.chars() {
      *group.entry(c).or_insert(0) += 1;
    }
  }
  part1 += group.len();
  part2 += group.values().filter(|c| **c == members).count();
  println!("part1: {}", part1);
  println!("part2: {}", part2);
}
