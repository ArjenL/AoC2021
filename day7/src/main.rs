use std::io;

fn distance1(src: i32, dst: i32) -> i32 { i32::abs(src - dst) }

// (1..n).sum() == n(n+1)/2. Clever compiler :-)
fn distance2(src: i32, dst: i32) -> i32 { (1..=distance1(src, dst)).sum() }

fn solutions(input: &[i32]) {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let fuel: i32 = (min..=max)
        .map(|pos| input.iter().map(|n| distance1(*n, pos)).sum())
        .min()
        .unwrap();
    println!("part 1: {}", fuel);

    let fuel: i32 = (min..=max)
        .map(|pos| input.iter().map(|n| distance2(*n, pos)).sum())
        .min()
        .unwrap();
    println!("part 2: {}", fuel);
}

fn main() -> Result<(), io::Error> {
    let input = include_str!("../../inputs/day7.txt");
    let input: Vec<_> = input.split(',').flat_map(|n| n.parse::<i32>()).collect();

    solutions(&input);

    Ok(())
}
