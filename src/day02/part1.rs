use std::collections::HashMap;
use crate::day02::{Cube, Input};
use crate::day02::input::Game;
use crate::Output;

pub fn solve(input: &Input) -> Output {
    let rule: HashMap<Cube, u32> = [
        (Cube::Red, 12),
        (Cube::Green, 13),
        (Cube::Blue, 14)
    ]
        .iter()
        .cloned()
        .collect();

    input
        .iter()
        .filter(|game| is_game_possible(game, &rule))
        .map(|game| game.id)
        .sum::<u32>()
        .into()

}

fn is_game_possible(game: &Game, rule: &HashMap<Cube, u32>) -> bool {
    let mut possible = true;
    for (cube_type, count) in rule {
        for draw in &game.draws {
            if let Some(cubes_in_game) = draw.get(cube_type) {
                if cubes_in_game > &count {
                    possible = false;
                    break;
                }
            }
        }
    }
    possible
}

