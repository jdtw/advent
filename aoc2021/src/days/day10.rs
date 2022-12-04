const INPUT: &str = "input/day10.txt";

pub fn solution() {
    let input = input::string(INPUT);
    let mut part1 = 0;
    let mut scores = Vec::new();
    for l in input.lines() {
        match check_syntax(l) {
            Check::SyntaxErr(t) => {
                part1 += t.syntax_score();
            }
            Check::Incomplete(completion) => {
                let mut score = 0;
                for c in completion {
                    score *= 5;
                    score += c.completion_score();
                }
                scores.push(score);
            }
        }
    }
    scores.sort();
    let part2 = scores[scores.len() / 2];
    println!("Part1: {}\nPart2: {}", part1, part2);
}

fn check_syntax(l: &str) -> Check {
    let mut stack = Vec::new();
    for c in l.chars() {
        match Chunk::from(c) {
            Chunk::Open(t) => stack.push(t),
            Chunk::Close(t) => {
                if stack.pop().filter(|top| top == &t).is_none() {
                    return Check::SyntaxErr(t);
                }
            }
        }
    }
    Check::Incomplete(stack.into_iter().rev().collect())
}

enum Check {
    SyntaxErr(Type),
    Incomplete(Vec<Type>),
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Round,
    Square,
    Curly,
    Angle,
}

enum Chunk {
    Open(Type),
    Close(Type),
}

impl Type {
    fn syntax_score(&self) -> u64 {
        use Type::*;
        match self {
            Round => 3,
            Square => 57,
            Curly => 1197,
            Angle => 25137,
        }
    }

    fn completion_score(&self) -> u64 {
        use Type::*;
        match self {
            Round => 1,
            Square => 2,
            Curly => 3,
            Angle => 4,
        }
    }
}

impl From<char> for Chunk {
    fn from(c: char) -> Self {
        use Chunk::*;
        use Type::*;
        match c {
            '(' => Open(Round),
            ')' => Close(Round),
            '[' => Open(Square),
            ']' => Close(Square),
            '{' => Open(Curly),
            '}' => Close(Curly),
            '<' => Open(Angle),
            '>' => Close(Angle),
            _ => unreachable!(),
        }
    }
}
