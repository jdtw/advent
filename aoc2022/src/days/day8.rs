use util::pos::{DigitGrid, Pos, PosMap, PosSet};

const INPUT: &str = "input/day8.txt";

pub fn solution() {
    let trees: DigitGrid<u8> = input::parse(INPUT);
    let forest = Forest::from(trees);

    let visible = forest.visible();
    println!("Part1: {}", visible.len());

    let part2 = forest.max_scenic_score();
    println!("Part2: {}", part2);
}

struct Forest {
    trees: PosMap<u8>,
    max: Pos,
}

impl Forest {
    fn visible(&self) -> PosSet {
        let mut visible = PosSet::new();
        for row in 0..=self.max.1 {
            visible.extend(self.visible_in_row(row));
        }
        for col in 0..=self.max.0 {
            visible.extend(self.visible_in_col(col));
        }
        visible
    }

    /// Returns the union of trees that can be seen from the left
    /// and trees that can be seen from the right in this row.
    fn visible_in_row(&self, row: i64) -> PosSet {
        let p = Pos(0, row);
        let mut visible = PosSet::new();
        visible.extend(self.visible_from(p.iter_x(self.max.0)));
        visible.extend(self.visible_from(p.iter_x(self.max.0).rev()));
        visible
    }

    /// Returns the union of trees that can be seem from the top
    /// and trees that can be seen from the bottom in this col.
    fn visible_in_col(&self, col: i64) -> PosSet {
        let p = Pos(col, 0);
        let mut visible = PosSet::new();
        visible.extend(self.visible_from(p.iter_y(self.max.1)));
        visible.extend(self.visible_from(p.iter_y(self.max.1).rev()));
        visible
    }

    /// Returns the set of trees visible from this single direction.
    fn visible_from(&self, iter: impl Iterator<Item = Pos>) -> PosSet {
        let mut visible = PosSet::new();
        let mut max = -1;
        for p in iter {
            let height = self.get_height(p) as i16;
            if height > max {
                visible.insert(p);
                max = height;
            }
        }
        visible
    }

    fn max_scenic_score(&self) -> u64 {
        self.trees
            .keys()
            .copied()
            .map(|p| self.scenic_score(p))
            .max()
            .unwrap()
    }

    fn scenic_score(&self, from: Pos) -> u64 {
        let up = Pos(from.0, 0).iter_y(from.1).rev();
        let down = from.iter_y(self.max.1);
        let left = Pos(0, from.1).iter_x(from.0).rev();
        let right = from.iter_x(self.max.0);

        let height = self.get_height(from);
        self.score_range(height, up)
            * self.score_range(height, down)
            * self.score_range(height, left)
            * self.score_range(height, right)
    }

    /// Returns the scenic score of the given range [from, to].
    fn score_range(&self, height: u8, range: impl Iterator<Item = Pos>) -> u64 {
        let mut count = 0;
        for p in range.skip(1) {
            count += 1;
            if self.get_height(p) >= height {
                break;
            }
        }
        count
    }

    fn get_height(&self, p: Pos) -> u8 {
        *self.trees.get(&p).unwrap()
    }
}

impl From<DigitGrid<u8>> for Forest {
    fn from(trees: DigitGrid<u8>) -> Self {
        let max = trees.max();
        Self {
            trees: trees.0,
            max,
        }
    }
}
