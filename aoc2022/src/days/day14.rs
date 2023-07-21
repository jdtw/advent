use pest::Parser;
use pest_derive::Parser;
use std::str::FromStr;

use util::pos::{Pos, PosMap};

const INPUT: &str = "input/day14.txt";

pub fn solution() {
    let paths: Vec<RockPath> = input::parse_lines(INPUT);
    let mut cave = Cave::from(paths.as_slice());
    let mut part1 = 0;
    while cave.pour(Part::One).is_some() {
        part1 += 1;
    }
    println!("Part1: {part1}");

    let mut cave = Cave::from(paths.as_slice());
    let mut part2 = 0;
    while let Some(pos) = cave.pour(Part::Two) {
        part2 += 1;
        if pos == Pos(500, 0) {
            break;
        }
    }
    println!("Part2: {part2}");
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Rock,
    Sand,
}

#[derive(Debug, Clone)]
struct Cave {
    cells: PosMap<Cell>,
    y_max: i64,
}

enum Part {
    One,
    Two,
}

impl Cave {
    fn pour(&mut self, part: Part) -> Option<Pos> {
        let mut x = 500;
        let y_max = match part {
            Part::One => self.y_max,
            Part::Two => self.y_max + 2,
        };
        for y in 0..y_max {
            let (left, down, right) =
                if matches!(part, Part::Two) && y + 1 == y_max {
                    // Quick, fill in the floor!
                    (Some(&Cell::Rock), Some(&Cell::Rock), Some(&Cell::Rock))
                } else {
                    (
                        self.cells.get(&Pos(x - 1, y + 1)),
                        self.cells.get(&Pos(x, y + 1)),
                        self.cells.get(&Pos(x + 1, y + 1)),
                    )
                };
            match (left, down, right) {
                (_, None, _) => {
                    // Keep falling down...
                }
                (None, Some(_), _) => {
                    // Move to the left...
                    x -= 1;
                }
                (Some(_), Some(_), None) => {
                    // Move to the right...
                    x += 1;
                }
                (Some(_), Some(_), Some(_)) => {
                    // Come to rest!
                    self.cells.insert(Pos(x, y), Cell::Sand);
                    return Some(Pos(x, y));
                }
            }
        }
        return None;
    }
}

impl From<&[RockPath]> for Cave {
    fn from(paths: &[RockPath]) -> Self {
        let mut y_max = 0;
        let mut cells = PosMap::new();
        for RockPath(path) in paths {
            for pair in path.windows(2) {
                let p1 = &pair[0];
                let p2 = &pair[1];
                y_max = y_max.max(p1.1).max(p2.1);
                let Pos(dx, dy) = *p1 - *p2;
                let iter = match (dx, dy) {
                    (0, dy) if dy > 0 => p2.iter_y(p1.1),
                    (0, _) => p1.iter_y(p2.1),
                    (dx, 0) if dx > 0 => p2.iter_x(p1.0),
                    (_, 0) => p1.iter_x(p2.0),
                    _ => unreachable!(),
                };
                for p in iter {
                    cells.insert(p, Cell::Rock);
                }
            }
        }
        Cave { cells, y_max }
    }
}

#[derive(Parser)]
#[grammar_inline = r#"
num  = { ASCII_DIGIT+ }
pos  = { num ~ "," ~ num }
path = { pos ~ (" -> " ~ pos)* }
"#]
struct RockPathParser;

#[derive(Debug)]
struct RockPath(Vec<Pos>);

impl FromStr for RockPath {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = RockPathParser::parse(Rule::path, s)
            .unwrap()
            .next()
            .unwrap()
            .into_inner();
        let mut points = Vec::new();
        for pos in path {
            let mut pos = pos.into_inner();
            let x = pos.next().unwrap().as_str();
            let y = pos.next().unwrap().as_str();
            points.push(Pos(x.parse().unwrap(), y.parse().unwrap()));
        }
        Ok(RockPath(points))
    }
}
