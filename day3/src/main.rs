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
    let mut gamma = 0;
    let mut epsilon = 0;

    for (zeroes, ones) in bit_counts(&bitstrings).iter() {
        let add_gamma = zeroes > ones;
        gamma = gamma << 1 | add_gamma as u32;
        epsilon = epsilon << 1 | !add_gamma as u32;
    }

    println!("gamma x epsilon: {}", gamma * epsilon);
}

fn filter_bit(bits: (u32, u32)) -> u32 {
    (bits.1 >= bits.0) as u32
}

fn bitstring_to_num(bitstring: &Vec<u32>) -> u32 {
    bitstring.iter().fold(0, |num, b: &u32| num * 2 + b)
}

fn part2(bitstrings: &Vec<Vec<u32>>) {
    let mut possibles: Vec<_> = bitstrings.clone();
    for row in 0..bitstrings[0].len() {
        let f = filter_bit(bit_count(&possibles, row));
        possibles.retain(|e| e[row] == f);
        if possibles.len() == 1 {
            break;
        }
    }
    let o2_generator_rating = bitstring_to_num(&possibles[0]);

    let mut possibles: Vec<_> = bitstrings.clone();
    for row in 0..bitstrings[0].len() {
        let f = filter_bit(bit_count(&possibles, row));
        possibles.retain(|e| e[row] != f);
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
