use std::collections::HashMap;
use crate::day02::{Cube, Input};
use crate::day02::input::Game;
use crate::Output;

pub fn solve(input: &Input) -> Output {
    input.iter().map(get_minimum_cubes).sum::<u32>().into()
}

pub fn get_minimum_cubes(game: &Game) -> u32{
    let mut results: HashMap<Cube, u32> = HashMap::new();

    for draw in &game.draws {
        let draw_results: HashMap<Cube, u32> = draw.clone();

        for (cube, count) in draw_results {
            results.entry(cube)
                .and_modify(|value| {
                    if *value < count {
                        *value = count;
                    }
                })
                .or_insert(count);
        }
    }
    results.values().product::<u32>()
}