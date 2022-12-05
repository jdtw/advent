use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const INPUT: &str = "input/day12.txt";

pub fn solution() {
    let graph: Graph = input::parse(INPUT);
    let part1 = graph.paths(/*allow_second*/ false).len();
    let part2 = graph.paths(/*allow_second*/ true).len();
    println!("Part1: {}\nPart2: {}", part1, part2);
}

#[derive(Debug, Clone)]
struct Graph {
    edges: HashMap<String, Vec<String>>,
}

impl Graph {
    fn paths(&self, allow_twice: bool) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        let mut stack = vec![State::new(allow_twice)];
        while let Some(s) = stack.pop() {
            let cave = s.cave();
            if cave == "end" {
                paths.push(s.owned_path());
                continue;
            }
            for next in self.edges.get(cave).unwrap() {
                if let Some(s) = s.try_extend(next) {
                    stack.push(s);
                }
            }
        }
        paths
    }
}

#[derive(Debug)]
struct State<'graph> {
    // Note: Turns out we didn't need to track the paths themselves,
    // but it's useful for debugging.
    path: Vec<&'graph str>,
    visited: HashSet<&'graph str>,
    // Part1 doesn't allow a second visit. Part2 allows a second visit to
    // exactly one small cave.
    allow_twice: bool,
    second: Option<&'graph str>,
}

impl<'graph> State<'graph> {
    fn new(allow_twice: bool) -> Self {
        State {
            path: vec!["start"],
            visited: HashSet::from(["start"]),
            allow_twice,
            second: None,
        }
    }

    /// Returns None if we have already visited this cave.
    fn try_extend(&self, cave: &'graph str) -> Option<Self> {
        let second = if is_small(cave) && self.visited.contains(cave) {
            if !self.allow_twice
                || self.second.is_some()
                || cave == "start"
                || cave == "end"
            {
                return None;
            }
            Some(cave)
        } else {
            self.second
        };

        let mut path = self.path.clone();
        path.push(cave);

        let mut visited = self.visited.clone();
        if is_small(cave) {
            visited.insert(cave);
        }

        Some(State {
            path,
            visited,
            allow_twice: self.allow_twice,
            second,
        })
    }

    fn cave(&self) -> &'graph str {
        self.path.last().unwrap()
    }

    fn owned_path(self) -> Vec<String> {
        self.path.into_iter().map(String::from).collect()
    }
}

fn is_small(s: &str) -> bool {
    s.chars().next().unwrap().is_ascii_lowercase()
}

impl FromStr for Graph {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        for l in s.lines() {
            let Entry(a, b) = l.parse().unwrap();
            edges.entry(a.clone()).or_default().push(b.clone());
            edges.entry(b).or_default().push(a);
        }
        Ok(Graph { edges })
    }
}

struct Entry(String, String);
impl FromStr for Entry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-').unwrap();
        Ok(Entry(a.to_owned(), b.to_owned()))
    }
}
