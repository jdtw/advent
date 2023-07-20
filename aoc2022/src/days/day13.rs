use std::{cmp::Ordering, collections::VecDeque};

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

const INPUT: &str = "input/day13.txt";

#[derive(Parser)]
#[grammar_inline = r#"
number  =  { ASCII_DIGIT+ }
value   = _{ number | list }
list    =  { "[]" | "[" ~ value ~ ("," ~ value)* ~ "]" }
pair    =  { (list ~ NEWLINE ~ list ~ NEWLINE*) }
packets =  { pair+ }
"#]
struct PacketParser;

pub fn solution() {
    let raw = input::string(INPUT);
    let packets = PacketParser::parse(Rule::packets, &raw)
        .unwrap()
        .next()
        .unwrap();
    let mut pairs = Vec::new();
    for pair in packets.into_inner() {
        let mut pair = pair.into_inner();
        let left = pair.next().unwrap().into_inner();
        let right = pair.next().unwrap().into_inner();
        pairs.push(Pair(Packet::from(left), Packet::from(right)));
    }
    let mut part1 = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if pair.in_order() {
            part1 += i + 1;
        }
    }
    println!("Part1: {part1}");

    // Break the packets out of pairs to sort them.
    let mut packets = Vec::new();
    for Pair(lhs, rhs) in pairs {
        packets.push(lhs);
        packets.push(rhs);
    }
    // Add the special divider packets...
    let begin = Packet(vec![
        Token::Open,
        Token::Open,
        Token::Num(2),
        Token::Close,
        Token::Close,
    ]);
    let end = Packet(vec![
        Token::Open,
        Token::Open,
        Token::Num(6),
        Token::Close,
        Token::Close,
    ]);
    packets.push(begin.clone());
    packets.push(end.clone());
    packets.sort_unstable_by(compare_packets);
    let begin = packets.iter().position(|p| p == &begin).unwrap();
    let end = packets.iter().position(|p| p == &end).unwrap();
    let part2 = (begin + 1) * (end + 1);
    println!("Part2: {part2}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Open,
    Close,
    Num(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet(Vec<Token>);

impl From<Pairs<'_, Rule>> for Packet {
    fn from(value: Pairs<'_, Rule>) -> Self {
        let mut tokens = Vec::new();
        tokens.push(Token::Open);
        for v in value {
            match v.as_rule() {
                Rule::number => {
                    let n = v.as_str();
                    tokens.push(Token::Num(n.parse().unwrap()))
                }
                Rule::list => {
                    let Packet(mut inner_tokens) = Packet::from(v.into_inner());
                    tokens.append(&mut inner_tokens);
                }
                _ => unreachable!(),
            }
        }
        tokens.push(Token::Close);
        Packet(tokens)
    }
}

#[derive(Debug)]
struct Pair(Packet, Packet);

fn compare_packets(Packet(lhs): &Packet, Packet(rhs): &Packet) -> Ordering {
    use Token::*;

    let mut lhs = VecDeque::from(lhs.clone());
    let mut rhs = VecDeque::from(rhs.clone());
    loop {
        match (lhs.pop_front(), rhs.pop_front()) {
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            // Same element types...
            (Some(Open), Some(Open)) => (),
            (Some(Close), Some(Close)) => (),
            (Some(Num(a)), Some(Num(b))) => {
                if a > b {
                    return Ordering::Greater;
                }
                if a < b {
                    return Ordering::Less;
                }
            }
            // rhs runs out of elements first means not in order.
            (Some(Num(_)), Some(Close)) | (Some(Open), Some(Close)) => {
                return Ordering::Greater
            }
            // lhs runs out of elements is in order.
            (Some(Close), Some(Num(_))) | (Some(Close), Some(Open)) => {
                return Ordering::Less;
            }
            // Exactly one of the elements is a number. Lift it into a list.
            (Some(Num(n)), Some(Open)) => {
                lhs.push_front(Token::Close);
                lhs.push_front(Num(n));
            }
            (Some(Open), Some(Num(n))) => {
                rhs.push_front(Token::Close);
                rhs.push_front(Num(n));
            }
        }
    }
}

impl Pair {
    fn in_order(&self) -> bool {
        matches!(
            compare_packets(&self.0, &self.1),
            Ordering::Less | Ordering::Equal
        )
    }
}
