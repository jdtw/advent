use std::{collections::HashMap, str::FromStr};

use pest::Parser;
use pest_derive::Parser;

const INPUT: &str = "input/day16.txt";

pub fn solution() {
    let cave: Cave = dbg!(input::parse(INPUT));
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Label(String);

#[derive(Parser)]
#[grammar_inline = r#"
valve  = { ASCII_ALPHA_UPPER{2} }
num    = { ASCII_DIGIT+ }
valves = { valve ~ (", " ~ valve)* }
rule   = { "Valve " ~ valve ~ " has flow rate=" ~ num ~ "; tunnels lead to valves " ~ valves }
rules  = { rule ~ ("\n" ~ rule)* }
"#]
struct ValveParser;

#[derive(Debug)]
struct Cave {
    flow_rates: HashMap<Label, u64>,
    tunnels: HashMap<Label, Vec<Label>>,
}

impl FromStr for Cave {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut flow_rates = HashMap::new();
        let mut tunnels = HashMap::new();
        let rules = ValveParser::parse(Rule::rules, s)
            .unwrap()
            .next()
            .unwrap()
            .into_inner();
        for rule in rules {
            let mut rule = rule.into_inner();
            let label = Label(rule.next().unwrap().as_str().to_owned());
            let flow_rate: u64 = rule.next().unwrap().as_str().parse().unwrap();
            let leads_to = rule
                .next()
                .unwrap()
                .into_inner()
                .map(|l| Label(l.as_str().to_owned()))
                .collect();
            flow_rates.insert(label.clone(), flow_rate);
            tunnels.insert(label, leads_to);
        }
        Ok(Cave {
            flow_rates,
            tunnels,
        })
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
