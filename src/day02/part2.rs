use crate::day02::Input;
use crate::Output;

pub fn solve(input: &Input) -> Output {
    input.iter().map(|_| 0).sum::<u32>().into()
}