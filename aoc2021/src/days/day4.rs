use std::{collections::HashSet, str::FromStr};

const INPUT: &str = "input/day4.txt";

pub fn solution() {
    let mut bingo: Bingo = input::parse(input::path(INPUT));

    let mut won = HashSet::new();
    let mut first = 0;
    let mut last = 0;
    for n in bingo.rand {
        for (i, b) in bingo.boards.iter_mut().enumerate() {
            if won.contains(&i) {
                continue;
            }
            if let Some(win) = b.mark(n) {
                if first == 0 {
                    first = win;
                }
                last = win;
                won.insert(i);
            }
        }
    }
    println!("Part1: {}\nPart2: {}", first, last);
}

#[derive(Debug)]
struct Bingo {
    rand: Vec<u8>,
    boards: Vec<Board>,
}

#[derive(Default, Debug)]
struct Board {
    board: Vec<Vec<u8>>,
    marks: Vec<(usize, usize)>,
}

impl Board {
    fn mark(&mut self, n: u8) -> Option<u64> {
        for x in 0..5 {
            for y in 0..5 {
                if self.board[x][y] == n {
                    self.marks.push((x, y));
                    if self.has_bingo() {
                        return Some(n as u64 * self.unmarked_sum());
                    }
                }
            }
        }
        None
    }

    fn has_bingo(&self) -> bool {
        for i in 0..5 {
            // Count the number of marks in the row.
            let row = self.marks.iter().filter(|row| row.0 == i).count();
            if row == 5 {
                return true;
            }
            // Count the number of marks in the col.
            let col = self.marks.iter().filter(|row| row.1 == i).count();
            if col == 5 {
                return true;
            }
        }
        false
    }

    fn unmarked_sum(&self) -> u64 {
        let marked = self.marks.iter().collect::<HashSet<_>>();
        let mut sum = 0;
        for x in 0..5 {
            for y in 0..5 {
                if !marked.contains(&(x, y)) {
                    sum += self.board[x][y] as u64;
                }
            }
        }
        sum
    }
}

impl FromStr for Bingo {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // first line is the list of "random" draws.
        let csv = lines.next().unwrap();
        let rand = input::csv_str(csv);

        let mut boards = Vec::new();
        while lines.next().is_some() {
            let mut board = Board::default();
            for _ in 0..5 {
                let row = lines.next().unwrap();
                let row = input::space_str(row);
                board.board.push(row);
            }
            boards.push(board);
        }

        Ok(Bingo { rand, boards })
    }
}
