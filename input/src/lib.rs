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

pub fn path<P: AsRef<Path>>(relative: P) -> PathBuf {
    Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(relative)
}

pub fn string<P: AsRef<Path>>(p: P) -> String {
    fs::read_to_string(p).unwrap()
}

pub fn parse<I, P>(p: P) -> I
where
    I: FromStr,
    I::Err: Debug,
    P: AsRef<Path>,
{
    let s = string(p);
    I::from_str(&s).unwrap()
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

pub struct SpaceVec<T>(pub Vec<T>);
impl<T: FromStr> FromStr for SpaceVec<T> {
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<T>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(SpaceVec(vec))
    }
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
    csv_str(&s)
}

pub fn csv_str<I>(s: &str) -> Vec<I>
where
    I: FromStr,
    I::Err: Debug,
{
    let CsvVec::<I>(v) = s.parse().unwrap();
    v
}

pub fn space_str<I>(s: &str) -> Vec<I>
where
    I: FromStr,
    I::Err: Debug,
{
    let SpaceVec::<I>(v) = s.parse().unwrap();
    v
}

pub fn split2<T, U>(s: &str, delimiter: &str) -> (T, U)
where
    T: FromStr,
    T::Err: Debug,
    U: FromStr,
    U::Err: Debug,
{
    let (t, u) = s.split_once(delimiter).unwrap();
    (T::from_str(t).unwrap(), U::from_str(u).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_vec() {
        let SpaceVec::<i32>(v) = "1 2 3  4 5000".parse().unwrap();
        assert_eq!(v, vec![1, 2, 3, 4, 5000]);
    }

    #[test]
    fn test_empty_space_vec() {
        let SpaceVec::<i32>(v) = "".parse().unwrap();
        assert!(v.is_empty());
    }

    #[test]
    fn test_csv_vec() {
        let CsvVec::<i32>(v) = "1,2,3,4,5000".parse().unwrap();
        assert_eq!(v, vec![1, 2, 3, 4, 5000]);
    }

    #[test]
    fn test_empty_csv_vec() {
        let CsvVec::<i32>(v) = "".parse().unwrap();
        assert!(v.is_empty());
    }

    #[test]
    fn test_split2() {
        assert_eq!((1, -2), split2::<u8, i8>("1,-2", ","));
    }
}
