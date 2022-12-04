use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

const INPUT: &str = "input/day11.txt";

pub fn solution() {
    let mut octos: Octos = input::parse(INPUT);
    let mut part1 = 0;
    for _ in 0..100 {
        part1 += octos.step();
    }
    println!("Part1: {}", part1);
    while octos.step() != octos.len() {}
    println!("Part2: {}", octos.counter);
}

#[derive(Debug)]
struct Octos {
    octos: HashMap<Pos, u8>,
    counter: usize,
}

impl Octos {
    fn step(&mut self) -> usize {
        // First, the energy level of each octopus increases by 1.

        // Then, any octopus with an energy level greater than 9 flashes.
        // This increases the energy level of all adjacent octopuses by 1,
        // including octopuses that are diagonally adjacent. If this causes
        // an octopus to have an energy level greater than 9, it also flashes.
        // This process continues as long as new octopuses keep having their
        // energy level increased beyond 9. (An octopus can only flash at most
        // once per step.)

        // Finally, any octopus that flashed during this step has its energy
        // level set to 0, as it used all of its energy to flash.
        let mut flashes = HashSet::new();
        let mut stack: Vec<Pos> = self.octos.keys().copied().collect();
        while let Some(p) = stack.pop() {
            let v = self.octos.get_mut(&p).unwrap();
            *v += 1;
            if *v > 9 && !flashes.contains(&p) {
                flashes.insert(p);
                for n in p.neighbors() {
                    if self.octos.contains_key(&n) {
                        stack.push(n);
                    }
                }
            }
        }

        for p in flashes.iter() {
            self.octos.entry(*p).and_modify(|v| *v = 0);
        }
        self.counter += 1;
        flashes.len()
    }

    #[allow(dead_code)]
    fn view(&self, n: i64) -> OctosView {
        let mut rows = Vec::new();
        for y in 0..n {
            let mut row = Vec::new();
            for x in 0..n {
                row.push(*self.octos.get(&Pos(x, y)).unwrap());
            }
            rows.push(row);
        }
        OctosView(rows)
    }

    fn len(&self) -> usize {
        self.octos.len()
    }
}

struct OctosView(Vec<Vec<u8>>);

impl Display for OctosView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let OctosView(rows) = self;
        for row in rows {
            for o in row {
                write!(f, "{}", o)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Pos(i64, i64);

impl Pos {
    fn neighbors(&self) -> [Pos; 8] {
        let &Pos(x, y) = self;
        [
            Pos(x + 1, y),
            Pos(x + 1, y + 1),
            Pos(x, y + 1),
            Pos(x - 1, y + 1),
            Pos(x - 1, y),
            Pos(x - 1, y - 1),
            Pos(x, y - 1),
            Pos(x + 1, y - 1),
        ]
    }
}

impl FromStr for Octos {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut octos = HashMap::new();
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let o = c.to_digit(10).unwrap();
                octos.insert(Pos(x as i64, y as i64), o as u8);
            }
        }
        Ok(Octos { octos, counter: 0 })
    }
}
