use input::{input_path, parse_lines, CharVec};

const INPUT: &str = "input/day03.txt";

pub fn solution() {
  let path = input_path(INPUT);
  let map: Vec<CharVec<Cell>> = parse_lines(path);
  let part1 = count_trees(&map, 3, 1);
  println!("part1: {}", part1);
  let part2 = count_trees(&map, 1, 1)
    * count_trees(&map, 3, 1)
    * count_trees(&map, 5, 1)
    * count_trees(&map, 7, 1)
    * count_trees(&map, 1, 2);
  println!("part2: {}", part2);
}

fn count_trees(map: &[CharVec<Cell>], right: usize, down: usize) -> u64 {
  let mut trees = 0;
  for (x, y) in Pos::new(right, down) {
    if y >= map.len() {
      break;
    }
    let CharVec(row) = &map[y];
    trees += match row[x % row.len()] {
      Cell::Empty => 0,
      Cell::Tree => 1,
    };
  }
  trees
}

struct Pos {
  row: usize,
  col: usize,
  right: usize,
  down: usize,
}

impl Pos {
  fn new(right: usize, down: usize) -> Pos {
    Pos {
      row: 0,
      col: 0,
      right,
      down,
    }
  }
}

impl Iterator for Pos {
  type Item = (usize, usize);
  fn next(&mut self) -> Option<Self::Item> {
    let next = (self.row, self.col);
    self.row += self.right;
    self.col += self.down;
    Some(next)
  }
}

enum Cell {
  Tree,
  Empty,
}

impl From<char> for Cell {
  fn from(c: char) -> Self {
    match c {
      '.' => Cell::Empty,
      '#' => Cell::Tree,
      _ => panic!("Invalid cell {}", c),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_pos() {
    let mut p = Pos::new(3, 1);
    assert_eq!(p.next(), Some((0, 0)));
    assert_eq!(p.next(), Some((3, 1)));
  }
}
