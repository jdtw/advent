use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use pest::Parser;
use pest_derive::Parser;

//const INPUT: &str = "input/day16_test.txt";
const INPUT: &str = "input/day16.txt";

pub fn solution() {
    let cave: Cave = input::parse(INPUT);
    let part1 = cave.release_pressure();
    println!("Part1: {part1}");
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Label(char, char);

#[derive(Debug)]
struct Cave {
    flow_rates: HashMap<Label, u64>,
    tunnels: HashMap<Label, Vec<Label>>,
}

#[derive(Debug, Clone)]
struct State {
    at: Label,
    elapsed: u8,
    released: u64,
    open: HashSet<Label>,
    // Track how many valves we've opened during previous visits.
    // Don't bother going back to a valve if all we've done is
    // waste time but haven't increased the pressure we've released.
    visits: HashMap<Label, usize>,
}

// TODO: Can I track state globally instead of per-path?
// If I do depth first, I should be able to prune paths where
// I've released more pressure in less time for any given valve?

impl Cave {
    fn release_pressure(&self) -> u64 {
        let mut max = 0;
        let mut queue = VecDeque::new();
        queue.push_back(State {
            at: Label('A', 'A'),
            elapsed: 0,
            released: 0,
            open: HashSet::new(),
            visits: HashMap::new(),
        });
        while let Some(state) = queue.pop_front() {
            println!("{}", state.elapsed);
            // Base case -- our thirty minutes is up.
            if state.elapsed == 30 {
                if state.released > max {
                    max = state.released
                }
                continue;
            }
            // Try opening this valve if the flow rate is greater than zero.
            if !state.open.contains(&state.at) {
                let rate = self.flow_rate(&state.at);
                if rate > 0 {
                    let mut new = state.clone();
                    new.open.insert(new.at);
                    new.elapsed += 1;
                    let remaining = 30 - new.elapsed;
                    new.released += rate * remaining as u64;
                    queue.push_back(new);
                }
            }
            // Try each of the tunnels.
            for l in self.tunnels.get(&state.at).unwrap() {
                if let Some(previous) = state.visits.get(l) {
                    if *previous == state.open.len() {
                        // This would be a waste of time. Don't go back.
                        continue;
                    }
                }
                let mut new = state.clone();
                new.elapsed += 1;
                new.at = *l;
                // We're going to a new valve. Track how many we've
                // opened at this point.
                new.visits.insert(state.at, state.open.len());
                queue.push_back(new);
            }
        }
        max
    }

    fn flow_rate(&self, label: &Label) -> u64 {
        *self.flow_rates.get(label).unwrap()
    }
}

#[derive(Parser)]
#[grammar_inline = r#"
valve  = { ASCII_ALPHA_UPPER{2} }
num    = { ASCII_DIGIT+ }
valves = { valve ~ (", " ~ valve)* }
rule   = { "Valve " ~ valve ~ " has flow rate=" ~ num ~ ("; tunnel leads to valve " | "; tunnels lead to valves ") ~ valves }
rules  = { rule ~ ("\n" ~ rule)* }
"#]
struct ValveParser;

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
            let mut chars = rule.next().unwrap().as_str().chars();
            let label = Label(chars.next().unwrap(), chars.next().unwrap());
            let flow_rate: u64 = rule.next().unwrap().as_str().parse().unwrap();
            let leads_to = rule
                .next()
                .unwrap()
                .into_inner()
                .map(|l| {
                    let mut chars = l.as_str().chars();
                    Label(chars.next().unwrap(), chars.next().unwrap())
                })
                .collect();
            flow_rates.insert(label, flow_rate);
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
