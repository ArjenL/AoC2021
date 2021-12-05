use regex::Regex;
use lazy_static::lazy_static;
use std::{io::{self, Read}, collections::HashMap};

fn solution(input: &str, include_part2: bool) -> usize {
    lazy_static!{
        static ref RE: Regex = Regex::new(r"^(?P<x1>[\d]+),(?P<y1>[\d]+) -> (?P<x2>[\d]+),(?P<y2>[\d]+)$").unwrap();
    }
    let mut coords: HashMap<(i32, i32), u32> = HashMap::new();

    for line in input.lines() {
        let caps = RE.captures(line).unwrap();
        let value = |name| caps.name(name).unwrap().as_str().parse::<i32>().unwrap();
        let x1 = value("x1");
        let y1 = value("y1");
        let x2 = value("x2");
        let y2 = value("y2");

        if y1 == y2 { // horizontal
            for x in i32::min(x1, x2)..=i32::max(x1, x2) {
                coords.entry((x, y1)).and_modify(|r| *r += 1).or_insert(0);
            }
        }
        else if x1 == x2 { // vertical
            for y in i32::min(y1, y2)..=i32::max(y1, y2) {
                coords.entry((x1, y)).and_modify(|r| *r += 1).or_insert(0);
            }
        }
        else { // diagonal
            if include_part2 {
                let dx = if x1 < x2 { 1 } else { -1 };
                let dy = if y1 < y2 { 1 } else { -1 };
                let mut coord = (x1, y1);

                for _ in i32::min(x1, x2)..=i32::max(x1, x2) {
                    coords.entry(coord).and_modify(|r| *r += 1).or_insert(0);
                    coord = (coord.0 + dx, coord.1 + dy);
                }
            }
        }
    }

    coords.iter().filter(|(_, &v)| v > 1).count()
}
fn main() -> Result<(), io::Error> {
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input)?;

    println!("Number of overlapping points (horizontal/vertial) > 1: {}", solution(&input, false));
    println!("Number of overlapping points (horizontal/vertial/diagonal) > 1: {}", solution(&input, true));

    Ok(())
}

