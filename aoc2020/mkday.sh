#!/bin/bash
set -e

echo "pub mod day${1};" >> ./src/days.rs
cat << EOF > "./src/days/day${1}.rs"
const INPUT: &str = "input/day${1}.txt";

pub fn solution() {
  todo!()
}
EOF
sed -i '' -e "s#// NEXT SOLUTION#days::day${1}::solution,\n// NEXT SOLUTION#" "./src/main.rs"
cargo fmt
