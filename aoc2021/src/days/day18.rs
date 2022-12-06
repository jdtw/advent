use std::{fmt::Display, ops::Add, str::FromStr};

const INPUT: &str = "input/day18.txt";

pub fn solution() {
    let numbers: Vec<Number> = input::parse_lines(INPUT);
    let sum: Number = numbers
        .clone()
        .into_iter()
        .reduce(|acc, num| acc + num)
        .unwrap();
    let pair = Pair::from(sum);
    let part1 = pair.magnitude();
    println!("Part1: {}", part1);

    let mut part2 = 0;
    for (i, n) in numbers.iter().enumerate() {
        for (j, m) in numbers.iter().enumerate() {
            if i == j {
                continue;
            }
            let sum: Number = n.clone() + m.clone();
            let mag = Pair::from(sum).magnitude();
            if mag > part2 {
                part2 = mag;
            }
        }
    }
    println!("Part2: {}", part2);
}

/// Number is an intermediate representation. I can't reason about pairs,
/// but I can do the adding, exploding, and spliting with this IR.
#[derive(Debug, Clone)]
struct Number(Vec<Atom>);
#[derive(Debug, Clone, Copy)]
enum Atom {
    Open,
    Close,
    Num(u64),
}

/// Pair represents the nested structure of the actual number.
#[derive(Debug, Default, Clone)]
struct Pair {
    store: Vec<Item>,
    pair: (usize, usize),
}
#[derive(Debug, Clone, Copy)]
enum Item {
    Num(u64),
    Pair(usize, usize),
}

impl<'a> Pair {
    fn parse_item(&mut self, atoms: &'a [Atom]) -> (Item, &'a [Atom]) {
        match &atoms[0] {
            Atom::Num(n) => (Item::Num(*n), &atoms[1..]),
            Atom::Open => {
                let (first, atoms) = self.parse_item(&atoms[1..]);
                let (second, atoms) = self.parse_item(atoms);
                (
                    Item::Pair(self.store(first), self.store(second)),
                    &atoms[1..],
                )
            }
            _ => unreachable!(),
        }
    }

    fn store(&mut self, i: Item) -> usize {
        let id = self.store.len();
        self.store.push(i);
        id
    }

    fn item(&self, i: usize) -> &Item {
        &self.store[i]
    }

    fn magnitude(&self) -> u64 {
        self.magnitude_of(&Item::Pair(self.pair.0, self.pair.1))
    }

    fn magnitude_of(&self, i: &Item) -> u64 {
        match i {
            Item::Num(n) => *n,
            Item::Pair(first, second) => {
                (self.magnitude_of(self.item(*first)) * 3)
                    + (self.magnitude_of(self.item(*second)) * 2)
            }
        }
    }
}

impl From<Number> for Pair {
    fn from(Number(n): Number) -> Self {
        let mut pair = Pair::default();
        let (Item::Pair(first, second), atoms) = pair.parse_item(&n) else {
            panic!("expected a pair!!");
        };
        assert!(atoms.is_empty());
        pair.pair = (first, second);
        pair
    }
}

impl Number {
    fn reduce(&mut self) {
        loop {
            while self.explode_once() {}
            if !self.split_once() {
                break;
            }
        }
    }
    fn explode_once(&mut self) -> bool {
        let mut stack = Vec::new();
        let mut prev: Option<usize> = None;
        let mut open_count = 0;
        let mut items = self.0.iter();
        while let Some(i) = items.next() {
            match i {
                Atom::Open => {
                    open_count += 1;
                    if open_count > 4 {
                        // Consume the 'n,m]' of this pair...
                        let Atom::Num(lhs) = items.next().unwrap() else {
                            panic!("expected a num!");
                        };
                        let Atom::Num(rhs) = items.next().unwrap() else {
                            panic!("expected a num!");
                        };
                        items.next().unwrap();

                        // Add lhs to the previous num.
                        if let Some(idx) = prev.take() {
                            let Atom::Num(n) = &mut stack[idx] else {
                                panic!("expected a num!");
                            };
                            *n += lhs;
                        }

                        // Replace this pair with zero.
                        stack.push(Atom::Num(0));
                        let mut explode_rhs = Some(rhs);

                        // And now the rest of the list, incrementing the next number we find.
                        for i in items.by_ref() {
                            match i {
                                Atom::Num(n) if explode_rhs.is_some() => stack
                                    .push(Atom::Num(
                                        n + explode_rhs.take().unwrap(),
                                    )),
                                _ => stack.push(*i),
                            }
                        }
                        self.0 = stack;
                        return true;
                    }
                    stack.push(Atom::Open);
                }
                Atom::Close => {
                    open_count -= 1;
                    stack.push(Atom::Close);
                }
                Atom::Num(n) => {
                    prev = Some(stack.len());
                    stack.push(Atom::Num(*n));
                }
            }
        }
        false
    }

