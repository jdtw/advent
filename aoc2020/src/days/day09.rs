use input::{input_path, parse_lines};
use std::cmp::Ordering;

const INPUT: &str = "input/day09.txt";

pub fn solution() {
  let nums: Vec<u64> = parse_lines(input_path(INPUT));
  let n = part1(&nums).expect("no solution found!");
  println!("part1: {}", n);
  println!("part2: {}", part2(n, &nums).expect("no solution found!"));
}

fn part1(nums: &[u64]) -> Option<u64> {
  for i in 0..nums.len() - 25 {
    let prev = &nums[i..i + 25];
    let n = nums[i + 25];
    if !is_sum_of(n, prev) {
      return Some(n);
    }
  }
  None
}

fn part2(n: u64, nums: &[u64]) -> Option<u64> {
  for i in 0..nums.len() - 1 {
    let mut sum = nums[i];
    for j in i + 1..nums.len() {
      sum += nums[j];
      match n.cmp(&sum) {
        Ordering::Equal => {
          // This is it. Now find the smallest and largest.
          let mut range = Vec::from(&nums[i..j + 1]);
          range.sort();
          return Some(range.first().unwrap() + range.last().unwrap());
        }
        Ordering::Less => break,
        Ordering::Greater => continue,
      }
    }
  }
  None
}

fn is_sum_of(n: u64, prev: &[u64]) -> bool {
  assert!(prev.len() == 25);
  let mut sorted = Vec::from(prev);
  sorted.sort();
  for i in 0..sorted.len() - 1 {
    if sorted[i] >= n {
      return false;
    }
    for j in i + 1..sorted.len() {
      match n.cmp(&(sorted[i] + sorted[j])) {
        Ordering::Equal => return true,
        Ordering::Less => break,
        Ordering::Greater => continue,
      }
    }
  }
  false
}
