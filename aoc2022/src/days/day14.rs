use pest::Parser;
use pest_derive::Parser;
use std::str::FromStr;

use util::pos::Pos;

const INPUT: &str = "input/day14.txt";

pub fn solution() {
    let paths: Vec<RockPath> = input::parse_lines(INPUT);
    println!("{:?}", paths);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_plus_one() {
        assert_eq!(1 + 1, 2);
    }
}
