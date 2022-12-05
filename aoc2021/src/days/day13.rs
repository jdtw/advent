use std::{collections::HashSet, fmt::Display, str::FromStr};

const INPUT: &str = "input/day13.txt";

pub fn solution() {
    let Instructions(mut points, folds) = input::parse(INPUT);
    for (i, f) in folds.into_iter().enumerate() {
        points.fold(f);
        if i == 0 {
            let part1 = points.0.len();
            println!("Part1: {}", part1);
        }
    }
    println!("Part2:\n{}", points);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i64, i64);

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(i64),
    Y(i64),
}

struct Instructions(Points, Vec<Fold>);

struct Points(HashSet<Pos>);

impl Points {
    fn fold(&mut self, f: Fold) {
        match f {
            Fold::X(n) => self.fold_left(n),
            Fold::Y(n) => self.fold_up(n),
        }
    }

    fn fold_up(&mut self, n: i64) {
        let mut mirror = Vec::new();
        for p in self.0.iter() {
            if p.1 > n {
                mirror.push(*p);
            }
        }
        for p in mirror {
            self.0.remove(&p);
            self.0.insert(Pos(p.0, n - (p.1 - n)));
        }
    }

    fn fold_left(&mut self, n: i64) {
        let mut mirror = Vec::new();
        for p in self.0.iter() {
            if p.0 > n {
                mirror.push(*p);
            }
        }
        for p in mirror {
            self.0.remove(&p);
            self.0.insert(Pos(n - (p.0 - n), p.1));
        }
    }
}

impl FromStr for Instructions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = HashSet::new();
        let mut lines = s.lines();
        for l in lines.by_ref() {
            if l.is_empty() {
                break;
            }
            let (x, y) = l.split_once(',').unwrap();
            points.insert(Pos(x.parse().unwrap(), y.parse().unwrap()));
        }
        let folds = lines.map(|l| l.parse::<Fold>().unwrap()).collect();
        Ok(Instructions(Points(points), folds))
    }
}

impl FromStr for Fold {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("fold along ").unwrap();
        let (dim, n) = s.split_once('=').unwrap();
        let n = n.parse().unwrap();
        match dim {
            "x" => Ok(Fold::X(n)),
            "y" => Ok(Fold::Y(n)),
            _ => panic!("unknown dim {}", dim),
        }
    }
}

impl Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Points(points) = self;
        let xmax = points.iter().map(|p| p.0).max().unwrap();
        let ymax = points.iter().map(|p| p.1).max().unwrap();

        for y in 0..=ymax {
            for x in 0..=xmax {
                let c = points.get(&Pos(x, y)).map(|_| '#').unwrap_or(' ');
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
