//const INPUT: &str = "input/day12_test.txt";
const INPUT: &str = "input/day12.txt";
use std::{collections::VecDeque, str::FromStr};

use anyhow::anyhow;
use input::{parse_str_lines, CharVec};
use util::pos::{Pos, PosMap, PosSet};

pub fn solution() {
    let map: Heights = input::parse(INPUT);
    let part1 = map.ascend();
    let part2 = map.descend();
    println!("Part1: {part1}\nPart2: {part2}");
}

#[derive(Default, Debug)]
struct Heights {
    begin: Pos,
    end: Pos,
    heights: PosMap<u8>,
}

struct Path {
    len: usize,
    pos: Pos,
}

impl Heights {
    fn ascend(&self) -> usize {
        let mut visited = PosSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(Path {
            len: 0,
            pos: self.begin,
        });
        visited.insert(self.begin);
        while let Some(path) = queue.pop_front() {
            // Base case: we reached the end!
            if path.pos == self.end {
                // BFS means this is the shortest path.
                return path.len;
            }

            // Visit reachable neighbors that aren't already in the queue.
            let current_height = self.heights.get(&path.pos).unwrap();
            for n in path.pos.compass_neighbors() {
                if visited.contains(&n) {
                    continue;
                }
                if let Some(h) = self.heights.get(&n) {
                    let dh = *h as i8 - *current_height as i8;
                    if dh > 1 {
                        continue;
                    }
                    queue.push_back(Path {
                        len: path.len + 1,
                        pos: n,
                    });
                    visited.insert(n);
                }
            }
        }
        panic!(
            "Couldn't find a path from {:?} to {:?}",
            self.begin, self.end
        )
    }

    fn descend(&self) -> usize {
        let mut visited = PosSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(Path {
            len: 0,
            pos: self.end,
        });
        visited.insert(self.begin);
        while let Some(path) = queue.pop_front() {
            let current_height = self.heights.get(&path.pos).unwrap();

            // Base case: we found the closest 'a'.
            if *current_height == 0 {
                // BFS means this is the shortest path.
                return path.len;
            }

            // Visit reachable neighbors that aren't already in the queue.
            for n in path.pos.compass_neighbors() {
                if visited.contains(&n) {
                    continue;
                }
                if let Some(h) = self.heights.get(&n) {
                    let dh = *current_height as i8 - *h as i8;
                    if dh > 1 {
                        continue;
                    }
                    queue.push_back(Path {
                        len: path.len + 1,
                        pos: n,
                    });
                    visited.insert(n);
                }
            }
        }
        panic!(
            "Couldn't find a path from {:?} to {:?}",
            self.begin, self.end
        )
    }
}

impl FromStr for Heights {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Heights::default();
        let heights: Vec<CharVec<char>> = parse_str_lines(s);
        for (y, CharVec(h)) in heights.into_iter().enumerate() {
            for (x, c) in h.into_iter().enumerate() {
                let pos = Pos(x as i64, y as i64);
                match c {
                    'S' => {
                        map.begin = pos;
                        map.heights.insert(pos, 0);
                    }
                    'E' => {
                        map.end = pos;
                        map.heights.insert(pos, 'z' as u8 - 'a' as u8);
                    }
                    'a'..='z' => {
                        map.heights.insert(pos, c as u8 - 'a' as u8);
                    }
                    _ => return Err(anyhow!("Invalid char {:?}", c)),
                }
            }
        }
        Ok(map)
    }
}
