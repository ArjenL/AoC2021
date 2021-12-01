use std::io::{self, BufRead};

fn main() {
    let measurements: Vec<u32> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|v| v.parse::<u32>())
        .collect();

    let answer1 = measure(&measurements, 2);
    println!("Measurements larger than previous measurement: {}", answer1);

    let answer2 = measure(&measurements, 4);
    println!("Sums larger than previous sum: {}", answer2);
}

fn measure(input: &[u32], window: usize) -> usize {
    input
        .windows(window)
        .filter(|s| s[1..window].iter().sum::<u32>() > s[0..window - 1].iter().sum())
        .count()
}
