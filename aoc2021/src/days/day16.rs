use std::str::FromStr;

use anyhow::anyhow;

const INPUT: &str = "input/day16.txt";

pub fn solution() {
    let t: Transmission = input::parse(INPUT);
    let outermost = Outermost::from(t);
    let part1 = outermost.sum_versions();
    println!("Part1: {}", part1);
    let part2 = outermost.val();
    println!("Part2: {}", part2);
}

#[derive(Debug)]
enum Op {
    Literal(u64),
    Sum(Vec<usize>),
    Mul(Vec<usize>),
    Min(Vec<usize>),
    Max(Vec<usize>),
    Gt(Vec<usize>),
    Lt(Vec<usize>),
    Eq(Vec<usize>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    op: Op,
}

#[derive(Debug)]
struct Outermost {
    store: Vec<Packet>,
    packet: Packet,
}

#[derive(Debug, Default)]
struct Transmission {
    bits: Vec<u8>,
    cursor: usize,
    store: Vec<Packet>,
}

struct Bits<'transmission>(&'transmission [u8]);

impl Transmission {
    fn take(&mut self, n: usize) -> Bits {
        let slice = &self.bits[self.cursor..self.cursor + n];
        self.cursor += n;
        Bits(slice)
    }

    fn parse(&mut self) -> Packet {
        let version = u8::from(self.take(3));
        let type_id = u8::from(self.take(3));
        let op = match type_id {
            0 => Op::Sum(self.parse_op()),
            1 => Op::Mul(self.parse_op()),
            2 => Op::Min(self.parse_op()),
            3 => Op::Max(self.parse_op()),
            4 => Op::Literal(self.parse_literal()),
            5 => Op::Gt(self.parse_op()),
            6 => Op::Lt(self.parse_op()),
            7 => Op::Eq(self.parse_op()),
            _ => panic!("unknown type ID {}", type_id),
        };
        Packet { version, op }
    }

    fn parse_literal(&mut self) -> u64 {
        let mut bits = Vec::new();
        loop {
            let Bits(chunk) = self.take(5);
            bits.extend_from_slice(&chunk[1..]);
            if chunk[0] == 0 {
                break;
            }
        }
        u64::from(Bits(&bits))
    }

    fn parse_op(&mut self) -> Vec<usize> {
        let mut subs = Vec::new();
        let length_type_id = u8::from(self.take(1));
        match length_type_id {
            0 => {
                // If the length type ID is 0, then the next 15 bits are a number
                // that represents the total length in bits of the sub-packets
                // contained by this packet.
                let subpacket_bits = usize::from(self.take(15));
                let stop = self.cursor + subpacket_bits;
                while self.cursor != stop {
                    assert!(self.cursor < stop);
                    let p = self.parse();
                    subs.push(self.store(p));
                }
            }
            _ => {
                // If the length type ID is 1, then the next 11 bits are a number
                // that represents the number of sub-packets immediately contained
                // by this packet.
                let num_subpackets = u64::from(self.take(11));
                for _ in 0..num_subpackets {
                    let p = self.parse();
                    subs.push(self.store(p));
                }
            }
        }
        subs
    }

    fn store(&mut self, p: Packet) -> usize {
        let id = self.store.len();
        self.store.push(p);
        id
    }
}

