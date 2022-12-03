use std::{collections::HashMap, str::FromStr};

const INPUT: &str = "input/day5.txt";

pub fn solution() {
    let lines: Vec<Line> = input::parse_lines(INPUT);

    let mut part1_map: HashMap<Pos, u32> = HashMap::new();
    let mut part2_map: HashMap<Pos, u32> = HashMap::new();
    for l in lines {
        for p in l.part1_iter() {
            *part1_map.entry(p).or_default() += 1;
        }
        for p in l.part2_iter() {
            *part2_map.entry(p).or_default() += 1;
        }
    }
    //    for y in 0..=9 {
    //        for x in 0..=9 {
    //            match part2_map.get(&Pos { x, y }) {
    //                Some(n) => print!("{}", n),
    //                None => print!("."),
    //            }
    //        }
    //        println!();
    //    }
    let part1 = part1_map.iter().filter(|e| e.1 > &1).count();
    let part2 = part2_map.iter().filter(|e| e.1 > &1).count();
    println!("Part1: {}\nPart2: {}", part1, part2);
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Line {
    a: Pos,
    b: Pos,
}

impl Line {
    fn part1_iter(&self) -> LineIter {
        LineIter {
            cur: self.a,
            dst: self.b,
            done: false,
            vertical: false,
        }
    }
    fn part2_iter(&self) -> LineIter {
        LineIter {
            cur: self.a,
            dst: self.b,
            done: false,
            vertical: true,
        }
    }
}

#[derive(Debug)]
struct LineIter {
    cur: Pos,
    dst: Pos,
    done: bool,
    vertical: bool,
}

impl Iterator for LineIter {
    type Item = Pos;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let cur = Some(self.cur);
        match (self.dst.x - self.cur.x, self.dst.y - self.cur.y) {
            (0, 0) => {
                self.done = true;
            }
            (0, _) => {
                self.cur.y += 1;
            }
            (_, 0) => {
                self.cur.x += 1;
            }
            _ if !self.vertical => {
                return None;
            }
            (_, dy) if dy < 0 => {
                self.cur.x += 1;
                self.cur.y -= 1;
            }
            _ => {
                self.cur.x += 1;
                self.cur.y += 1;
            }
        }
        cur
    }
}

impl FromStr for Pos {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Pos {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_once(" -> ").unwrap();
        let p1 = p1.parse().unwrap();
        let p2 = p2.parse().unwrap();
        if p1 < p2 {
            Ok(Line { a: p1, b: p2 })
        } else {
            Ok(Line { a: p2, b: p1 })
        }
    }
}
