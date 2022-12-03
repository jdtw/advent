const INPUT: &str = "input/day6.txt";

struct Fish(usize, u8);

pub fn solution() {
    let mut fish: Vec<Fish> =
        input::csv(INPUT).into_iter().map(|f| Fish(1, f)).collect();
    for _ in 0..80 {
        tick(&mut fish);
    }
    println!("Part1: {}", count(&fish));
    for _ in 80..256 {
        tick(&mut fish);
    }
    println!("Part2: {}", count(&fish));
}

fn tick(fish: &mut Vec<Fish>) {
    let mut new = 0;
    for Fish(n, f) in fish.iter_mut() {
        if *f == 0 {
            new += *n;
            *f = 6;
        } else {
            *f -= 1;
        }
    }
    fish.push(Fish(new, 8));
}

fn count(fish: &[Fish]) -> usize {
    fish.iter().map(|Fish(n, _)| n).sum()
}
