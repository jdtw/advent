use std::collections::HashSet;

const INPUT: &str = "input/day6.txt";

pub fn solution() {
    let input: Vec<char> = input::string(INPUT).chars().collect();
    let part1 = find_unique(&input, 4);
    let part2 = find_unique(&input, 14);
    println!("Part1: {}\nPart2: {}", part1, part2);
}

fn find_unique(input: &[char], size: usize) -> usize {
    size + input
        .windows(size)
        .map(|window| window.iter().collect::<HashSet<_>>().len())
        .take_while(|unique| unique != &size)
        .count()
}
