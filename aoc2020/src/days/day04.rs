use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::{
  collections::HashMap,
  fs::File,
  io::{BufRead, BufReader},
};

const INPUT: &str = "input/day04.txt";

pub fn solution() {
  let passes = parse_input();
  let mut valid = 0;
  for Pass(pass) in passes.iter() {
    if pass.contains_key("byr")
      && pass.contains_key("iyr")
      && pass.contains_key("eyr")
      && pass.contains_key("hgt")
      && pass.contains_key("hcl")
      && pass.contains_key("ecl")
      && pass.contains_key("pid")
    {
      valid += 1;
    }
  }
  println!("part1: {}", valid);
  let mut valid = 0;
  for pass in passes.iter() {
    if pass.validate().is_ok() {
      valid += 1;
    }
  }
  println!("part2: {}", valid);
}

fn parse_input() -> Vec<Pass> {
  let p = input::input_path(INPUT);
  let f = File::open(p).unwrap();
  let f = BufReader::new(f);
  let mut passes = Vec::new();
  let mut pass = HashMap::new();
  for line in f.lines() {
    let line = line.unwrap();
    if line.is_empty() {
      passes.push(Pass(pass));
      pass = HashMap::new();
      continue;
    }
    for pair in line.split_whitespace() {
      let mut pair = pair.split(':');
      let key = pair.next().unwrap();
      let val = pair.next().unwrap();
      pass.insert(key.into(), val.into());
    }
  }
  if !pass.is_empty() {
    passes.push(Pass(pass));
  }
  passes
}

struct Pass(HashMap<String, String>);

impl Pass {
  fn get(&self, key: &str) -> Result<&str> {
    self
      .0
      .get(key)
      .map(String::as_str)
      .ok_or_else(|| anyhow!("missing '{}'", key))
  }

  fn validate(&self) -> Result<()> {
    // 'byr' is a number between 1920 and 2002
    let byr = self.get("byr")?;
    let byr: u16 = byr.parse().context("'byr' must be a number")?;
    if byr < 1920 || byr > 2002 {
      return Err(anyhow!("'byr' out of range"));
    }

    // 'iyr' is a number between 2010 and 2020
    let iyr = self.get("iyr")?;
    let iyr: u16 = iyr.parse().context("'iyr' must be a number")?;
    if iyr < 2010 || iyr > 2020 {
      return Err(anyhow!("'iyr' out of range"));
    }

    // 'eyr' is a number between 2020 and 2030
    let eyr = self.get("eyr")?;
    let eyr: u16 = eyr.parse().context("'eyr' must be a number")?;
    if eyr < 2020 || eyr > 2030 {
      return Err(anyhow!("'eyr' out of range"));
    }

    let hgt = self.get("hgt")?;
    validate_hgt(hgt)?;

    let hcl = self.get("hcl")?;
    validate_hcl(hcl)?;

    let ecl = self.get("ecl")?;
    validate_ecl(ecl)?;

    let pid = self.get("pid")?;
    validate_pid(pid)?;

    Ok(())
  }
}

fn validate_hgt(hgt: &str) -> Result<()> {
  let re = Regex::new(r"^(\d+)(in|cm)$")?;
  let caps = re.captures(hgt).ok_or_else(|| anyhow!("malformed 'hgt'"))?;
  let hgt: u16 = caps[1].parse()?;
  match &caps[2] {
    "in" => {
      if hgt < 59 || hgt > 76 {
        return Err(anyhow!("'hgt' in inches out of range"));
      }
    }
    "cm" => {
      if hgt < 150 || hgt > 193 {
        return Err(anyhow!("'hgt' in cm out of range"));
      }
    }
    // The regex ensures this is unreachable
    _ => unreachable!(),
  }
  Ok(())
}

fn validate_hcl(hcl: &str) -> Result<()> {
  let re = Regex::new(r"^\#[0-9a-f]{6}$")?;
  match re.is_match(hcl) {
    true => Ok(()),
    false => Err(anyhow!("Invalid 'hcl'")),
  }
}

fn validate_ecl(ecl: &str) -> Result<()> {
  let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$")?;
  match re.is_match(ecl) {
    true => Ok(()),
    false => Err(anyhow!("Invalid 'ecl'")),
  }
}

fn validate_pid(pid: &str) -> Result<()> {
  let re = Regex::new(r"^\d{9}$")?;
  match re.is_match(pid) {
    true => Ok(()),
    false => Err(anyhow!("Invalid 'pid'")),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_hgt() {
    // cm ranges
    assert!(validate_hgt("149cm").is_err());
    assert!(validate_hgt("150cm").is_ok());
    assert!(validate_hgt("193cm").is_ok());
    assert!(validate_hgt("194cm").is_err());

    // in ranges
    assert!(validate_hgt("58in").is_err());
    assert!(validate_hgt("59in").is_ok());
    assert!(validate_hgt("76in").is_ok());
    assert!(validate_hgt("77in").is_err());

    // invalid unit
    assert!(validate_hgt("100abc").is_err());

    // invalid number
    assert!(validate_hgt("cm").is_err());
  }

  #[test]
  fn test_hcl() {
    assert!(validate_hcl("#00aaff").is_ok());
    assert!(validate_hcl("#012345").is_ok());
    assert!(validate_hcl("#00aafff").is_err());
    assert!(validate_hcl("00aaff").is_err());
    assert!(validate_hcl("#zzzzzz").is_err());
  }

  #[test]
  fn test_ecl() {
    assert!(validate_ecl("amb").is_ok());
    assert!(validate_ecl("oth").is_ok());
    assert!(validate_ecl("xxx").is_err());
    assert!(validate_ecl("o").is_err());
  }
}
