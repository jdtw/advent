use std::str::FromStr;

const INPUT: &str = "input/day5.txt";

pub fn solution() {
    let mut input: Input = input::parse(INPUT);
    let part1 = input.clone().execute();
    println!("Part1: {}", part1);
    let part2 = input.execute_9001();
    println!("Part1: {}", part2);
}

#[derive(Debug, Default, Clone)]
struct Input {
    cranes: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Input {
    fn execute(&mut self) -> String {
        for &Instruction { count, from, to } in &self.instructions {
            for _ in 0..count {
                let item = self.cranes[from].pop().unwrap();
                self.cranes[to].push(item);
            }
        }
        self.top_crates()
    }

    fn execute_9001(&mut self) -> String {
        for &Instruction { count, from, to } in &self.instructions {
            let mut items = Vec::with_capacity(count);
            for _ in 0..count {
                items.push(self.cranes[from].pop().unwrap());
            }
            self.cranes[to].extend(items.into_iter().rev());
        }
        self.top_crates()
    }

    fn top_crates(&self) -> String {
        self.cranes.iter().map(|c| c.last().unwrap()).collect()
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut drawing = Vec::new();
        let mut lines = s.lines();
        for l in lines.by_ref() {
            if l.is_empty() {
                break;
            }
            let chars: Vec<char> = l.chars().collect();
            drawing.push(chars)
        }

        // Allocate the cranes.
        let num_cranes = drawing
            .pop()
            .unwrap()
            .into_iter()
            .filter(|c| !c.is_ascii_whitespace())
            .count();
        let mut cranes: Vec<Vec<char>> = vec![Vec::new(); num_cranes];

        // And fill them...
        drawing.reverse();
        for line in drawing {
            for (i, item) in line.iter().skip(1).step_by(4).enumerate() {
                if *item != ' ' {
                    cranes[i].push(*item);
                }
            }
        }
        let mut instructions = Vec::new();
        for l in lines {
            let inst: Instruction = l.parse().unwrap();
            instructions.push(inst);
        }
        Ok(Input {
            cranes,
            instructions,
        })
    }
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("move ")
            .unwrap()
            .replace("from ", "")
            .replace("to ", "");
        let mut split = s.split_ascii_whitespace();
        let count = split.next().unwrap().parse().unwrap();
        let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = split.next().unwrap().parse::<usize>().unwrap() - 1;
        Ok(Instruction { count, from, to })
    }
}
