use crate::day01::Input;
use crate::Output;

pub fn solve(input: &Input) -> Output {
    input.iter().copied().map(get_digits).sum::<u32>().into()
}

fn get_digits(s: &str) -> u32 {
    let first = s.chars().find(|c| c.is_ascii_digit()).unwrap();
    let last = s.chars().rev().find(|c| c.is_ascii_digit()).unwrap();

    format!("{}{}", first, last).parse::<u32>().unwrap()
}