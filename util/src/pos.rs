use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::{Add, AddAssign},
    str::FromStr,
};

use anyhow::anyhow;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Pos(pub i64, pub i64);

pub type PosSet = HashSet<Pos>;
pub type PosMap<T> = HashMap<Pos, T>;

impl Pos {
    pub fn compass_neighbors(&self) -> [Pos; 4] {
        let &Pos(x, y) = self;
        [Pos(x + 1, y), Pos(x, y + 1), Pos(x - 1, y), Pos(x, y - 1)]
    }
    pub fn neighbors(&self) -> [Pos; 8] {
        let &Pos(x, y) = self;
        [
            Pos(x + 1, y),
            Pos(x + 1, y + 1),
            Pos(x, y + 1),
            Pos(x - 1, y + 1),
            Pos(x - 1, y),
            Pos(x - 1, y - 1),
            Pos(x, y - 1),
            Pos(x + 1, y - 1),
        ]
    }
    pub fn distance(&self, other: &Pos) -> u64 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
    pub fn iter_x(&self, end: i64) -> PosIter {
        assert!(end >= self.0);
        PosIter {
            step: Box::new(|p| p + Pos(1, 0)),
            step_back: Box::new(|p| p + Pos(-1, 0)),
            pos: *self,
            end: Pos(end, self.1),
        }
    }
    pub fn iter_y(&self, end: i64) -> PosIter {
        assert!(end >= self.1);
        PosIter {
            step: Box::new(|p| p + Pos(0, 1)),
            step_back: Box::new(|p| p + Pos(0, -1)),
            pos: *self,
            end: Pos(self.0, end),
        }
    }
}

pub struct PosIter {
    step: Box<dyn Fn(Pos) -> Pos>,
    step_back: Box<dyn Fn(Pos) -> Pos>,
    pos: Pos,
    end: Pos,
}

impl Iterator for PosIter {
    type Item = Pos;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > self.end {
            return None;
        }
        let pos = self.pos;
        self.pos = (self.step)(pos);
        Some(pos)
    }
}

impl DoubleEndedIterator for PosIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.pos > self.end {
            return None;
        }
        let end = self.end;
        self.end = (self.step_back)(end);
        Some(end)
    }
}

pub struct DigitGrid<T>(pub PosMap<T>);
impl<T> FromStr for DigitGrid<T>
where
    T: TryFrom<u32>,
    <T as TryFrom<u32>>::Error: std::error::Error + Send + Sync + 'static,
{
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let d = c
                    .to_digit(10)
                    .ok_or_else(|| anyhow!("couldn't parse digit"))?;
                let d = T::try_from(d)?;
                map.insert(Pos(x as i64, y as i64), d);
            }
        }
        Ok(DigitGrid(map))
    }
}

impl<T> DigitGrid<T> {
    pub fn max(&self) -> Pos {
        *self.0.keys().max().unwrap()
    }
}

impl FromStr for Pos {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix('(')
            .unwrap_or(s)
            .strip_suffix(')')
            .unwrap_or(s);
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| anyhow!("invalid separator"))?;
        let x: i64 = x
            .parse()
            .map_err(|e| anyhow!("couldn't parse x coord: {}", e))?;
        let y: i64 = y
            .parse()
            .map_err(|e| anyhow!("couldn't parse y coord: {}", e))?;
        Ok(Pos(x, y))
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, Pos(dx, dy): Self) -> Self::Output {
        Pos(self.0 + dx, self.1 + dy)
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, Pos(dx, dy): Self) {
        self.0 += dx;
        self.1 += dy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("(1,200)".parse::<Pos>().unwrap(), Pos(1, 200));
        assert_eq!("-1,200".parse::<Pos>().unwrap(), Pos(-1, 200));
    }

    #[test]
    fn test_distance() {
        assert_eq!(Pos(-1, -1).distance(&Pos(1, 1)), 4);
    }

    #[test]
    fn test_addition() {
        assert_eq!(Pos(-100, 200) + Pos(100, -200), Pos(0, 0));
    }

    #[test]
    fn test_iter() {
        assert_eq!(
            Pos(0, 0).iter_x(2).collect::<Vec<_>>(),
            vec![Pos(0, 0), Pos(1, 0), Pos(2, 0)]
        );
        assert_eq!(
            Pos(0, 0).iter_y(2).rev().collect::<Vec<_>>(),
            vec![Pos(0, 2), Pos(0, 1), Pos(0, 0)]
        );
        assert_eq!(
            Pos(0, -2).iter_y(0).collect::<Vec<_>>(),
            vec![Pos(0, -2), Pos(0, -1), Pos(0, 0)]
        );

        let mut iter = Pos(0, 0).iter_x(2);
        assert_eq!(iter.next(), Some(Pos(0, 0)));
        assert_eq!(iter.next_back(), Some(Pos(2, 0)));
        assert_eq!(iter.next_back(), Some(Pos(1, 0)));
        assert_eq!(iter.next(), None);
    }
}
