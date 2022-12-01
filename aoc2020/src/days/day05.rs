use anyhow::{Error, Result};
use std::str::FromStr;

const INPUT: &str = "input/day05.txt";

pub fn solution() {
  let p = input::input_path(INPUT);
  let mut seats: Vec<SeatID> = input::parse_lines(p);
  seats.sort_by_key(|s| s.0);
  let SeatID(max) = seats.last().unwrap();
  println!("part1: {}", max);

  let mut part2 = 0;
  for i in 0..seats.len() - 1 {
    let SeatID(this) = seats[i];
    let SeatID(next) = seats[i + 1];
    if next - this == 2 {
      part2 = this + 1;
    }
  }
  println!("part2: {}", part2);
}

#[derive(Debug, Eq, PartialEq)]
struct SeatID(u32);

impl FromStr for SeatID {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self> {
    let mut id = 0;
    for (i, c) in s.chars().enumerate() {
      if c == 'B' || c == 'R' {
        id |= 1 << (9 - i);
      }
    }
    Ok(SeatID(id))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_seat_id_rom_str() {
    assert_eq!(SeatID::from_str("BFFFBBFRRR").unwrap(), SeatID(567));
    assert_eq!(SeatID::from_str("FFFBBBFRRR").unwrap(), SeatID(119));
    assert_eq!(SeatID::from_str("BBFFBBFRLL").unwrap(), SeatID(820));
  }
}
