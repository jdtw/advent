use std::{cmp::Ordering, str::FromStr};

use util::pos::Pos;

const INPUT: &str = "input/day17.txt";

pub fn solution() {
    let target: Target = input::parse(INPUT);

    let mut part1 = i64::MIN;
    let mut part2 = 0;
    for x in 0..target.xmax * 2 {
        for y in target.ymin * 2..=-target.ymin * 2 {
            let mut probe = Probe::new(Pos(x, y), target);
            let mut maxy = i64::MIN;
            loop {
                match probe.step() {
                    Status::OnTarget(pos) => {
                        if pos.1 > maxy {
                            maxy = pos.1;
                        }
                    }
                    Status::Hit => {
                        if maxy > part1 {
                            part1 = maxy
                        }
                        part2 += 1;
                        break;
                    }
                    Status::Overshot => {
                        break;
                    }
                }
            }
        }
    }

    println!("Part1: {}\nPart2: {}", part1, part2);
}

struct Probe {
    pos: Pos,
    velocity: Pos,
    target: Target,
}

enum Status {
    OnTarget(Pos),
    Hit,
    Overshot,
}

impl Probe {
    fn new(velocity: Pos, target: Target) -> Self {
        Probe {
            pos: Pos::default(),
            velocity,
            target,
        }
    }

    fn step(&mut self) -> Status {
        self.pos += self.velocity;
        let Pos(x, y) = self.velocity;
        let x = match x.cmp(&0) {
            Ordering::Less => x + 1,
            Ordering::Equal => 0,
            Ordering::Greater => x - 1,
        };
        self.velocity = Pos(x, y - 1);
        if self.target.contains(self.pos) {
            return Status::Hit;
        }
        if self.target.overshot(self.pos) {
            return Status::Overshot;
        }
        Status::OnTarget(self.pos)
    }
}

#[derive(Debug, Clone, Copy)]
struct Target {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}

impl Target {
    fn contains(&self, p: Pos) -> bool {
        (self.xmin..=self.xmax).contains(&p.0)
            && (self.ymin..=self.ymax).contains(&p.1)
    }

    fn overshot(&self, p: Pos) -> bool {
        // This is a hack and assumes the target is somewhere below!
        p.0 > self.xmax || p.1 < self.ymin
    }
}

impl FromStr for Target {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //target area: x=85..145, y=-163..-108
        let s = s
            .trim()
            .strip_prefix("target area: x=")
            .unwrap()
            .replace(" y=", "");
        let (x, y) = s.split_once(',').unwrap();
        let (xmin, xmax) = x.split_once("..").unwrap();
        let (ymin, ymax) = y.split_once("..").unwrap();
        Ok(Target {
            xmin: xmin.parse().unwrap(),
            xmax: xmax.parse().unwrap(),
            ymin: ymin.parse().unwrap(),
            ymax: ymax.parse().unwrap(),
        })
    }
}
