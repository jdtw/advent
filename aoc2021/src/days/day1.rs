use input::parse_lines;

const INPUT: &str = "input/day1.txt";

pub fn solution() {
    let data: Vec<i64> = parse_lines(INPUT);
    let mut part1 = 0;
    let mut part2 = 0;
    let mut prev = i64::MAX;
    for i in 0..data.len() {
        // Part one...
        if i > 0 && data[i] > data[i - 1] {
            part1 += 1;
        }
        // Part two...
        if i > 1 {
            let sum = data[i] + data[i - 1] + data[i - 2];
            if sum > prev {
                part2 += 1
            }
            prev = sum
        }
    }
    println!("part1: {}\npart2: {}", part1, part2);
}
