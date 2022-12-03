use std::collections::HashSet;

const INPUT: &str = "input/day3.txt";

pub fn solution() {
    let packs = input::string(INPUT);
    println!("Part1: {}\nPart2: {}", part1(&packs), part2(&packs));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for l in input.lines() {
        let (one, two) = l.split_at(l.len() / 2);
        let one = one.chars().collect::<HashSet<_>>();
        for c in two.chars() {
            if one.contains(&c) {
                sum += priority(c);
                break;
            }
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    for l in input.lines().collect::<Vec<_>>().chunks_exact(3) {
        let (one, two, three) = (l[0], l[1], l[2]);
        let one = one.chars().collect::<HashSet<_>>();
        let two = two.chars().collect::<HashSet<_>>();
        for c in three.chars() {
            if one.contains(&c) && two.contains(&c) {
                sum += priority(c);
                break;
            }
        }
    }
    sum
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("bad char"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_part1_part2() {
        let packs = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(part1(packs), 157);
        assert_eq!(part2(packs), 70);
    }
}
