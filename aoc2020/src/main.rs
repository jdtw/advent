mod days;

fn main() {
  let args: Vec<_> = std::env::args().collect();
  let day: usize = args[1].parse().unwrap();
  println!("Day {}", day);

  let days: Vec<fn()> = vec![
    || panic!("There is no day zero!"),
    days::day01::solution,
    days::day02::solution,
    days::day03::solution,
    days::day04::solution,
    days::day05::solution,
    days::day06::solution,
    days::day07::solution,
    days::day08::solution,
    days::day09::solution,
    days::day10::solution,
    // NEXT SOLUTION
  ];

  days[day]();
}
