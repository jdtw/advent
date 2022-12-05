use std::collections::{BinaryHeap, HashMap};

use util::pos::{DigitGrid, Pos, PosMap};

const INPUT: &str = "input/day15.txt";

pub fn solution() {
    let grid: DigitGrid<u8> = input::parse(INPUT);
    let mut cave = Cave::from(grid);
    let part1 = cave.min_path();
    println!("Part1: {}", part1);
    cave.expand();
    let part2 = cave.min_path();
    println!("Part2: {}", part2);
}

struct Cave(PosMap<u8>);

impl From<DigitGrid<u8>> for Cave {
    fn from(DigitGrid(g): DigitGrid<u8>) -> Self {
        Cave(g)
    }
}

impl Cave {
    // This is just Djikstra. Based on the example at https://doc.rust-lang.org/std/collections/binary_heap/index.html.
    fn min_path(&self) -> u64 {
        let end = self.end();
        let mut score: HashMap<_, _> =
            self.0.keys().cloned().map(|p| (p, u64::MAX)).collect();
        let mut heap = BinaryHeap::from([State::default()]);

        while let Some(s) = heap.pop() {
            if s.cur == end {
                return s.risk;
            }

            // have we already found something better?
            if s.risk > *score.get(&s.cur).unwrap() {
                continue;
            }

            for n in s.cur.compass_neighbors() {
                if let Some(risk) = self.0.get(&n) {
                    let next = State {
                        risk: *risk as u64 + s.risk,
                        cur: n,
                    };
                    // Only continue if this is the fastest way to get
                    // to position n!
                    let current_risk = score.get_mut(&n).unwrap();
                    if next.risk < *current_risk {
                        heap.push(next);
                        *current_risk = next.risk;
                    }
                }
            }
        }
        unreachable!()
    }

    fn end(&self) -> Pos {
        *self.0.keys().max().unwrap()
    }

    fn expand(&mut self) {
        let mut expanded = PosMap::new();
        let Pos(xmax, ymax) = self.end();
        for i in 0..5 {
            let dy = (ymax + 1) * i;
            for j in 0..5 {
                let dx = (xmax + 1) * j;
                let dr = i as u8 + j as u8;
                for (Pos(x, y), risk) in self.0.iter() {
                    let risk = (risk + dr - 1) % 9 + 1;
                    expanded.insert(Pos(x + dx, y + dy), risk);
                }
            }
        }
        self.0 = expanded;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct State {
    risk: u64,
    cur: Pos,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.cur.cmp(&other.cur))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            risk: 0,
            cur: Pos(0, 0),
        }
    }
}
