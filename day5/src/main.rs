use regex::Regex;
use lazy_static::lazy_static;
use std::{io::{self, BufRead}, collections::HashMap};

fn main() -> Result<(), io::Error> {
    lazy_static!{
        static ref RE: Regex = Regex::new(r"^(?P<x1>[\d]+),(?P<y1>[\d]+) -> (?P<x2>[\d]+),(?P<y2>[\d]+)$").unwrap();
    }

    let mut coords: HashMap<(u32, u32), u32> = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let caps = RE.captures(&line).unwrap();
        let x1 = caps.name("x1").unwrap().as_str().parse::<u32>().unwrap();
        let y1 = caps.name("y1").unwrap().as_str().parse::<u32>().unwrap();
        let x2 = caps.name("x2").unwrap().as_str().parse::<u32>().unwrap();
        let y2 = caps.name("y2").unwrap().as_str().parse::<u32>().unwrap();
        if y1 == y2 {
            for x in u32::min(x1, x2)..=u32::max(x1, x2) {
                let r = coords.entry((x, y1)).or_insert(0);
                *r += 1
            }
        }
        if x1 == x2 {
            for y in u32::min(y1, y2)..=u32::max(y1, y2) {
                let r = coords.entry((x1, y)).or_insert(0);
                *r += 1
            }
        }
    }
    let part1 = coords.iter().filter(|(_, &v)| v > 1).count();
    println!("Number of overlapping points > 1: {}", part1);

    Ok(())
}

