use std::str::FromStr;

use input::{input_path, parse_lines};

const INPUT: &str = "input/day3.txt";

pub fn solution() {
    let data: Vec<Num> = parse_lines(input_path(INPUT));
    let len = data[0].len;

    {
        let data: Vec<u32> = data.iter().map(|n| n.n).collect();

        let mut gamma = 0;
        let mut epsilon = 0;
        for i in 0..len {
            let shift = len - i - 1;
            let num_ones = data
                .iter()
                .flat_map(|n| {
                    let n = n >> shift;
                    if n & 1 == 1 {
                        Some(())
                    } else {
                        None
                    }
                })
                .count();
            let num_zeros = data.len() - num_ones;
            if num_ones > num_zeros {
                gamma |= 1 << shift;
            } else {
                epsilon |= 1 << shift;
            }
        }
        println!("part1: {}", gamma * epsilon);
    }

    let o = oxygen(data.clone());
    let c = co2(data);
    println!("part2: {}", o * c);
}

fn oxygen(nums: Vec<Num>) -> u32 {
    rating(nums, |zeros, ones| ones >= zeros)
}

fn co2(nums: Vec<Num>) -> u32 {
    rating(nums, |zeros, ones| zeros > ones)
}

fn rating(mut nums: Vec<Num>, keep: impl Fn(usize, usize) -> bool) -> u32 {
    let len = nums[0].len;
    for i in 0..len {
        if nums.len() == 1 {
            return nums[0].n;
        }
        let (zeros, ones) = count_nth(i, &nums);
        let bit = keep(zeros, ones);
        nums = filter(i, bit, &nums);
    }
    assert!(nums.len() == 1);
    nums[0].n
}

fn filter(i: usize, bit: bool, nums: &[Num]) -> Vec<Num> {
    nums.iter().filter(|n| n.nth(i) == bit).cloned().collect()
}

fn count_nth(i: usize, nums: &[Num]) -> (usize, usize) {
    let mut ones = 0;
    for n in nums {
        if n.nth(i) {
            ones += 1;
        }
    }
    (nums.len() - ones, ones)
}

#[derive(Debug, Clone)]
struct Num {
    n: u32,
    len: usize,
}
impl FromStr for Num {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = u32::from_str_radix(s, 2)?;
        Ok(Num { n, len: s.len() })
    }
}

impl Num {
    fn nth(&self, i: usize) -> bool {
        let shift = self.len - i - 1;
        let mask = 1 << shift;
        self.n & mask != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth() {
        let n = Num::from_str("100").unwrap();
        assert_eq!(n.nth(0), true);
        assert_eq!(n.nth(1), false);
        assert_eq!(n.nth(2), false);

        let n = Num::from_str("001").unwrap();
        assert_eq!(n.nth(0), false);
        assert_eq!(n.nth(1), false);
        assert_eq!(n.nth(2), true);

        let n = Num::from_str("111").unwrap();
        assert_eq!(n.nth(0), true);
        assert_eq!(n.nth(1), true);
        assert_eq!(n.nth(2), true);
    }

    #[test]
    fn test_oxygen() {
        let nums = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111",
            "11100", "10000", "11001", "00010", "01010",
        ]
        .into_iter()
        .map(Num::from_str)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
        assert_eq!(oxygen(nums), 23);
    }

    #[test]
    fn test_co2() {
        let nums = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111",
            "11100", "10000", "11001", "00010", "01010",
        ]
        .into_iter()
        .map(Num::from_str)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
        assert_eq!(co2(nums), 10);
    }
}
