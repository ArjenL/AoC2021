# 🎄 Advent of Code 2021 🎄

My [Rust](https://www.rust-lang.org) solutions to the
[Advent of Code 2021](https://adventofcode.com/2021) puzzles.

## Running the puzzles

The repository is a [Cargo](https://doc.rust-lang.org/cargo) workspace with
a separate binary target for each day. Input files are in the `inputs`
directory, as `dayX.txt`, and possibly `dayX_sampleX.txt` if there are sample
inputs given.

```bash
$ cargo run --release day1 < inputs/day1.txt
```
