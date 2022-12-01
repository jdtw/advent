use std::str::FromStr;

use input::parse_lines;

const INPUT: &str = "input/day3.txt";

pub fn solution() {
    let data: Vec<Num> = parse_lines(INPUT);
    let len = data[0].len;
    let data: Vec<u32> = data.into_iter().map(|n| n.n).collect();

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

#[derive(Debug)]
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
