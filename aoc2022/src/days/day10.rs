use std::str::FromStr;

const INPUT: &str = "input/day10.txt";

pub fn solution() {
    let instructions: Vec<Inst> = input::parse_lines(INPUT);
    let ticks: Vec<i64> = run(instructions).take(240).collect();
    let part1: i64 = ticks
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .take(6)
        .map(|(i, x)| (i as i64 + 1) * x)
        .sum();
    println!("Part1: {}", part1);

    for (i, sprite) in ticks.iter().enumerate() {
        let i = (i % 40) as i64;
        if i == 0 {
            println!();
        }
        if (sprite - 1..=sprite + 1).contains(&i) {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!()
}

fn run(mut instructions: Vec<Inst>) -> TickIter {
    instructions.reverse();
    TickIter {
        x: 1,
        prog: instructions,
    }
}

struct TickIter {
    x: i64,
    prog: Vec<Inst>,
}

#[derive(Copy, Clone)]
enum Inst {
    Noop,
    Addx(i64, usize),
}

impl Iterator for TickIter {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        let inst = self.prog.pop()?;
        // Always return the *current* value of the register, not the one at the
        // end of the cycle...
        let next = Some(self.x);
        // Execute the instruction.
        match inst {
            Inst::Noop => (),
            Inst::Addx(n, counter) if counter == 0 => {
                self.x += n;
            }
            Inst::Addx(n, counter) => {
                self.prog.push(Inst::Addx(n, counter - 1));
            }
        }
        next
    }
}

impl FromStr for Inst {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "noop" {
            Inst::Noop
        } else if let Some(n) = s.strip_prefix("addx ") {
            Inst::Addx(n.parse().unwrap(), 1)
        } else {
            panic!("Unknown instruction {}", s)
        })
    }
}
