use std::io::{
    BufRead, {self},
};
use thiserror::Error;

#[derive(Debug, Error)]
enum BingoError {
    #[error("Invalid game file")]
    Gamefile,
    #[error("File error")]
    File(#[from] io::Error),
    #[error("Board error")]
    Board,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum BingoNumber {
    Unmarked(u16),
    Marked(u16),
}

#[derive(Debug)]
struct BingoGame {
    numbers: Vec<u16>,
    index: usize,
    boards: Vec<BingoBoard>,
    winning_boards: Vec<usize>,
}

impl BingoGame {
    fn from_reader<R: BufRead>(mut input: R) -> Result<Self, BingoError> {
        let mut buffer = String::new();
        input.read_to_string(&mut buffer)?;
        let mut parts = buffer.split("\n\n");

        // First part is the list of bingo numbers separated by commas.
        let numbers: Vec<u16> = match parts.next() {
            Some(line) => line.split(',').map(|s| s.parse::<u16>().unwrap()).collect(),
            _ => return Err(BingoError::Gamefile),
        };

        // Rest of the parts are the board numbers separated by whitespace
        let mut boards: Vec<BingoBoard> = Vec::new();
        for p in parts {
            let board_numbers = p
                .split_ascii_whitespace()
                .flat_map(|n| n.parse::<u16>())
                .collect();
            boards.push(BingoBoard::from_vec(board_numbers)?);
        }

        Ok(Self {
            numbers,
            index: 0,
            boards,
            winning_boards: Vec::new(),
        })
    }

    fn run(&mut self) {
        for (index, number) in self.numbers.iter().enumerate() {
            let winners: Vec<_> = self
                .boards
                .iter_mut()
                .enumerate()
                .map(|(i, b)| (i, b.mark_number(*number)))
                .filter(|(_, b)| *b)
                .map(|(i, _)| i)
                .collect();
            for w in winners {
                self.winning_boards.push(w);
            }
            self.index = index;
        }
    }

    fn first_score(&self) -> Option<u32> {
        if self.winning_boards.len() > 0 {
            self.boards[self.winning_boards[0]].score
        } else {
            None
        }
    }

    fn last_score(&self) -> Option<u32> {
        if self.winning_boards.len() > 0 {
            self.boards[self.winning_boards[self.winning_boards.len() - 1]].score
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct BingoBoard {
    numbers: Vec<BingoNumber>,
    score: Option<u32>,
}

impl BingoBoard {
    fn from_vec(numbers: Vec<u16>) -> Result<Self, BingoError> {
        if numbers.len() != 25 {
            return Err(BingoError::Board);
        }
        let numbers = numbers.iter().map(|&n| BingoNumber::Unmarked(n)).collect();

        Ok(Self {
            numbers,
            score: None,
        })
    }

    fn mark_number(&mut self, number: u16) -> bool {
        if let Some(n) = self
            .numbers
            .iter_mut()
            .find(|n| **n == BingoNumber::Unmarked(number))
        {
            std::mem::replace(n, BingoNumber::Marked(number));
        }
        if matches!(self.score, Some(_)) {
            return false;
        }
        let has_bingo = self.has_bingo();
        if has_bingo {
            self.score = Some(self.score() * number as u32)
        }
        has_bingo
    }

    fn has_bingo(&self) -> bool {
        self.has_horizontal_bingo() || self.has_vertical_bingo()
    }

    fn has_horizontal_bingo(&self) -> bool {
        (0..5).any(|r| {
            self.numbers
                .iter()
                .skip(r * 5)
                .take(5)
                .all(|n| matches!(n, BingoNumber::Marked(_)))
        })
    }

    fn has_vertical_bingo(&self) -> bool {
        (0..5).any(|c| {
            self.numbers
                .iter()
                .skip(c)
                .step_by(5)
                .take(5)
                .all(|n| matches!(n, BingoNumber::Marked(_)))
        })
    }

    fn score(&self) -> u32 {
        self.numbers
            .iter()
            .filter_map(|n| {
                Some(match n {
                    BingoNumber::Unmarked(v) => *v as u32,
                    _ => 0,
                })
            })
            .sum()
    }
}
fn main() {
    let stdin = io::stdin();

    let mut game = BingoGame::from_reader(stdin.lock()).unwrap();
    game.run();
    let first_score = game.first_score().unwrap();
    let last_score = game.last_score().unwrap();

    println!("part1: {}, part2: {}", first_score, last_score);
}
