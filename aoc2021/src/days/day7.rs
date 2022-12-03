const INPUT: &str = "input/day7.txt";

pub fn solution() {
    let mut crabs: Vec<usize> = input::csv(INPUT);
    crabs.sort();
    let min = crabs[0];
    let max = crabs[crabs.len() - 1];
    let part1 = (min..=max)
        .map(|pos| part1_cost(&crabs, pos))
        .min()
        .unwrap();
    println!("Part1: {}", part1);
    let part2 = (min..=max)
        .map(|pos| part2_cost(&crabs, pos))
        .min()
        .unwrap();
    println!("Part2: {}", part2);
}

fn part1_cost(crabs: &[usize], pos: usize) -> usize {
    crabs.iter().map(|c| c.abs_diff(pos)).sum()
}

fn part2_cost(crabs: &[usize], pos: usize) -> usize {
    crabs
        .iter()
        .map(|c| (1..=c.abs_diff(pos)).sum::<usize>())
        .sum()
}
