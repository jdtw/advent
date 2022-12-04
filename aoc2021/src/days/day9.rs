use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const INPUT: &str = "input/day9.txt";

pub fn solution() {
    let map: Cave = input::parse(INPUT);
    let mut part1 = 0;
    let mut basins = Vec::new();
    for &p in map.height.keys() {
        if let Some(risk) = map.low_point(p) {
            part1 += risk;
            basins.push(map.basin_size(p))
        }
    }
    basins.sort();
    let part2 = basins
        .into_iter()
        .rev()
        .take(3)
        .reduce(|accum, item| accum * item)
        .unwrap();
    println!("Part1: {}\nPart2: {}", part1, part2);
}

#[derive(Debug)]
struct Cave {
    height: HashMap<Pos, u32>,
}

impl Cave {
    fn low_point(&self, p: Pos) -> Option<u64> {
        let h = self.height.get(&p).unwrap();
        if p.neighbors()
            .iter()
            .filter_map(|p| self.height.get(p))
            .all(|n| h < n)
        {
            Some(*h as u64 + 1)
        } else {
            None
        }
    }

    fn basin_size(&self, p: Pos) -> usize {
        let mut stack = vec![p];
        let mut visited = HashSet::new();
        while let Some(cur) = stack.pop() {
            visited.insert(cur);
            cur.neighbors()
                .into_iter()
                .filter(|p| match self.height.get(p) {
                    Some(h) if h < &9 => !visited.contains(p),
                    _ => false,
                })
                .for_each(|p| stack.push(p));
        }
        visited.len()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Pos(i64, i64);

impl Pos {
    fn neighbors(&self) -> [Pos; 4] {
        let &Pos(x, y) = self;
        [Pos(x + 1, y), Pos(x - 1, y), Pos(x, y + 1), Pos(x, y - 1)]
    }
}

impl FromStr for Cave {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut height = HashMap::new();
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let h = c.to_digit(10).unwrap();
                height.insert(Pos(x as i64, y as i64), h);
            }
        }
        Ok(Cave { height })
    }
}
