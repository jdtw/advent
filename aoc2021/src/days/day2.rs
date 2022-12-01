use std::str::FromStr;

use anyhow::{anyhow, bail};
use input::parse_lines;

const INPUT: &str = "input/day2.txt";

pub fn solution() {
    let data: Vec<Instruction> = parse_lines(INPUT);
    {
        let mut horiz = 0;
        let mut depth = 0;
        for inst in &data {
            match inst.direction {
                Dir::Up => depth -= inst.magnitute,
                Dir::Down => depth += inst.magnitute,
                Dir::Forward => horiz += inst.magnitute,
            }
        }
        let part1 = horiz * depth;
        println!("part1: {}", part1);
    }
    {
        let mut horiz = 0;
        let mut depth = 0;
        let mut aim = 0;
        for inst in data {
            match inst.direction {
                Dir::Up => aim -= inst.magnitute,
                Dir::Down => aim += inst.magnitute,
                Dir::Forward => {
                    horiz += inst.magnitute;
                    depth += aim * inst.magnitute;
                }
            }
        }
        let part1 = horiz * depth;
        println!("part2: {}", part1);
    }
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Forward,
}

#[derive(Debug)]
struct Instruction {
    direction: Dir,
    magnitute: i64,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, mag) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("couldn't split on ' '"))?;
        let mag = i64::from_str(mag)?;
        let dir = match dir {
            "forward" => Dir::Forward,
            "up" => Dir::Up,
            "down" => Dir::Down,
            _ => bail!("invalid direction"),
        };
        Ok(Instruction {
            direction: dir,
            magnitute: mag,
        })
    }
}
