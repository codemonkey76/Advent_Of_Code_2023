use crate::day01::Input;
use crate::Output;

pub fn solve(input: &Input) -> Output {
    input.iter().copied().map(|_| 0).sum::<u32>().into()
}