    fn split_once(&mut self) -> bool {
        let mut stack = Vec::new();
        let mut items = self.0.iter();
        while let Some(i) = items.next() {
            match i {
                Atom::Num(n) if n > &9 => {
                    let (lhs, rhs) = (n / 2, (n + 1) / 2);
                    stack.extend_from_slice(&[
                        Atom::Open,
                        Atom::Num(lhs),
                        Atom::Num(rhs),
                        Atom::Close,
                    ]);
                    for i in items.by_ref() {
                        stack.push(*i);
                    }
                    self.0 = stack;
                    return true;
                }
                _ => {
                    stack.push(*i);
                }
            }
        }
        false
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = vec![Atom::Open];
        output.extend(self.0.into_iter());
        output.extend(rhs.0.into_iter());
        output.push(Atom::Close);
        let mut n = Number(output);
        n.reduce();
        n
    }
}

impl FromStr for Number {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = Vec::new();
        let mut accum = Vec::new();
        for c in s.trim().chars() {
            match c {
                '[' => items.push(Atom::Open),
                ',' => {
                    if !accum.is_empty() {
                        let n = accum
                            .iter()
                            .collect::<String>()
                            .parse::<u64>()
                            .unwrap();
                        accum.clear();
                        items.push(Atom::Num(n));
                    }
                }
                ']' => {
                    if !accum.is_empty() {
                        let n = accum
                            .iter()
                            .collect::<String>()
                            .parse::<u64>()
                            .unwrap();
                        accum.clear();
                        items.push(Atom::Num(n));
                    }
                    items.push(Atom::Close);
                }
                _ => {
                    accum.push(c);
                }
            }
        }
        Ok(Number(items))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Atom::*;
        let mut prev = Atom::Open;
        for i in &self.0 {
            match (i, prev) {
                (&Open, Open) => write!(f, "[")?,
                (Open, _) => write!(f, ",[")?,
                (&Close, _) => write!(f, "]")?,
                (&Num(n), Open) => write!(f, "{}", n)?,
                (Num(n), _) => write!(f, ",{}", n)?,
            }
            prev = *i;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let lhs = Number::from_str("[1,2]").unwrap();
        let rhs = Number::from_str("[[3,4],5]").unwrap();
        let sum = lhs + rhs;
        assert_eq!(format!("{}", sum), "[[1,2],[[3,4],5]]");

        let lhs = Number::from_str("[[[1,3],[5,3]],[[1,3],[8,7]]]").unwrap();
        let rhs = Number::from_str("[[[4,9],[6,9]],[[8,2],[7,3]]]").unwrap();
        let sum = lhs + rhs;
        assert_eq!(
            format!("{}", sum),
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"
        );

        let lhs = Number::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let rhs = Number::from_str("[1,1]").unwrap();
        let sum = lhs + rhs;
        assert_eq!(format!("{}", sum), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_explode() {
        let mut n = Number::from_str("[[[[[9,8],1],2],3],4]").unwrap();
        n.explode_once();
        assert_eq!(format!("{}", n), "[[[[0,9],2],3],4]");

        let mut n = Number::from_str("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        n.explode_once();
        assert_eq!(format!("{}", n), "[7,[6,[5,[7,0]]]]");

        let mut n =
            Number::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        n.explode_once();
        assert_eq!(format!("{}", n), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        let mut n =
            Number::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
        n.explode_once();
        assert_eq!(format!("{}", n), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_magnitude() {
        let n = Number::from_str(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        )
        .unwrap();
        let p = Pair::from(n);
        assert_eq!(p.magnitude(), 3488);
    }
}
