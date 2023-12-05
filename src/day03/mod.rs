use crate::Output;

pub mod input;
mod part1;
mod part2;

pub type Input = Vec<Vec<char>>;

pub fn run_both() -> (Output, Output) {
    let input = input::read();
    (part1::solve(&input), part2::solve(&input))
}

#[derive(Debug)]
pub struct FoundNumber {
    pub number: String,
    pub position: Position
}
impl FoundNumber {
    pub fn new(number: String, col: usize, row: usize) -> Self {
        let position = Position {
            col, row
        };

        FoundNumber {
            number,
            position
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub col: usize,
    pub row: usize
}
impl Position {
    pub fn new(col: usize, row:usize) -> Self {
        Position { col, row}
    }
}