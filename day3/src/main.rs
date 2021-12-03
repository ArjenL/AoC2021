use std::io::{self, BufRead};

// Count the number of 0 and 1 bits in the given row and return a tuple of (#
// of 0-bits, # of 1-bits).
fn bit_count(bitstrings: &[Vec<u32>], row: usize) -> (u32, u32) {
    bitstrings
        .iter()
        .map(|bitstring| bitstring[row])
        .fold((0, 0), |(zero, one), c| {
            if c == 0 {
                (zero + 1, one)
            }
            else {
                (zero, one + 1)
            }
        })
}

fn bit_counts(bitstrings: &[Vec<u32>]) -> Vec<(u32, u32)> {
    (0..bitstrings[0].len())
        .map(|row| bit_count(&bitstrings, row))
        .collect()
}

fn part1(bitstrings: &Vec<Vec<u32>>) {
    let counts = bit_counts(&bitstrings);

    let most_zeroes = |c: &(u32, u32)| if c.0 > c.1 { 0 } else { 1 };
    let most_ones = |c: &(u32, u32)| if c.1 > c.0 { 0 } else { 1 };
    let to_int = |num: u32, b: u32| num * 2 + b;

    let gamma = counts.iter().map(most_zeroes).fold(0, to_int);
    let epsilon = counts.iter().map(most_ones).fold(0, to_int);

    println!("gamma x epsilon: {}", gamma * epsilon);
}

fn most_bits(bits: (u32, u32)) -> u32 {
    if bits.1 >= bits.0 {
        1
    }
    else {
        0
    }
}

fn least_bits(bits: (u32, u32)) -> u32 {
    if bits.1 >= bits.0 {
        0
    }
    else {
        1
    }
}

fn bitstring_to_num(bitstring: &Vec<u32>) -> u32 {
    bitstring.iter().fold(0, |num, b: &u32| num * 2 + b)
}

fn part2(bitstrings: &Vec<Vec<u32>>) {
    let mut possibles: Vec<_> = bitstrings.clone();
    for row in 0..bitstrings[0].len() {
        let f = most_bits(bit_count(&possibles, row));
        possibles.retain(|e| e[row] == f);
        if possibles.len() == 1 {
            break;
        }
    }
    let o2_generator_rating = bitstring_to_num(&possibles[0]);

    let mut possibles: Vec<_> = bitstrings.clone();
    for row in 0..bitstrings[0].len() {
        let f = least_bits(bit_count(&possibles, row));
        possibles.retain(|e| e[row] == f);
        if possibles.len() == 1 {
            break;
        }
    }
    let co2_scrubber_rating = bitstring_to_num(&possibles[0]);

    println!(
        "O2_generator_rating: {}, CO2_scrubber_rating: {}, life support rating: {}",
        o2_generator_rating,
        co2_scrubber_rating,
        o2_generator_rating * co2_scrubber_rating
    );
}

fn main() {
    let bitstrings: Vec<Vec<u32>> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|s| s.chars().map(|c| c.to_digit(2).unwrap()).collect())
        .collect();

    part1(&bitstrings);
    part2(&bitstrings);
}