impl<'this> Outermost {
    fn sum_versions(&self) -> u64 {
        self.packet.version as u64
            + self.store.iter().map(|p| p.version as u64).sum::<u64>()
    }

    fn val(&self) -> u64 {
        self.val_of(&self.packet)
    }

    fn val_of(&self, p: &Packet) -> u64 {
        use Op::*;
        match &p.op {
            Literal(v) => *v,
            Sum(ps) => self.val_iter(ps).sum(),
            Mul(ps) => self.val_iter(ps).product(),
            Min(ps) => self.val_iter(ps).min().unwrap(),
            Max(ps) => self.val_iter(ps).max().unwrap(),
            Gt(ps) => {
                let (lhs, rhs) = self.two_vals(ps);
                u64::from(lhs > rhs)
            }
            Lt(ps) => {
                let (lhs, rhs) = self.two_vals(ps);
                u64::from(lhs < rhs)
            }
            Eq(ps) => {
                let (lhs, rhs) = self.two_vals(ps);
                u64::from(lhs == rhs)
            }
        }
    }

    fn val_iter(
        &'this self,
        ps: &'this [usize],
    ) -> impl Iterator<Item = u64> + 'this {
        ps.iter().map(|p| self.val_of(&self.store[*p]))
    }

    fn two_vals(&self, ps: &[usize]) -> (u64, u64) {
        let lhs = self.val_of(&self.store[ps[0]]);
        let rhs = self.val_of(&self.store[ps[1]]);
        (lhs, rhs)
    }
}

impl From<Transmission> for Outermost {
    fn from(mut t: Transmission) -> Self {
        let p = t.parse();
        Outermost {
            store: t.store,
            packet: p,
        }
    }
}

impl<'transmission> From<Bits<'transmission>> for u64 {
    fn from(Bits(bits): Bits<'transmission>) -> Self {
        let mut n = 0u64;
        for b in bits {
            n <<= 1;
            n |= *b as u64;
        }
        n
    }
}

impl<'transmission> From<Bits<'transmission>> for usize {
    fn from(bits: Bits<'transmission>) -> Self {
        u64::from(bits) as usize
    }
}

impl<'transmission> From<Bits<'transmission>> for u8 {
    fn from(bits: Bits<'transmission>) -> Self {
        u64::from(bits) as u8
    }
}

const BITS: &[&[u8]] = &[
    &[0, 0, 0, 0],
    &[0, 0, 0, 1],
    &[0, 0, 1, 0],
    &[0, 0, 1, 1],
    &[0, 1, 0, 0],
    &[0, 1, 0, 1],
    &[0, 1, 1, 0],
    &[0, 1, 1, 1],
    &[1, 0, 0, 0],
    &[1, 0, 0, 1],
    &[1, 0, 1, 0],
    &[1, 0, 1, 1],
    &[1, 1, 0, 0],
    &[1, 1, 0, 1],
    &[1, 1, 1, 0],
    &[1, 1, 1, 1],
];

impl FromStr for Transmission {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = Vec::new();
        for c in s.trim().chars() {
            let d = c
                .to_digit(16)
                .ok_or_else(|| anyhow!("unknown digit {}", c))?
                as usize;
            bits.extend(BITS[d]);
        }
        Ok(Transmission {
            bits,
            ..Transmission::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn val(s: &str) -> u64 {
        let t: Transmission = s.parse().unwrap();
        let o = Outermost::from(t);
        o.val()
    }

    #[test]
    fn test_take() {
        let mut t: Transmission = "F0".parse().unwrap();
        assert!(matches!(t.take(4), Bits(&[1, 1, 1, 1])));
        assert!(matches!(t.take(4), Bits(&[0, 0, 0, 0])));
    }

    #[test]
    fn test_conversion() {
        assert_eq!(u64::from(Bits(&[1, 0, 1, 0, 1, 0])), 42);
    }

    #[test]
    fn test_parse_literal() {
        let mut t = Transmission {
            bits: vec![1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0],
            ..Transmission::default()
        };
        assert_eq!(t.parse_literal(), 2021);
    }

    #[test]
    fn test_val() {
        assert_eq!(val("C200B40A82"), 3);
        assert_eq!(val("04005AC33890"), 54);
        assert_eq!(val("880086C3E88112"), 7);
        assert_eq!(val("CE00C43D881120"), 9);
        assert_eq!(val("D8005AC2A8F0"), 1);
        assert_eq!(val("F600BC2D8F"), 0);
        assert_eq!(val("9C005AC2F8F0"), 0);
        assert_eq!(val("9C0141080250320F1802104A08"), 1);
    }
}
