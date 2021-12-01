use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let measurements: Vec<u32> = stdin
        .lines()
        .flatten()
        .flat_map(|v| v.parse::<u32>())
        .collect();

    let answer1 = measurements.windows(2).filter(|s| s[1] > s[0]).count();
    println!("Measurements larger than previous measurement: {}", answer1);

    let answer2 = measurements
        .windows(4)
        .filter(|s| s[1..4].iter().sum::<u32>() > s[0..3].iter().sum())
        .count();
    println!("Sums larger than previous sum: {}", answer2);
}
