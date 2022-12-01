mod days;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let day: usize = args[1].parse().unwrap();
    println!("Day {}", day);

    let days: Vec<fn()> = vec![
        || panic!("There is no day zero!"),
        days::day1::solution,
        // NEXT SOLUTION
    ];

    days[day]();
}
