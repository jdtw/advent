use std::str::FromStr;

const INPUT: &str = "input/day4.txt";

pub fn solution() {
    let assignments: Vec<Assignment> = input::parse_lines(INPUT);
    let mut part1 = 0;
    let mut part2 = 0;
    for a in assignments {
        if a.r1.contains(&a.r2) || a.r2.contains(&a.r1) {
            part1 += 1;
        }
        if a.r1.overlaps(&a.r2) {
            part2 += 1;
        }
    }
    println!("Part1: {}\nPart2: {}", part1, part2);
}

struct Assignment {
    r1: Range,
    r2: Range,
}

struct Range(u32, u32);

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.0..=self.1).contains(&other.0)
            || (self.0..=self.1).contains(&other.1)
            || (other.0..=other.1).contains(&self.0)
            || (other.0..=other.1).contains(&self.1)
    }
}

impl FromStr for Assignment {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r1, r2) = s.split_once(',').unwrap();
        Ok(Assignment {
            r1: r1.parse().unwrap(),
            r2: r2.parse().unwrap(),
        })
    }
}

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (begin, end) = s.split_once('-').unwrap();
        Ok(Range(begin.parse().unwrap(), end.parse().unwrap()))
    }
}
