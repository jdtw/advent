use std::collections::HashMap;

use input::{input_path, parse_lines};

const INPUT: &str = "input/day10.txt";

pub fn solution() {
  let mut adapters: Vec<u64> = parse_lines(input_path(INPUT));
  normalize_adapters(&mut adapters);
  let diffs = count_diffs(&adapters);
  println!("{:?}", diffs);
  println!("part1: {}", diffs[&1] * diffs[&3]);
}

fn normalize_adapters(adapters: &mut Vec<u64>) {
  adapters.sort();
  adapters.insert(0, 0);
  adapters.push(adapters.last().unwrap() + 3);
}

fn count_diffs(adapters: &[u64]) -> HashMap<u64, u64> {
  let mut diffs = HashMap::new();
  for i in 0..adapters.len() - 1 {
    let d = adapters[i + 1] - adapters[i];
    *diffs.entry(d).or_insert(0) += 1;
  }
  diffs
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_part1() {
    let mut input = vec![
      28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11,
      1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
    ];
    normalize_adapters(&mut input);
    let diffs = count_diffs(&input);
    assert_eq!(diffs[&1], 22);
    assert_eq!(diffs[&3], 10);
  }
}
