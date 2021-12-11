use std::io;
use std::{collections::HashMap, io::Read};

fn count_easy_digits(display: &[u8]) -> usize {
    display
        .iter()
        .filter(|d| {
            d.count_ones() == 2 || d.count_ones() == 3 || d.count_ones() == 4 || d.count_ones() == 7
        })
        .count()
}

fn decode_digits(line: &[u8]) -> u32 {
    let mut os = line[0..10].to_vec();
    os.sort_by_key(|a| a.count_ones());
    let mut digits = [0u8; 10];

    for i in os[0..10].iter() {
        let digit = match i.count_ones() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 if (*i & digits[4]).count_ones() == 2 && (*i & digits[7]).count_ones() == 2 => 2,
            5 if (*i & digits[4]).count_ones() == 3 && (*i & digits[7]).count_ones() == 3 => 3,
            5 if (*i & digits[4]).count_ones() == 3 && (*i & digits[7]).count_ones() == 2 => 5,
            6 if (*i & digits[4]).count_ones() == 3 && (*i & digits[7]).count_ones() == 3 => 0,
            6 if (*i & digits[4]).count_ones() == 3 && (*i & digits[7]).count_ones() == 2 => 6,
            6 if (*i & digits[4]).count_ones() == 4 && (*i & digits[7]).count_ones() == 3 => 9,
            7 => 8,
            _ => unreachable!(),
        };
        digits[digit] = *i;
    }
    let digits: HashMap<u8, u8> = digits.iter().enumerate().map(|v| (*v.1, v.0 as u8)).collect();

    line[10..14].iter().fold(0, |v, d| v * 10 + digits[d] as u32)
}

fn parse_line(line: &str) -> Vec<u8> {
    line.split_whitespace()
        .filter(|s| *s != "|")
        .map(|s| s.as_bytes().iter().fold(0, |v, c| v | (1 << (6 - (*c - b'a')))))
        .collect::<Vec<u8>>()
}

fn main() -> Result<(), io::Error> {
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input)?;

    let lines: Vec<_> = input.lines().map(parse_line).collect();
    let part1: usize = lines.iter().map(|l| count_easy_digits(&l[10..14])).sum();
    let part2: u32 = lines.iter().map(|l| decode_digits(l)).sum();

    println!("part1: {}", part1);
    println!("part2: {}", part2);

    Ok(())
}
