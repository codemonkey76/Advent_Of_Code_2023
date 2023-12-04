use crate::day02::input::Game;
use crate::Output;

pub mod input;
mod part1;
mod part2;

// pub type Input = Vec<&'static str>;
pub type Input = Vec<Game>;

pub fn run_both() -> (Output, Output) {
    let input = input::read();
    (part1::solve(&input), part2::solve(&input))
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Cube {
    Blue,
    Red,
    Green
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        match value {
            "blue" => Cube::Blue,
            "red" => Cube::Red,
            "green" => Cube::Green,
            _ => {panic!("Invalid cube")}
        }
    }
}