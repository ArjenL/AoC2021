use std::io::{self, BufRead};
use thiserror::Error;

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug, Error)]
enum CommandError {
    #[error("Invalid command")]
    InvalidCommand,
    #[error("Amount out of range")]
    Amount(#[from] std::num::ParseIntError),
}

impl std::str::FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd_str: Vec<&str> = s.split_ascii_whitespace().collect();
        let amount = cmd_str[1].parse::<u32>()?;

        let command = match cmd_str[0] {
            "forward" => Command::Forward(amount),
            "up" => Command::Up(amount),
            "down" => Command::Down(amount),
            _ => return Err(CommandError::InvalidCommand),
        };

        Ok(command)
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Position {
    horizontal: u32,
    depth: u32,
}

impl std::ops::Add<Command> for Position {
    type Output = Self;

    fn add(self, rhs: Command) -> Self::Output {
        match rhs {
            Command::Forward(amount) => Self {
                horizontal: self.horizontal + amount,
                ..self
            },
            Command::Down(amount) => Self {
                depth: self.depth + amount,
                ..self
            },
            Command::Up(amount) => Self {
                depth: self.depth - amount,
                ..self
            },
        }
    }
}

#[derive(Debug, Default)]
struct Submarine {
    position: Position,
}

impl Submarine {
    fn new() -> Self {
        Self {
            position: Default::default(),
        }
    }

    fn run(&mut self, commands: impl IntoIterator<Item = Command>) -> Position {
        for c in commands {
            self.position = self.position + c;
        }
        self.position
    }
}

fn main() {
    let commands: Vec<Command> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|v| v.parse::<Command>())
        .collect();

    let mut submarine = Submarine::new();
    let final_position = submarine.run(commands);

    println!(
        "part1: final horizontal position x final depth = {}",
        final_position.horizontal * final_position.depth
    );
}
