use input::{input_path, parse_lines};
use std::cmp::Ordering;

const INPUT: &str = "input/day01.txt";

pub fn solution() {
  let mut report: Vec<i32> = parse_lines(input_path(INPUT));
  report.sort();
  let part1 = (|| {
    for i in 0..report.len() - 1 {
      for j in (i + 1..report.len()).rev() {
        let sum = report[i] + report[j];
        match sum.cmp(&2020) {
          Ordering::Equal => return report[i] * report[j],
          Ordering::Greater => continue,
          Ordering::Less => break,
        }
      }
    }
    panic!("not found");
  })();
  println!("part1: {}", part1);
  let part2 = (|| {
    for i in 0..report.len() - 1 {
      for j in (i + 1..report.len()).rev() {
        for k in i + 1..j {
          let sum = report[i] + report[k] + report[j];
          match sum.cmp(&2020) {
            Ordering::Equal => return report[i] * report[k] * report[j],
            Ordering::Greater => break,
            Ordering::Less => continue,
          }
        }
      }
    }
    panic!("not found");
  })();
  println!("part2: {}", part2);
}
