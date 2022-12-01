use std::{
  fmt::Debug,
  fs::{self, File},
  io::{BufRead, BufReader},
  path::{Path, PathBuf},
  str::FromStr,
};

pub fn input_path<P: AsRef<Path>>(relative: P) -> PathBuf {
  Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(relative)
}

pub fn string<P: AsRef<Path>>(p: P) -> String {
  fs::read_to_string(p).unwrap()
}

#[cfg(test)]
pub fn parse_str_lines<I>(s: &str) -> Vec<I>
where
  I: FromStr,
  I::Err: Debug,
{
  s.lines().map(|l| l.parse::<I>().unwrap()).collect()
}

/// Parses each line of input in path `p` as an `I`.
pub fn parse_lines<I, P>(p: P) -> Vec<I>
where
  I: FromStr,
  I::Err: Debug,
  P: AsRef<Path>,
{
  let f = File::open(p).unwrap();
  let f = BufReader::new(f);
  f.lines()
    .map(|l| l.unwrap().parse::<I>().unwrap())
    .collect()
}

pub struct CsvVec<T>(pub Vec<T>);
impl<T: FromStr> FromStr for CsvVec<T> {
  type Err = T::Err;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let vec = s
      .trim()
      .split(',')
      .filter(|s| !s.is_empty())
      .map(|s| s.parse::<T>())
      .collect::<Result<Vec<_>, _>>()?;
    Ok(CsvVec(vec))
  }
}

pub struct CharVec<T>(pub Vec<T>);
impl<T: From<char>> FromStr for CharVec<T> {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let vec = s.chars().map(|c| c.into()).collect();
    Ok(CharVec(vec))
  }
}

pub fn csv<I, P>(p: P) -> Vec<I>
where
  I: FromStr,
  I::Err: Debug,
  P: AsRef<Path>,
{
  let s = fs::read_to_string(p).unwrap();
  let CsvVec::<I>(v) = s.parse().unwrap();
  v
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_csv_vec() {
    let CsvVec::<i32>(v) = "1,2,3,4,5".parse().unwrap();
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
  }

  #[test]
  fn test_empty_csv_vec() {
    let CsvVec::<i32>(v) = "".parse().unwrap();
    assert!(v.is_empty());
  }
}
