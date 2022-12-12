mod days;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let day: usize = args[1].parse().unwrap();
    println!("Day {}", day);

    let days: Vec<fn()> = vec![
        || panic!("There is no day zero!"),
        days::day1::solution,
        days::day2::solution,
        days::day3::solution,
        days::day4::solution,
        days::day5::solution,
        days::day6::solution,
        days::day7::solution,
        days::day8::solution,
        days::day9::solution,
        days::day10::solution,
        // NEXT SOLUTION
    ];

    days[day]();
}
