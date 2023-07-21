use std::{collections::HashSet, str::FromStr};

use pest::Parser;
use pest_derive::Parser;
use util::pos::Pos;

const INPUT: &str = "input/day15.txt";

pub fn solution() {
    let positions: Vec<Position> = input::parse_lines(INPUT);
    let part1 = part1(&positions);
    println!("Part1: {part1}");
    let part2 = part2(&positions);
    println!("Part2: {part2}");
}

fn part1(positions: &[Position]) -> usize {
    const ROW: i64 = 2_000_000;
    let mut set = HashSet::new();
    for (start, end) in row_coverage(positions, ROW) {
        for i in start..=end {
            set.insert(i);
        }
    }
    for p in positions {
        if p.beacon.1 == ROW {
            set.remove(&p.beacon.0);
        }
    }
    set.len()
}

fn part2(positions: &[Position]) -> i64 {
    const ROW: i64 = 4_000_000;
    for y in 0..=ROW {
        let intervals: Vec<_> = row_coverage(positions, y)
            .into_iter()
            .filter(|(start, end)| *end >= 0 && *start <= ROW)
            .collect();
        if intervals.len() > 1 {
            assert_eq!(intervals.len(), 2);
            let (_, x1) = intervals[0];
            let (x2, _) = intervals[1];
            assert_eq!(x2 - x1, 2);
            return (x1 + 1) * ROW + y;
        }
    }
    unreachable!()
}

// Returns the row coverage in a list of non-overlapping intervals.
fn row_coverage(positions: &[Position], y: i64) -> Vec<(i64, i64)> {
    let mut intervals: Vec<(i64, i64)> =
        positions.iter().filter_map(|p| p.row_coverage(y)).collect();
    intervals.sort();
    let mut merged = Vec::new();
    let mut start = intervals[0].0;
    let mut end: i64 = intervals[0].1;
    for &(cur_start, cur_end) in &intervals[1..] {
        // If this interval is entirely within the interval
        // already being built, ignore it.
        if start <= cur_start && cur_end <= end {
            continue;
        }
        // If this is the end of the interval, add it to the
        // list and start a new one.
        if cur_start > end + 1 {
            merged.push((start, end));
            start = cur_start
        }
        end = cur_end
    }
    merged.push((start, end));
    merged
}

#[derive(Debug)]
struct Position {
    sensor: Pos,
    beacon: Pos,
}

impl Position {
    // Returns the x values covered by this sensor in the given row.
    fn row_coverage(&self, y: i64) -> Option<(i64, i64)> {
        let row_distance = self.sensor.1.abs_diff(y);
        let reach = self.sensor.distance(&self.beacon);
        if row_distance > reach {
            return None;
        }
        let delta = i64::try_from(reach - row_distance).unwrap();
        Some((self.sensor.0 - delta, self.sensor.0 + delta))
    }
}

#[derive(Parser)]
#[grammar_inline = r#"
num       = { "-"? ~ ASCII_DIGIT+ }
pos       = { "x=" ~ num ~ ", y=" ~ num }
positions = { "Sensor at " ~ pos ~ ": closest beacon is at " ~ pos }
"#]
struct PositionParser;

impl FromStr for Position {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions = PositionParser::parse(Rule::positions, s)
            .unwrap()
            .next()
            .unwrap()
            .into_inner();
        let mut sensor = positions.next().unwrap().into_inner();
        let sensor = Pos(
            sensor.next().unwrap().as_str().parse().unwrap(),
            sensor.next().unwrap().as_str().parse().unwrap(),
        );
        let mut beacon = positions.next().unwrap().into_inner();
        let beacon = Pos(
            beacon.next().unwrap().as_str().parse().unwrap(),
            beacon.next().unwrap().as_str().parse().unwrap(),
        );
        Ok(Position { sensor, beacon })
    }
}
