use std::{collections::HashSet, fmt, str::FromStr};

use anyhow::anyhow;
use input::{input_path, parse_lines};
use once_cell::unsync::Lazy;
use regex::Regex;

const INPUT: &str = "input/day08.txt";

pub fn solution() {
  let program: Vec<Instruction> = parse_lines(input_path(INPUT));

  // Part 1
  println!("part1: {}", run(&program));

  // Part 2
  for i in 0..program.len() {
    let mut program = program.clone();
    program[i] = match program[i] {
      Instruction::Acc(_) => continue,
      Instruction::Jmp(n) => Instruction::Nop(n),
      Instruction::Nop(n) => Instruction::Jmp(n),
    };
    if let Terminate::Terminate(acc) = run(&program) {
      println!("part2: {}", acc);
      return;
    }
  }
  unreachable!()
}

enum Terminate {
  Loop(i64),
  Terminate(i64),
}

fn run(program: &[Instruction]) -> Terminate {
  use Instruction::*;
  let mut acc = 0;
  let mut ip = 0;
  let mut visited = HashSet::new();
  let terminal = program.len() as i64;
  while !visited.contains(&ip) {
    if ip == terminal {
      return Terminate::Terminate(acc);
    }
    visited.insert(ip);
    match program[ip as usize] {
      Nop(_) => ip += 1,
      Jmp(n) => ip += n,
      Acc(n) => {
        acc += n;
        ip += 1;
      }
    }
  }
  Terminate::Loop(acc)
}

impl fmt::Display for Terminate {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Terminate::Loop(n) => write!(f, "loop {}", n),
      Terminate::Terminate(n) => write!(f, "terminate {}", n),
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
  Nop(i64),
  Jmp(i64),
  Acc(i64),
}

impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use Instruction::*;
    match self {
      Nop(n) => write!(f, "nop {}", n),
      Jmp(n) => write!(f, "jmp {}", n),
      Acc(n) => write!(f, "acc {}", n),
    }
  }
}

impl FromStr for Instruction {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let re = Lazy::new(|| {
      Regex::new(r"(?P<op>nop|acc|jmp) (?P<num>[+-]\d+)").unwrap()
    });
    let caps = re.captures(s).ok_or_else(|| anyhow!("malformed input"))?;
    let num = i64::from_str(&caps["num"])?;
    match &caps["op"] {
      "nop" => Ok(Instruction::Nop(num)),
      "jmp" => Ok(Instruction::Jmp(num)),
      "acc" => Ok(Instruction::Acc(num)),
      _ => Err(anyhow!("unknown instruction")),
    }
  }
}
