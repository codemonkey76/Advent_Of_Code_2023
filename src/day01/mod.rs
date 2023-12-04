use crate::Output;

pub mod input;
mod part1;
mod part2;

pub type Input = Vec<&'static str>;

pub fn run_both() -> (Output, Output) {
    let input = input::read();
    (part1::solve(&input), part2::solve(&input))
}