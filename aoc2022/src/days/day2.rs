use std::str::FromStr;

use anyhow::bail;

const INPUT: &str = "input/day2.txt";

pub fn solution() {
    let rounds: Vec<Round> = input::parse_lines(INPUT);
    let mut part1 = 0;
    let mut part2 = 0;
    for r in rounds {
        part1 += r.part1_score();
        part2 += r.part2_score();
    }
    println!("part1: {}\npart2: {}", part1, part2);
}

#[derive(Debug, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn vs(&self, opponent: Shape) -> Outcome {
        use Shape::*;
        match (self, &opponent) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => {
                Outcome::Win
            }
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => {
                Outcome::Draw
            }
            _ => Outcome::Lose,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Shape,
    you: char,
}

impl Round {
    fn part1_score(&self) -> u64 {
        let you = match self.you {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => unreachable!(),
        };
        let outcome = you.vs(self.opponent);
        outcome.score() + you.score()
    }

    fn part2_score(&self) -> u64 {
        use Outcome::*;
        use Shape::*;
        let (outcome, you) = match (self.you, self.opponent) {
            // X means you need to lose,
            ('X', Rock) => (Lose, Scissors),
            ('X', Paper) => (Lose, Rock),
            ('X', Scissors) => (Lose, Paper),
            // Y means you need to end the round in a draw,
            ('Y', Rock) => (Draw, Rock),
            ('Y', Paper) => (Draw, Paper),
            ('Y', Scissors) => (Draw, Scissors),
            // and Z means you need to win.
            ('Z', Rock) => (Win, Paper),
            ('Z', Paper) => (Win, Scissors),
            ('Z', Scissors) => (Win, Rock),
            _ => unreachable!(),
        };
        //assert_eq!(outcome, you.vs(self.opponent));
        outcome.score() + you.score()
    }
}

impl FromStr for Round {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let round = input::space_str::<char>(s);
        Ok(Round {
            opponent: match round[0] {
                'A' => Shape::Rock,
                'B' => Shape::Paper,
                'C' => Shape::Scissors,
                _ => bail!("Unknown shape"),
            },
            you: round[1],
        })
    }
}
