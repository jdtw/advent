use std::str::FromStr;
use util::pos::{Pos, PosSet};

const INPUT: &str = "input/day9.txt";

pub fn solution() {
    let instructions: Vec<Instruction> = input::parse_lines(INPUT);

    let mut part1 = Rope::new(2);
    let mut part2 = Rope::new(10);
    for i in &instructions {
        part1.mv(i);
        part2.mv(i);
    }
    println!(
        "Part1: {}\nPart2: {}",
        part1.visited.len(),
        part2.visited.len()
    );
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Pos>,
    visited: PosSet,
}

impl Rope {
    fn new(num_knots: usize) -> Self {
        Self {
            knots: vec![Pos(0, 0); num_knots],
            visited: PosSet::from([Pos(0, 0)]),
        }
    }

    fn mv(&mut self, Instruction(direction, magnitude): &Instruction) {
        let d = match direction {
            Direction::Up => Pos(0, 1),
            Direction::Down => Pos(0, -1),
            Direction::Left => Pos(-1, 0),
            Direction::Right => Pos(1, 0),
        };
        for _ in 0..*magnitude {
            self.knots[0] += d;
            let tail = self.knots.len() - 1;
            for i in 0..tail {
                self.follow(i, i + 1);
            }
            self.visited.insert(self.knots[tail]);
        }
    }

    fn follow(&mut self, head: usize, tail: usize) {
        let Pos(head_x, head_y) = self.knots[head];
        let Pos(tail_x, tail_y) = &mut self.knots[tail];
        let (dx, dy) = (head_x - *tail_x, head_y - *tail_y);
        let (dx_abs, dy_abs) = (dx.abs(), dy.abs());

        // The actual follow update should be clamped to 1 or -1.
        // Lazily evaluate to avoid divide by zero.
        let (follow_x, follow_y) = (|| dx / dx_abs, || dy / dy_abs);

        if dx == 0 && dy_abs == 2 {
            *tail_y += follow_y();
        } else if dy == 0 && dx_abs == 2 {
            *tail_x += follow_x();
        } else if dx_abs > 0 && dy_abs > 0 && dx_abs + dy_abs > 2 {
            *tail_x += follow_x();
            *tail_y += follow_y();
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction(Direction, usize);

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().next().unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            c => panic!("Unknown direction {}", c),
        })
    }
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, magnitude) = input::split2(s, " ");
        Ok(Self(direction, magnitude))
    }
}
